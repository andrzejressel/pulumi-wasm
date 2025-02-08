#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserPoolSchemaAttributeNumberAttributeConstraint {
    /// - Maximum allowed value.
    #[builder(into)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Box<String>,
    /// - Minimum allowed value.
    #[builder(into)]
    #[serde(rename = "minValue")]
    pub r#min_value: Box<String>,
}
