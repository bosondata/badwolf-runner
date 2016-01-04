extern crate badwolf_spec;

use std::env;
use std::process;

use badwolf_spec::Runner;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: badwolf-run <spec file>");
        process::exit(-1);
    }
    let spec_path = &args[1];
    let runner = Runner::from_file(&spec_path);
    runner.run();
}
