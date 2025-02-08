#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LiteTopicPartitionConfigCapacity {
    /// Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16.
    #[builder(into)]
    #[serde(rename = "publishMibPerSec")]
    pub r#publish_mib_per_sec: Box<i32>,
    /// Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16.
    #[builder(into)]
    #[serde(rename = "subscribeMibPerSec")]
    pub r#subscribe_mib_per_sec: Box<i32>,
}
