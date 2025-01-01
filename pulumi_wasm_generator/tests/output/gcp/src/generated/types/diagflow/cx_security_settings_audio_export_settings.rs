#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxSecuritySettingsAudioExportSettings {
    /// Filename pattern for exported audio.
    #[builder(into, default)]
    #[serde(rename = "audioExportPattern")]
    pub r#audio_export_pattern: Box<Option<String>>,
    /// File format for exported audio file. Currently only in telephony recordings.
    /// * MULAW: G.711 mu-law PCM with 8kHz sample rate.
    /// * MP3: MP3 file format.
    /// * OGG: OGG Vorbis.
    /// Possible values are: `MULAW`, `MP3`, `OGG`.
    #[builder(into, default)]
    #[serde(rename = "audioFormat")]
    pub r#audio_format: Box<Option<String>>,
    /// Enable audio redaction if it is true.
    #[builder(into, default)]
    #[serde(rename = "enableAudioRedaction")]
    pub r#enable_audio_redaction: Box<Option<bool>>,
    /// Cloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy.
    #[builder(into, default)]
    #[serde(rename = "gcsBucket")]
    pub r#gcs_bucket: Box<Option<String>>,
}
