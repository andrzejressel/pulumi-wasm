#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableExternalDataConfigurationBigtableOptions {
    /// A list of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable.  Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "columnFamilies")]
    pub r#column_families: Box<Option<Vec<super::super::types::bigquery::TableExternalDataConfigurationBigtableOptionsColumnFamily>>>,
    /// If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "ignoreUnspecifiedColumnFamilies")]
    pub r#ignore_unspecified_column_families: Box<Option<bool>>,
    /// If field is true, then each column family will be read as a single JSON column. Otherwise they are read as a repeated cell structure containing timestamp/value tuples. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "outputColumnFamiliesAsJson")]
    pub r#output_column_families_as_json: Box<Option<bool>>,
    /// If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "readRowkeyAsString")]
    pub r#read_rowkey_as_string: Box<Option<bool>>,
}
