#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SsisPackageResponse {
    /// Metadata description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Folder id which contains package.
    #[builder(into, default)]
    #[serde(rename = "folderId")]
    pub r#folder_id: Box<Option<f64>>,
    /// Metadata id.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Metadata name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Parameters in package
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::types::SsisParameterResponse>>>,
    /// Project id which contains package.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<f64>>,
    /// Project version which contains package.
    #[builder(into, default)]
    #[serde(rename = "projectVersion")]
    pub r#project_version: Box<Option<f64>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Package'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type: Box<super::constants::ConstStringPackage>,
}
