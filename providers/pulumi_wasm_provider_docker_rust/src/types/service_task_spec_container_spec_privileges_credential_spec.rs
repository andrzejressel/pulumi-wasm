#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from this file
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    /// Load credential spec from this value in the Windows registry
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}
