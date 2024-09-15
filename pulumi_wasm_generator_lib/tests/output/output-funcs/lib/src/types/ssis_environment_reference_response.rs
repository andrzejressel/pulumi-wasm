#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct SsisEnvironmentReferenceResponse {
    /// Environment folder name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "environmentFolderName")]
    pub r#environment_folder_name: Box<Option<String>>,
    /// Environment name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "environmentName")]
    pub r#environment_name: Box<Option<String>>,
    /// Environment reference id.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Reference type
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "referenceType")]
    pub r#reference_type: Box<Option<String>>,
}
