#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationInputsSchemaRecordFormatMappingParameters {
    /// Mapping information when the record format uses delimiters.
    /// See CSV Mapping Parameters below for more details.
    #[builder(into, default)]
    #[serde(rename = "csv")]
    pub r#csv: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsSchemaRecordFormatMappingParametersCsv>>,
    /// Mapping information when JSON is the record format on the streaming source.
    /// See JSON Mapping Parameters below for more details.
    #[builder(into, default)]
    #[serde(rename = "json")]
    pub r#json: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsSchemaRecordFormatMappingParametersJson>>,
}