#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpec {
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    #[serde(rename = "configs")]
    pub r#configs: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecConfig>>>,
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Box<Option<crate::types::ServiceTaskSpecContainerSpecDnsConfig>>,
    #[serde(rename = "env")]
    pub r#env: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Box<Option<crate::types::ServiceTaskSpecContainerSpecHealthcheck>>,
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecHost>>>,
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecLabel>>>,
    #[serde(rename = "mounts")]
    pub r#mounts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecMount>>>,
    #[serde(rename = "privileges")]
    pub r#privileges: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivileges>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecSecret>>>,
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Box<Option<String>>,
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Box<Option<String>>,
    #[serde(rename = "sysctl")]
    pub r#sysctl: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
