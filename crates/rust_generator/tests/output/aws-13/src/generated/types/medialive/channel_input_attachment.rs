#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachment {
    /// User-specified settings for defining what the conditions are for declaring the input unhealthy and failing over to a different input. See Automatic Input Failover Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "automaticInputFailoverSettings")]
    pub r#automatic_input_failover_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettings>>,
    /// User-specified name for the attachment.
    #[builder(into)]
    #[serde(rename = "inputAttachmentName")]
    pub r#input_attachment_name: Box<String>,
    /// The ID of the input.
    #[builder(into)]
    #[serde(rename = "inputId")]
    pub r#input_id: Box<String>,
    /// Settings of an input. See Input Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "inputSettings")]
    pub r#input_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettings>>,
}
