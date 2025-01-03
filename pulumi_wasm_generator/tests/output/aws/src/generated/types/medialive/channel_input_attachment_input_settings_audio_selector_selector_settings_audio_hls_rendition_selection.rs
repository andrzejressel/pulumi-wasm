#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection {
    /// Specifies the GROUP-ID in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: Box<String>,
    /// Specifies the NAME in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
