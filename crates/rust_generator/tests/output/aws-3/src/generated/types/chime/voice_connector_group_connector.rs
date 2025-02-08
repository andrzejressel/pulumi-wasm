#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VoiceConnectorGroupConnector {
    /// The priority associated with the Amazon Chime Voice Connector, with 1 being the highest priority. Higher priority Amazon Chime Voice Connectors are attempted first.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The Amazon Chime Voice Connector ID.
    #[builder(into)]
    #[serde(rename = "voiceConnectorId")]
    pub r#voice_connector_id: Box<String>,
}
