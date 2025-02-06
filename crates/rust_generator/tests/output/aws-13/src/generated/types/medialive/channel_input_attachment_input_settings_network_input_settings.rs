#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsNetworkInputSettings {
    /// Specifies HLS input settings when the uri is for a HLS manifest. See HLS Input Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "hlsInputSettings")]
    pub r#hls_input_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings>>,
    /// Check HTTPS server certificates.
    #[builder(into, default)]
    #[serde(rename = "serverValidation")]
    pub r#server_validation: Box<Option<String>>,
}
