use yaml_rust::{Yaml, YamlLoader};


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Notification {
    pub email: Vec<String>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Specification {
    pub dockerfile: String,
    pub scripts: Vec<String>,
    pub after_success: Vec<String>,
    pub after_failure: Vec<String>,
    pub notification: Notification,
}


impl Specification {

    pub fn from_str(src: &str) -> Specification {
        let docs = YamlLoader::load_from_str(src);
        match docs {
            Ok(docs) => {
                if docs.len() != 1 {
                    let notification = Notification {
                        email: vec![],
                    };
                    return Specification {
                        dockerfile: "Dockerfile".to_owned(),
                        scripts: vec![],
                        after_success: vec![],
                        after_failure: vec![],
                        notification: notification,
                    };
                }
                let doc = &docs[0];
                let dockerfile = match &doc["dockerfile"] {
                    &Yaml::String(ref dockerfile) => dockerfile.to_owned(),
                    _ => "Dockerfile".to_owned(),
                };
                let scripts = match &doc["script"] {
                    &Yaml::String(ref script) => vec![script.to_owned()],
                    &Yaml::Array(ref script) => {
                        script.iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect()
                    }
                    _ => vec![],
                };
                let after_success = match &doc["after_success"] {
                    &Yaml::String(ref script) => vec![script.to_owned()],
                    &Yaml::Array(ref script) => {
                        script.iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect()
                    }
                    _ => vec![],
                };
                let after_failure = match &doc["after_failure"] {
                    &Yaml::String(ref script) => vec![script.to_owned()],
                    &Yaml::Array(ref script) => {
                        script.iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect()
                    }
                    _ => vec![],
                };
                let notification = match &doc["notification"] {
                    &Yaml::Hash(ref notif) => {
                        let key = Yaml::String("email".to_owned());
                        let email = match notif.get(&key) {
                            Some(email) => {
                                match email {
                                    &Yaml::String(ref mail) => vec![mail.to_owned()],
                                    &Yaml::Array(ref mail) => {
                                        mail.iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect()
                                    },
                                    _ => vec![],
                                }
                            },
                            None => vec![],
                        };
                        Notification {
                            email: email,
                        }
                    },
                    _ => {
                        Notification {
                            email: vec![],
                        }
                    }
                };
                Specification {
                    dockerfile: dockerfile,
                    scripts: scripts,
                    after_success: after_success,
                    after_failure: after_failure,
                    notification: notification,
                }
            },
            Err(..) => {
                let notification = Notification {
                    email: vec![],
                };
                Specification {
                    dockerfile: "Dockerfile".to_owned(),
                    scripts: vec![],
                    after_success: vec![],
                    after_failure: vec![],
                    notification: notification,
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Specification;

    #[test]
    fn test_empty_input() {
        let src = "";
        let spec = Specification::from_str(src);

        assert_eq!(&spec.dockerfile, "Dockerfile");
        assert_eq!(spec.scripts.len(), 0);
        assert_eq!(spec.after_success.len(), 0);
        assert_eq!(spec.after_failure.len(), 0);
        assert_eq!(spec.notification.email.len(), 0);
    }

    #[test]
    fn test_custom_dockerfile() {
        let src = "dockerfile: MyDockerfile";
        let spec = Specification::from_str(src);

        assert_eq!(&spec.dockerfile, "MyDockerfile");
    }

    #[test]
    fn test_single_script() {
        let src = "script: cargo build";
        let spec = Specification::from_str(src);

        assert_eq!(spec.scripts.len(), 1);
        assert_eq!(spec.scripts, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_single_script_list() {
        let src = "script:
          - cargo build
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.scripts.len(), 1);
        assert_eq!(spec.scripts, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_multi_script_list() {
        let src = "script:
          - cargo build
          - cargo run
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.scripts.len(), 2);
        assert_eq!(spec.scripts, vec!["cargo build".to_owned(), "cargo run".to_owned()]);
    }

    #[test]
    fn test_single_after_success() {
        let src = "after_success: cargo build";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_success.len(), 1);
        assert_eq!(spec.after_success, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_single_after_success_list() {
        let src = "after_success:
          - cargo build
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_success.len(), 1);
        assert_eq!(spec.after_success, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_multi_after_success_list() {
        let src = "after_success:
          - cargo build
          - cargo run
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_success.len(), 2);
        assert_eq!(spec.after_success, vec!["cargo build".to_owned(), "cargo run".to_owned()]);
    }

    #[test]
    fn test_single_after_failure() {
        let src = "after_failure: cargo build";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_failure.len(), 1);
        assert_eq!(spec.after_failure, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_single_after_failure_list() {
        let src = "after_failure:
          - cargo build
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_failure.len(), 1);
        assert_eq!(spec.after_failure, vec!["cargo build".to_owned()]);
    }

    #[test]
    fn test_multi_after_failure_list() {
        let src = "after_failure:
          - cargo build
          - cargo run
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.after_failure.len(), 2);
        assert_eq!(spec.after_failure, vec!["cargo build".to_owned(), "cargo run".to_owned()]);
    }

    #[test]
    fn test_single_notification_email() {
        let src = "notification:
          email: x@y.com
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.notification.email.len(), 1);
        assert_eq!(spec.notification.email, vec!["x@y.com".to_owned()]);
    }

    #[test]
    fn test_single_notification_email_list() {
        let src = "notification:
          email:
            - x@y.com
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.notification.email.len(), 1);
        assert_eq!(spec.notification.email, vec!["x@y.com".to_owned()]);
    }

    #[test]
    fn test_multi_notification_email_list() {
        let src = "notification:
          email:
            - x@y.com
            - y@x.com
        ";
        let spec = Specification::from_str(src);

        assert_eq!(spec.notification.email.len(), 2);
        assert_eq!(spec.notification.email, vec!["x@y.com".to_owned(), "y@x.com".to_owned()]);
    }
}
