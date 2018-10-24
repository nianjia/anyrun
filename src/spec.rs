use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use super::*;

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

#[derive(Serialize, Deserialize)]
#[derive(Default)]
struct User {
    uid: i32,
    gid: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_gids: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize)]
struct Process {
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal: Option<bool>,
    user: User,
    cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<String>>,
    args: Vec<String>,
}

impl Default for Process {
    fn default() -> Self {
        Self {
            terminal : Some(true),
            user : Default::default(),
            cwd : "/".to_string(),
            env : Some(vec!(
                String::from("PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"),
                String::from("TERM=xterm")
            )),
            args : vec!(String::from("sh")) 
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PlatformSpec {
    Linux(linux::Spec),
}

impl PlatformSpec {
    pub fn platform_default(platform: Platform) -> Self {
        match platform {
            _ => PlatformSpec::Linux(linux::Spec::default()),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Spec {
    #[serde(rename = "ociVersion")]
    oci_version: String,
    root: Root,
    process: Process,
    hostname: String,
    #[serde(flatten)]
    platform: Option<PlatformSpec>
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            oci_version : OCI_VERSION.to_string(),
            root: Default::default(),
            process: Default::default(),
            hostname: HOSTNAME.to_string(),
            platform: None
        }
    }
}

pub fn run(platform: Platform) -> i32 {
    let mut spec = Spec::default();
    spec.platform = Some(PlatformSpec::platform_default(platform));

    
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
    //println!("{}", serialized);
    0
}
