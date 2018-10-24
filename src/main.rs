#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::process;
use clap::{Arg, SubCommand, App};
mod spec;
mod platform;

use self::platform::*;
// use spec;

#[derive(Debug)]
pub enum Platform {
    Linux,
    Solaris,
    Windows,
    VM
}

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = &app.get_matches();

    let platform = match matches.is_present("vm")  {
        false => {
            if cfg!(target_os = "linux") {
                Platform::Linux
            } else if cfg!(windows) {
                Platform::Windows
            } else if cfg!(target_os = "solaris") {
                Platform::Solaris
            } else {
                eprintln!("Error: Not support on this platform!");
                process::exit(1);
            }
        }
        _ => Platform::VM
    };

    let ret = match matches.subcommand_name() {
        Some("spec") => spec::run(platform),
        _ => {
            1
        }
    };

    process::exit(ret)
}   
