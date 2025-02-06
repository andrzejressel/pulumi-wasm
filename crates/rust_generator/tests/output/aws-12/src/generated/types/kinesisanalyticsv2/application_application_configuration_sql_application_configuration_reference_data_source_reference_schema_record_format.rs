#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormat {
    /// Provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.
    #[builder(into)]
    #[serde(rename = "mappingParameters")]
    pub r#mapping_parameters: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormatMappingParameters>,
    /// The type of record format. Valid values: `CSV`, `JSON`.
    #[builder(into)]
    #[serde(rename = "recordFormatType")]
    pub r#record_format_type: Box<String>,
}
