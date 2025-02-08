#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfiguration {
    /// Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. See `deserializer` block below for details.
    #[builder(into)]
    #[serde(rename = "deserializer")]
    pub r#deserializer: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfigurationDeserializer>,
}
