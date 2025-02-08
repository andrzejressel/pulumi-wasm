#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CatalogTableStorageDescriptor {
    /// List of locations that point to the path where a Delta table is located.
    #[builder(into, default)]
    #[serde(rename = "additionalLocations")]
    pub r#additional_locations: Box<Option<Vec<String>>>,
    /// List of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into, default)]
    #[serde(rename = "bucketColumns")]
    pub r#bucket_columns: Box<Option<Vec<String>>>,
    /// Configuration block for columns in the table. See `columns` below.
    #[builder(into, default)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Option<Vec<super::super::types::glue::CatalogTableStorageDescriptorColumn>>>,
    /// Whether the data in the table is compressed.
    #[builder(into, default)]
    #[serde(rename = "compressed")]
    pub r#compressed: Box<Option<bool>>,
    /// Input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into, default)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<Option<String>>,
    /// Physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Must be specified if the table contains any dimension columns.
    #[builder(into, default)]
    #[serde(rename = "numberOfBuckets")]
    pub r#number_of_buckets: Box<Option<i32>>,
    /// Output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into, default)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<Option<String>>,
    /// User-supplied properties in key-value form.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Object that references a schema stored in the AWS Glue Schema Registry. When creating a table, you can pass an empty list of columns for the schema, and instead use a schema reference. See Schema Reference below.
    #[builder(into, default)]
    #[serde(rename = "schemaReference")]
    pub r#schema_reference: Box<Option<super::super::types::glue::CatalogTableStorageDescriptorSchemaReference>>,
    /// Configuration block for serialization and deserialization ("SerDe") information. See `ser_de_info` below.
    #[builder(into, default)]
    #[serde(rename = "serDeInfo")]
    pub r#ser_de_info: Box<Option<super::super::types::glue::CatalogTableStorageDescriptorSerDeInfo>>,
    /// Configuration block with information about values that appear very frequently in a column (skewed values). See `skewed_info` below.
    #[builder(into, default)]
    #[serde(rename = "skewedInfo")]
    pub r#skewed_info: Box<Option<super::super::types::glue::CatalogTableStorageDescriptorSkewedInfo>>,
    /// Configuration block for the sort order of each bucket in the table. See `sort_columns` below.
    #[builder(into, default)]
    #[serde(rename = "sortColumns")]
    pub r#sort_columns: Box<Option<Vec<super::super::types::glue::CatalogTableStorageDescriptorSortColumn>>>,
    /// Whether the table data is stored in subdirectories.
    #[builder(into, default)]
    #[serde(rename = "storedAsSubDirectories")]
    pub r#stored_as_sub_directories: Box<Option<bool>>,
}
