#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
