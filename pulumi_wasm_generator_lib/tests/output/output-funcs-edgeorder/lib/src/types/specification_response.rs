//! Specifications of the configurations

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SpecificationResponse {
    /// Name of the specification
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value of the specification
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
