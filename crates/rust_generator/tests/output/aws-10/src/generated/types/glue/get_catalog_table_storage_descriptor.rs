#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTableStorageDescriptor {
    /// List of locations that point to the path where a Delta table is located
    #[builder(into)]
    #[serde(rename = "additionalLocations")]
    pub r#additional_locations: Box<Vec<String>>,
    /// List of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into)]
    #[serde(rename = "bucketColumns")]
    pub r#bucket_columns: Box<Vec<String>>,
    /// Configuration block for columns in the table. See `columns` below.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorColumn>>,
    /// Whether the data in the table is compressed.
    #[builder(into)]
    #[serde(rename = "compressed")]
    pub r#compressed: Box<bool>,
    /// Input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<String>,
    /// Physical location of the table. By default, this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Is if the table contains any dimension columns.
    #[builder(into)]
    #[serde(rename = "numberOfBuckets")]
    pub r#number_of_buckets: Box<i32>,
    /// Output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<String>,
    /// Map of initialization parameters for the SerDe, in key-value form.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<std::collections::HashMap<String, String>>,
    /// Object that references a schema stored in the AWS Glue Schema Registry. See `schema_reference` below.
    #[builder(into)]
    #[serde(rename = "schemaReferences")]
    pub r#schema_references: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSchemaReference>>,
    /// Configuration block for serialization and deserialization ("SerDe") information. See `ser_de_info` below.
    #[builder(into)]
    #[serde(rename = "serDeInfos")]
    pub r#ser_de_infos: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSerDeInfo>>,
    /// Configuration block with information about values that appear very frequently in a column (skewed values). See `skewed_info` below.
    #[builder(into)]
    #[serde(rename = "skewedInfos")]
    pub r#skewed_infos: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSkewedInfo>>,
    /// Configuration block for the sort order of each bucket in the table. See `sort_columns` below.
    #[builder(into)]
    #[serde(rename = "sortColumns")]
    pub r#sort_columns: Box<Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSortColumn>>,
    /// Whether the table data is stored in subdirectories.
    #[builder(into)]
    #[serde(rename = "storedAsSubDirectories")]
    pub r#stored_as_sub_directories: Box<bool>,
}
