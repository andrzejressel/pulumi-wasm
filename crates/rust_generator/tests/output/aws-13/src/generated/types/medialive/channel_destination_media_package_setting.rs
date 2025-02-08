#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelDestinationMediaPackageSetting {
    /// ID of the channel in MediaPackage that is the destination for this output group.
    #[builder(into)]
    #[serde(rename = "channelId")]
    pub r#channel_id: Box<String>,
}
