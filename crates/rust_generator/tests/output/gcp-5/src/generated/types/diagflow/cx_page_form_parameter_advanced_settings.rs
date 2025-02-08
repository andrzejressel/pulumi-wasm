#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxPageFormParameterAdvancedSettings {
    /// Define behaviors for DTMF (dual tone multi frequency). DTMF settings does not override each other. DTMF settings set at different levels define DTMF detections running in parallel. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// * Page level
    /// * Parameter level
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dtmfSettings")]
    pub r#dtmf_settings: Box<Option<super::super::types::diagflow::CxPageFormParameterAdvancedSettingsDtmfSettings>>,
}
