#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    /// SELinux level label
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// SELinux role label
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    /// SELinux type label
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// SELinux user label
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
