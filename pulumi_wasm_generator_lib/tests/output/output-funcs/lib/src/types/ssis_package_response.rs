//! Ssis Package.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SsisPackageResponse {
    /// Metadata description.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Folder id which contains package.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "folderId")]
    pub r#folder_id: Box<Option<f64>>,
    /// Metadata id.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Metadata name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Parameters in package
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<crate::types::SsisParameterResponse>>>,
    /// Project id which contains package.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<f64>>,
    /// Project version which contains package.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "projectVersion")]
    pub r#project_version: Box<Option<f64>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Package'.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<crate::__ConstString_Package>,
}
