#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OrganizationConformancePackInputParameter {
    /// The input key.
    #[builder(into)]
    #[serde(rename = "parameterName")]
    pub r#parameter_name: Box<String>,
    /// The input value.
    #[builder(into)]
    #[serde(rename = "parameterValue")]
    pub r#parameter_value: Box<String>,
}
