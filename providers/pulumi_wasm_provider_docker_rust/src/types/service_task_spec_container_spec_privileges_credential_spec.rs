#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}
