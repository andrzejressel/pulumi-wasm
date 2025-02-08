#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomLayerCloudwatchConfiguration {
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// A block the specifies how an opsworks logs look like. See Log Streams.
    #[builder(into, default)]
    #[serde(rename = "logStreams")]
    pub r#log_streams: Box<Option<Vec<super::super::types::opsworks::CustomLayerCloudwatchConfigurationLogStream>>>,
}
