#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DlpProfileContextAwarenessSkip {
    /// Return all matches, regardless of context analysis result, if the data is a file.
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}
