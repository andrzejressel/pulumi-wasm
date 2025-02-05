#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelDestinationMediaPackageSetting {
    /// ID of the channel in MediaPackage that is the destination for this output group.
    #[builder(into)]
    #[serde(rename = "channelId")]
    pub r#channel_id: Box<String>,
}
