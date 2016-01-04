use std::io::prelude::*;
use std::fs::File;
use std::process::Command;

use spec::Specification;


#[derive(Debug, Clone)]
pub struct Runner {
    pub spec: Specification,
}

impl Runner {

    pub fn new(spec: Specification) -> Runner {
        Runner {
            spec: spec,
        }
    }

    pub fn from_file(path: &str) -> Runner {
        let mut f = File::open(path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();

        Runner::from_str(&buf)
    }

    pub fn from_str(src: &str) -> Runner {
        let spec = Specification::from_str(src);
        Runner::new(spec)
    }

    fn run_commands(&self, commands: &[String]) -> (bool, i32) {
        for command in commands {
            let status = Command::new("sh")
                                    .arg("-c")
                                    .arg(command)
                                    .status()
                                    .unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e);
            });
            if !status.success() {
                return (false, status.code().unwrap_or(-1i32));
            }
        }
        (true, 0i32)
    }

    pub fn run(&self) -> i32 {
        let (success, exit_code) = self.run_commands(&self.spec.scripts);
        if success {
            self.run_commands(&self.spec.after_success);
        } else {
            self.run_commands(&self.spec.after_failure);
        }
        exit_code
    }
}

#[cfg(test)]
mod tests {
    use super::Runner;

    #[test]
    fn test_run_success() {
        let spec = "
        script: pwd
        after_success: ls -lrth
        ";
        let runner = Runner::from_str(spec);
        let exit_code = runner.run();
        assert_eq!(exit_code, 0i32);
    }

    #[test]
    fn test_fn_failure() {
        let spec = "
        script: not-exists-command
        after_failure: ls -lrth
        ";
        let runner = Runner::from_str(spec);
        let exit_code = runner.run();
        assert_eq!(exit_code, 127i32);
    }
}
