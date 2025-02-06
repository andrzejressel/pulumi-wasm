#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FolderFeedFeedOutputConfigPubsubDestination {
    /// Destination on Cloud Pubsub topic.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
