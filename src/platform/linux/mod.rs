#[derive(Serialize, Deserialize)]
struct Device {
    allow: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    dev_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<String>
}

impl Default for Device {
    fn default() -> Self {
        Self {
            allow: false,
            dev_type: None,
            major: None,
            minor: None,
            access: Some("rwm".to_string())
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Resources {
    #[serde(rename = "devices")]
    device_whitelist: Vec<Device>
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            device_whitelist: vec!(Device::default()),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Namespace {
    #[serde(rename = "type")]
    ns_type: NSType,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum NSType {
    Pid,
    Network,
    Ipc,
    Uts,
    Mount,
    User,
    Cgroup
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    resource: Resources,
    namespaces: Vec<Namespace>
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            resource: Default::default(),
            namespaces: vec!(
                Namespace { ns_type: NSType::Pid, path: None },
                Namespace { ns_type: NSType::Network, path: None },
                Namespace { ns_type: NSType::Ipc, path: None },
                Namespace { ns_type: NSType::Uts, path: None }
            )
        }
    }
}
