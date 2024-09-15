#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct SsisProjectResponse {
    /// Metadata description.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Environment reference in project
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "environmentRefs")]
    pub r#environment_refs: Box<Option<Vec<crate::types::SsisEnvironmentReferenceResponse>>>,
    /// Folder id which contains project.
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
    /// Parameters in project
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<crate::types::SsisParameterResponse>>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Project'.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Project version.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<f64>>,
}
