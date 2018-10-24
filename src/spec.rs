use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};

const OCI_VERSION : &'static str = "0.1.0";
const ROOTFS : &'static str = "rootfs";

const HOSTNAME : &'static str = "nianjia-container";

#[derive(Serialize, Deserialize)]
struct Root {
    path: String,
    readonly: bool,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            path: ROOTFS.to_string(),
            readonly: false
        }
    }
}

#[allow(non_snake_case)] 
#[derive(Serialize, Deserialize)]
struct Spec {
    ociVersion: String,
    root: Root,
    hostname: String,
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            ociVersion : OCI_VERSION.to_string(),
            root: Default::default(),
            hostname: HOSTNAME.to_string(),
        }
    }
}

pub fn run() -> i32 {
    let spec = Spec::default();
    
    let serialized = serde_json::to_string_pretty(&spec).unwrap();

    let file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open("config.json");

    let ret = match file {
        Ok(mut f) => {
            f.write(serialized.as_bytes()).unwrap();
            0
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::AlreadyExists =>  
                    eprintln!("File config.json exists. Remove it first"),
                _ => eprintln!("{}", error)
            };
            -1
        }
    };
    ret
}
