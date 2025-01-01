#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNotificationConfigPubsub {
    /// Whether or not the notification config is enabled
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Choose what type of notifications you want to receive. If no filters are applied, you'll receive all notification types. Structure is documented below.
    /// 
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::container::ClusterNotificationConfigPubsubFilter>>,
    /// The pubsub topic to push upgrade notifications to. Must be in the same project as the cluster. Must be in the format: `projects/{project}/topics/{topic}`.
    #[builder(into, default)]
    #[serde(rename = "topic")]
    pub r#topic: Box<Option<String>>,
}
