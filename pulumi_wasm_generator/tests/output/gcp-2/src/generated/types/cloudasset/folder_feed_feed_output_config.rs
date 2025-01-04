#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FolderFeedFeedOutputConfig {
    /// Destination on Cloud Pubsub.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubsubDestination")]
    pub r#pubsub_destination: Box<super::super::types::cloudasset::FolderFeedFeedOutputConfigPubsubDestination>,
}
