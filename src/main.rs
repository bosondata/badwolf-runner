extern crate badwolf_runner;

use std::env;
use std::path::Path;
use std::process;

use badwolf_runner::Runner;


fn main() {
    let args: Vec<String> = env::args().collect();
    let spec_path: String;
    if args.len() != 2 {
        spec_path = ".badwolf.yml".to_owned();
        let path = Path::new(&spec_path);
        if !path.exists() {
            println!("Usage: badwolf-run <spec path>");
            process::exit(-1);
        }
    } else {
        spec_path = args[1].to_owned();
    }
    let runner = Runner::from_file(&spec_path);
    runner.run();
}
