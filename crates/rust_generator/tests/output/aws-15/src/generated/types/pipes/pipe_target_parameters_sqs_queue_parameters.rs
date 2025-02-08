#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersSqsQueueParameters {
    /// This parameter applies only to FIFO (first-in-first-out) queues. The token used for deduplication of sent messages.
    #[builder(into, default)]
    #[serde(rename = "messageDeduplicationId")]
    pub r#message_deduplication_id: Box<Option<String>>,
    /// The FIFO message group ID to use as the target.
    #[builder(into, default)]
    #[serde(rename = "messageGroupId")]
    pub r#message_group_id: Box<Option<String>>,
}
