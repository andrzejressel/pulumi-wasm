#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolSchemaStringAttributeConstraints {
    /// Maximum length of an attribute value of the string type.
    #[builder(into, default)]
    #[serde(rename = "maxLength")]
    pub r#max_length: Box<Option<String>>,
    /// Minimum length of an attribute value of the string type.
    #[builder(into, default)]
    #[serde(rename = "minLength")]
    pub r#min_length: Box<Option<String>>,
}
