#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PartitionStorageDescriptor {
    /// A list of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into, default)]
    #[serde(rename = "bucketColumns")]
    pub r#bucket_columns: Box<Option<Vec<String>>>,
    /// A list of the Columns in the table.
    #[builder(into, default)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Option<Vec<super::super::types::glue::PartitionStorageDescriptorColumn>>>,
    /// True if the data in the table is compressed, or False if not.
    #[builder(into, default)]
    #[serde(rename = "compressed")]
    pub r#compressed: Box<Option<bool>>,
    /// The input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into, default)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<Option<String>>,
    /// The physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Must be specified if the table contains any dimension columns.
    #[builder(into, default)]
    #[serde(rename = "numberOfBuckets")]
    pub r#number_of_buckets: Box<Option<i32>>,
    /// The output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into, default)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<Option<String>>,
    /// User-supplied properties in key-value form.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Serialization/deserialization (SerDe) information.
    #[builder(into, default)]
    #[serde(rename = "serDeInfo")]
    pub r#ser_de_info: Box<Option<super::super::types::glue::PartitionStorageDescriptorSerDeInfo>>,
    /// Information about values that appear very frequently in a column (skewed values).
    #[builder(into, default)]
    #[serde(rename = "skewedInfo")]
    pub r#skewed_info: Box<Option<super::super::types::glue::PartitionStorageDescriptorSkewedInfo>>,
    /// A list of Order objects specifying the sort order of each bucket in the table.
    #[builder(into, default)]
    #[serde(rename = "sortColumns")]
    pub r#sort_columns: Box<Option<Vec<super::super::types::glue::PartitionStorageDescriptorSortColumn>>>,
    /// True if the table data is stored in subdirectories, or False if not.
    #[builder(into, default)]
    #[serde(rename = "storedAsSubDirectories")]
    pub r#stored_as_sub_directories: Box<Option<bool>>,
}
