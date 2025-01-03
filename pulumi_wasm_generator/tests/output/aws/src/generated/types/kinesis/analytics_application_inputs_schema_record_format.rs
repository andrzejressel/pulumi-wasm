#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationInputsSchemaRecordFormat {
    /// The Mapping Information for the record format.
    /// See Mapping Parameters below for more details.
    #[builder(into, default)]
    #[serde(rename = "mappingParameters")]
    pub r#mapping_parameters: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsSchemaRecordFormatMappingParameters>>,
    /// The type of Record Format. Can be `CSV` or `JSON`.
    #[builder(into, default)]
    #[serde(rename = "recordFormatType")]
    pub r#record_format_type: Box<Option<String>>,
}
