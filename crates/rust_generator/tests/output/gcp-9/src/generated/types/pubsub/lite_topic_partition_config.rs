#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LiteTopicPartitionConfig {
    /// The capacity configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<super::super::types::pubsub::LiteTopicPartitionConfigCapacity>>,
    /// The number of partitions in the topic. Must be at least 1.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
}
