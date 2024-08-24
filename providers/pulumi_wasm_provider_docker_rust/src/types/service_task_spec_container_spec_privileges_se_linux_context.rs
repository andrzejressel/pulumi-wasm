#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    /// SELinux level label
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// SELinux role label
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    /// SELinux type label
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// SELinux user label
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
