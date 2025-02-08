#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExtensionsInstanceRuntimeData {
    /// The fatal error state for the extension instance
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "fatalError")]
    pub r#fatal_error: Box<Option<super::super::types::firebase::ExtensionsInstanceRuntimeDataFatalError>>,
    /// The processing state for the extension instance
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "processingState")]
    pub r#processing_state: Box<Option<super::super::types::firebase::ExtensionsInstanceRuntimeDataProcessingState>>,
    /// The time of the last state update.
    #[builder(into, default)]
    #[serde(rename = "stateUpdateTime")]
    pub r#state_update_time: Box<Option<String>>,
}
