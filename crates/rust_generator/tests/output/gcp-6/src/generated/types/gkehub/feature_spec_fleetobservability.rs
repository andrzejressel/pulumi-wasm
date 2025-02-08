#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureSpecFleetobservability {
    /// Specified if fleet logging feature is enabled for the entire fleet. If UNSPECIFIED, fleet logging feature is disabled for the entire fleet.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Box<Option<super::super::types::gkehub::FeatureSpecFleetobservabilityLoggingConfig>>,
}
