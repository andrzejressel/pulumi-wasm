#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxFlowAdvancedSettings {
    /// If present, incoming audio is exported by Dialogflow to the configured Google Cloud Storage destination. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "audioExportGcsDestination")]
    pub r#audio_export_gcs_destination: Box<Option<super::super::types::diagflow::CxFlowAdvancedSettingsAudioExportGcsDestination>>,
    /// Define behaviors for DTMF (dual tone multi frequency). DTMF settings does not override each other. DTMF settings set at different levels define DTMF detections running in parallel. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// * Page level
    /// * Parameter level
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dtmfSettings")]
    pub r#dtmf_settings: Box<Option<super::super::types::diagflow::CxFlowAdvancedSettingsDtmfSettings>>,
    /// Settings for logging. Settings for Dialogflow History, Contact Center messages, StackDriver logs, and speech logging. Exposed at the following levels:
    /// * Agent level
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "loggingSettings")]
    pub r#logging_settings: Box<Option<super::super::types::diagflow::CxFlowAdvancedSettingsLoggingSettings>>,
    /// Settings for speech to text detection. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// * Page level
    /// * Parameter level
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "speechSettings")]
    pub r#speech_settings: Box<Option<super::super::types::diagflow::CxFlowAdvancedSettingsSpeechSettings>>,
}
