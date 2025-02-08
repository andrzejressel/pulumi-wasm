#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FormTypeModel {
    /// Smithy document that indicates the model of the API. Must be between the lengths 1 and 100,000 and be encoded as a smithy document.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "smithy")]
    pub r#smithy: Box<String>,
}
