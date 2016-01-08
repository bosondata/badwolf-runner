extern crate clap;
extern crate badwolf_runner;

use std::env;
use std::path::Path;
use std::process;

use clap::{Arg, App};
use badwolf_runner::Runner;


fn main() {
    let mut app = App::new("badwolf-run")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Messense Lv <messense@icloud.com")
                      .about("Badwolf test runner")
                      .arg(Arg::with_name("SPEC")
                            .help("Badwolf specification path")
                            .required(false)
                            .index(1));
    let matches = app.get_matches_from_safe_borrow(env::args())
                     .unwrap_or_else(|_| { process::exit(128) });
    let spec_path = matches.value_of("SPEC")
                           .unwrap_or(".badwolf.yml");
    let path = Path::new(spec_path);
    if !path.exists() {
        let _ = app.print_help();
        process::exit(128);
    }
    let runner = Runner::from_file(spec_path);
    process::exit(runner.run());
}
