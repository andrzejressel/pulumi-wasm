#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from this file
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    /// Load credential spec from this value in the Windows registry
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}
