//! Ssis variable.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SsisVariableResponse {
    /// Variable type.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "dataType")]
    pub r#data_type: Box<Option<String>>,
    /// Variable description.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Variable id.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Variable name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Whether variable is sensitive.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sensitive")]
    pub r#sensitive: Box<Option<bool>>,
    /// Variable sensitive value.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sensitiveValue")]
    pub r#sensitive_value: Box<Option<String>>,
    /// Variable value.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
