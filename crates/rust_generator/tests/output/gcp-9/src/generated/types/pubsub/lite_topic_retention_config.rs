#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LiteTopicRetentionConfig {
    /// The provisioned storage, in bytes, per partition. If the number of bytes stored
    /// in any of the topic's partitions grows beyond this value, older messages will be
    /// dropped to make room for newer ones, regardless of the value of period.
    #[builder(into)]
    #[serde(rename = "perPartitionBytes")]
    pub r#per_partition_bytes: Box<String>,
    /// How long a published message is retained. If unset, messages will be retained as
    /// long as the bytes retained for each partition is below perPartitionBytes. A
    /// duration in seconds with up to nine fractional digits, terminated by 's'.
    /// Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "period")]
    pub r#period: Box<Option<String>>,
}
