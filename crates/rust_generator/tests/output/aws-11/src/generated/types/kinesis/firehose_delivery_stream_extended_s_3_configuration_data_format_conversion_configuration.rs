#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration {
    /// Defaults to `true`. Set it to `false` if you want to disable format conversion while preserving the configuration details.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Specifies the deserializer that you want Kinesis Data Firehose to use to convert the format of your data from JSON. See `input_format_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "inputFormatConfiguration")]
    pub r#input_format_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfiguration>,
    /// Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data to the Parquet or ORC format. See `output_format_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "outputFormatConfiguration")]
    pub r#output_format_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfiguration>,
    /// Specifies the AWS Glue Data Catalog table that contains the column information. See `schema_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "schemaConfiguration")]
    pub r#schema_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationSchemaConfiguration>,
}
