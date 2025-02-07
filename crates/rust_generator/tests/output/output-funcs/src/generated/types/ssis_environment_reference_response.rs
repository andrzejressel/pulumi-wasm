#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SsisEnvironmentReferenceResponse {
    /// Environment folder name.
    #[builder(into, default)]
    #[serde(rename = "environmentFolderName")]
    pub r#environment_folder_name: Box<Option<String>>,
    /// Environment name.
    #[builder(into, default)]
    #[serde(rename = "environmentName")]
    pub r#environment_name: Box<Option<String>>,
    /// Environment reference id.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Reference type
    #[builder(into, default)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: Box<Option<String>>,
}
