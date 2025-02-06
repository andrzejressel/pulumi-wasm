#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobExtract {
    /// The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.
    /// The default value is NONE. DEFLATE and SNAPPY are only supported for Avro.
    #[builder(into, default)]
    #[serde(rename = "compression")]
    pub r#compression: Box<Option<String>>,
    /// The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.
    /// The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.
    /// The default value for models is SAVED_MODEL.
    #[builder(into, default)]
    #[serde(rename = "destinationFormat")]
    pub r#destination_format: Box<Option<String>>,
    /// A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[builder(into)]
    #[serde(rename = "destinationUris")]
    pub r#destination_uris: Box<Vec<String>>,
    /// When extracting data in CSV format, this defines the delimiter to use between fields in the exported data.
    /// Default is ','
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// Whether to print out a header row in the results. Default is true.
    #[builder(into, default)]
    #[serde(rename = "printHeader")]
    pub r#print_header: Box<Option<bool>>,
    /// A reference to the model being exported.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceModel")]
    pub r#source_model: Box<Option<super::super::types::bigquery::JobExtractSourceModel>>,
    /// A reference to the table being exported.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sourceTable")]
    pub r#source_table: Box<Option<super::super::types::bigquery::JobExtractSourceTable>>,
    /// Whether to use logical types when extracting to AVRO format.
    #[builder(into, default)]
    #[serde(rename = "useAvroLogicalTypes")]
    pub r#use_avro_logical_types: Box<Option<bool>>,
}
