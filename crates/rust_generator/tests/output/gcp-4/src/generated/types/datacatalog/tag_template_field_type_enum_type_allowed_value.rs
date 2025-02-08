#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TagTemplateFieldTypeEnumTypeAllowedValue {
    /// The display name for this template.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
}
