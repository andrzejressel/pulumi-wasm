#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration {
    /// Enable speaker search.
    #[builder(into)]
    #[serde(rename = "speakerSearchStatus")]
    pub r#speaker_search_status: Box<String>,
    /// Enable voice tone analysis.
    #[builder(into)]
    #[serde(rename = "voiceToneAnalysisStatus")]
    pub r#voice_tone_analysis_status: Box<String>,
}
