#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QueueHttpTargetHeaderOverride {
    /// Header embodying a key and a value.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<super::super::types::cloudtasks::QueueHttpTargetHeaderOverrideHeader>,
}
