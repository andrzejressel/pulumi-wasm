//! Ssis environment.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SsisEnvironmentResponse {
    /// Metadata description.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Folder id which contains environment.
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
    /// The type of SSIS object metadata.
    /// Expected value is 'Environment'.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<crate::__ConstString_Environment>,
    /// Variable in environment
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "variables")]
    pub r#variables: Box<Option<Vec<crate::types::SsisVariableResponse>>>,
}
