#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNotificationConfig {
    /// The pubsub config for the cluster's upgrade notifications.
    #[builder(into)]
    #[serde(rename = "pubsub")]
    pub r#pubsub: Box<super::super::types::container::ClusterNotificationConfigPubsub>,
}
