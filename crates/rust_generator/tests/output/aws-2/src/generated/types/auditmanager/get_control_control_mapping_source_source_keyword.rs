#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetControlControlMappingSourceSourceKeyword {
    #[builder(into)]
    #[serde(rename = "keywordInputType")]
    pub r#keyword_input_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "keywordValue")]
    pub r#keyword_value: Box<String>,
}
