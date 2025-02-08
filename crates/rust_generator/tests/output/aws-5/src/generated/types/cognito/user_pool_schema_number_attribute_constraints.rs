#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolSchemaNumberAttributeConstraints {
    /// Maximum value of an attribute that is of the number data type.
    #[builder(into, default)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Box<Option<String>>,
    /// Minimum value of an attribute that is of the number data type.
    #[builder(into, default)]
    #[serde(rename = "minValue")]
    pub r#min_value: Box<Option<String>>,
}
