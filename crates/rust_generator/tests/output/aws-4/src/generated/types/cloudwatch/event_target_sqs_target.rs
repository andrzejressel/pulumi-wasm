#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetSqsTarget {
    /// The FIFO message group ID to use as the target.
    #[builder(into, default)]
    #[serde(rename = "messageGroupId")]
    pub r#message_group_id: Box<Option<String>>,
}
