#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VaultNotification {
    /// You can configure a vault to publish a notification for `ArchiveRetrievalCompleted` and `InventoryRetrievalCompleted` events.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Box<Vec<String>>,
    /// The SNS Topic ARN.
    #[builder(into)]
    #[serde(rename = "snsTopic")]
    pub r#sns_topic: Box<String>,
}
