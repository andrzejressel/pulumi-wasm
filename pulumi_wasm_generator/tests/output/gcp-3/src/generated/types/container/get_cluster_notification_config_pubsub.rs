#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNotificationConfigPubsub {
    /// Whether or not the notification config is enabled
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Allows filtering to one or more specific event types. If event types are present, those and only those event types will be transmitted to the cluster. Other types will be skipped. If no filter is specified, or no event types are present, all event types will be sent
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Vec<super::super::types::container::GetClusterNotificationConfigPubsubFilter>>,
    /// The pubsub topic to push upgrade notifications to. Must be in the same project as the cluster. Must be in the format: projects/{project}/topics/{topic}.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
