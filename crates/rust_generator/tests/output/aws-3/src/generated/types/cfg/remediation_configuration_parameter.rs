#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RemediationConfigurationParameter {
    /// Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value is dynamic and changes at run-time.
    #[builder(into, default)]
    #[serde(rename = "resourceValue")]
    pub r#resource_value: Box<Option<String>>,
    /// Value is static and does not change at run-time.
    #[builder(into, default)]
    #[serde(rename = "staticValue")]
    pub r#static_value: Box<Option<String>>,
    /// List of static values.
    #[builder(into, default)]
    #[serde(rename = "staticValues")]
    pub r#static_values: Box<Option<Vec<String>>>,
}
