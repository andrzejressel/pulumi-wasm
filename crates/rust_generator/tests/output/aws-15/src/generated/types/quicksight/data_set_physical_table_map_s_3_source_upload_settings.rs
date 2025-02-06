#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetPhysicalTableMapS3SourceUploadSettings {
    /// Whether the file has a header row, or the files each have a header row.
    #[builder(into, default)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: Box<Option<bool>>,
    /// Delimiter between values in the file.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// File format. Valid values are `CSV`, `TSV`, `CLF`, `ELF`, `XLSX`, and `JSON`.
    #[builder(into, default)]
    #[serde(rename = "format")]
    pub r#format: Box<Option<String>>,
    /// A row number to start reading data from.
    #[builder(into, default)]
    #[serde(rename = "startFromRow")]
    pub r#start_from_row: Box<Option<i32>>,
    /// Text qualifier. Valid values are `DOUBLE_QUOTE` and `SINGLE_QUOTE`.
    #[builder(into, default)]
    #[serde(rename = "textQualifier")]
    pub r#text_qualifier: Box<Option<String>>,
}
