#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNotificationConfig {
    /// Notification config for Cloud Pub/Sub
    #[builder(into)]
    #[serde(rename = "pubsubs")]
    pub r#pubsubs: Box<Vec<super::super::types::container::GetClusterNotificationConfigPubsub>>,
}
