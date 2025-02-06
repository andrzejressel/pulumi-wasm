#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationProcessingConfiguration {
    /// Enables or disables data processing.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Specifies the data processors as multiple blocks. See `processors` block below for details.
    #[builder(into, default)]
    #[serde(rename = "processors")]
    pub r#processors: Box<Option<Vec<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationProcessingConfigurationProcessor>>>,
}
