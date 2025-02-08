#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxAgentAdvancedSettingsAudioExportGcsDestination {
    /// The Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.
    /// Format: gs://bucket/object-name-or-prefix
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
