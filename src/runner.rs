use std::io::prelude::*;
use std::fs::File;
use std::process::{self, Command};

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

        let spec = Specification::from_str(&buf);
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

    pub fn run(&self) {
        let (success, exit_code) = self.run_commands(&self.spec.scripts);
        if success {
            self.run_commands(&self.spec.after_success);
        } else {
            self.run_commands(&self.spec.after_failure);
            process::exit(exit_code);
        }
    }
}
