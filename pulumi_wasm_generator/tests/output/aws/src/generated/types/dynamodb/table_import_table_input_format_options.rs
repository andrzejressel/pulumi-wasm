#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableImportTableInputFormatOptions {
    /// This block contains the processing options for the CSV file being imported:
    #[builder(into, default)]
    #[serde(rename = "csv")]
    pub r#csv: Box<Option<super::super::types::dynamodb::TableImportTableInputFormatOptionsCsv>>,
}