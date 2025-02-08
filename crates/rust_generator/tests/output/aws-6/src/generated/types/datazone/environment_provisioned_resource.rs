#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentProvisionedResource {
    /// The name of the environment.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The value of an environment profile parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
