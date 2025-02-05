#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration {
    /// Labels all personally identifiable information (PII) identified in Transcript events.
    #[builder(into, default)]
    #[serde(rename = "contentIdentificationType")]
    pub r#content_identification_type: Box<Option<String>>,
    /// Redacts all personally identifiable information (PII) identified in Transcript events.
    #[builder(into, default)]
    #[serde(rename = "contentRedactionType")]
    pub r#content_redaction_type: Box<Option<String>>,
    /// Enables partial result stabilization in Transcript events.
    #[builder(into, default)]
    #[serde(rename = "enablePartialResultsStabilization")]
    pub r#enable_partial_results_stabilization: Box<Option<bool>>,
    /// Filters partial Utterance events from delivery to the insights target.
    #[builder(into, default)]
    #[serde(rename = "filterPartialResults")]
    pub r#filter_partial_results: Box<Option<bool>>,
    /// Language code for the transcription model.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<String>,
    /// Name of custom language model for transcription.
    #[builder(into, default)]
    #[serde(rename = "languageModelName")]
    pub r#language_model_name: Box<Option<String>>,
    /// Level of stability to use when partial results stabilization is enabled.
    #[builder(into, default)]
    #[serde(rename = "partialResultsStability")]
    pub r#partial_results_stability: Box<Option<String>>,
    /// Types of personally identifiable information (PII) to redact from a Transcript event.
    #[builder(into, default)]
    #[serde(rename = "piiEntityTypes")]
    pub r#pii_entity_types: Box<Option<String>>,
    /// Enables speaker partitioning (diarization) in your Transcript events.
    #[builder(into, default)]
    #[serde(rename = "showSpeakerLabel")]
    pub r#show_speaker_label: Box<Option<bool>>,
    /// Method for applying a vocabulary filter to Transcript events.
    #[builder(into, default)]
    #[serde(rename = "vocabularyFilterMethod")]
    pub r#vocabulary_filter_method: Box<Option<String>>,
    /// Name of the custom vocabulary filter to use when processing Transcript events.
    #[builder(into, default)]
    #[serde(rename = "vocabularyFilterName")]
    pub r#vocabulary_filter_name: Box<Option<String>>,
    /// Name of the custom vocabulary to use when processing Transcript events.
    #[builder(into, default)]
    #[serde(rename = "vocabularyName")]
    pub r#vocabulary_name: Box<Option<String>>,
}
