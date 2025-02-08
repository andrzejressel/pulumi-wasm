#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobLoad {
    /// Accept rows that are missing trailing optional columns. The missing values are treated as nulls.
    /// If false, records with missing trailing columns are treated as bad records, and if there are too many bad records,
    /// an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[builder(into, default)]
    #[serde(rename = "allowJaggedRows")]
    pub r#allow_jagged_rows: Box<Option<bool>>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.
    /// The default value is false.
    #[builder(into, default)]
    #[serde(rename = "allowQuotedNewlines")]
    pub r#allow_quoted_newlines: Box<Option<bool>>,
    /// Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    #[builder(into, default)]
    #[serde(rename = "autodetect")]
    pub r#autodetect: Box<Option<bool>>,
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into, default)]
    #[serde(rename = "createDisposition")]
    pub r#create_disposition: Box<Option<String>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinationEncryptionConfiguration")]
    pub r#destination_encryption_configuration: Box<Option<super::super::types::bigquery::JobLoadDestinationEncryptionConfiguration>>,
    /// The destination table to load the data into.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinationTable")]
    pub r#destination_table: Box<super::super::types::bigquery::JobLoadDestinationTable>,
    /// The character encoding of the data. The supported values are UTF-8 or ISO-8859-1.
    /// The default value is UTF-8. BigQuery decodes the data after the raw, binary data
    /// has been split using the values of the quote and fieldDelimiter properties.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.
    /// To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts
    /// the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the
    /// data in its raw, binary state. BigQuery also supports the escape sequence "\t" to specify a tab separator.
    /// The default value is a comma (',').
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// Indicates if BigQuery should allow extra values that are not represented in the table schema.
    /// If true, the extra values are ignored. If false, records with extra columns are treated as bad records,
    /// and if there are too many bad records, an invalid error is returned in the job result.
    /// The default value is false. The sourceFormat property determines what BigQuery treats as an extra value:
    /// CSV: Trailing columns
    /// JSON: Named values that don't match any column names
    #[builder(into, default)]
    #[serde(rename = "ignoreUnknownValues")]
    pub r#ignore_unknown_values: Box<Option<bool>>,
    /// If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.
    /// For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited
    /// GeoJSON: set to GEOJSON.
    #[builder(into, default)]
    #[serde(rename = "jsonExtension")]
    pub r#json_extension: Box<Option<String>>,
    /// The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,
    /// an invalid error is returned in the job result. The default value is 0, which requires that all records are valid.
    #[builder(into, default)]
    #[serde(rename = "maxBadRecords")]
    pub r#max_bad_records: Box<Option<i32>>,
    /// Specifies a string that represents a null value in a CSV file. The default value is the empty string. If you set this
    /// property to a custom value, BigQuery throws an error if an
    /// empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as
    /// an empty value.
    #[builder(into, default)]
    #[serde(rename = "nullMarker")]
    pub r#null_marker: Box<Option<String>>,
    /// Parquet Options for load and make external tables.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "parquetOptions")]
    pub r#parquet_options: Box<Option<super::super::types::bigquery::JobLoadParquetOptions>>,
    /// If sourceFormat is set to "DATASTORE_BACKUP", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.
    /// Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.
    /// If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[builder(into, default)]
    #[serde(rename = "projectionFields")]
    pub r#projection_fields: Box<Option<Vec<String>>>,
    /// The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,
    /// and then uses the first byte of the encoded string to split the data in its raw, binary state.
    /// The default value is a double-quote ('"'). If your data does not contain quoted sections, set the property value to an empty string.
    /// If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    #[builder(into, default)]
    #[serde(rename = "quote")]
    pub r#quote: Box<Option<String>>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or
    /// supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;
    /// when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.
    /// For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:
    /// ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.
    /// ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[builder(into, default)]
    #[serde(rename = "schemaUpdateOptions")]
    pub r#schema_update_options: Box<Option<Vec<String>>>,
    /// The number of rows at the top of a CSV file that BigQuery will skip when loading the data.
    /// The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    /// When autodetect is on, the behavior is the following:
    /// skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,
    /// the row is read as data. Otherwise data is read starting from the second row.
    /// skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.
    /// skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,
    /// row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[builder(into, default)]
    #[serde(rename = "skipLeadingRows")]
    pub r#skip_leading_rows: Box<Option<i32>>,
    /// The format of the data files. For CSV files, specify "CSV". For datastore backups, specify "DATASTORE_BACKUP".
    /// For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro, specify "AVRO". For parquet, specify "PARQUET".
    /// For orc, specify "ORC". [Beta] For Bigtable, specify "BIGTABLE".
    /// The default value is CSV.
    #[builder(into, default)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Box<Option<String>>,
    /// The fully-qualified URIs that point to your data in Google Cloud.
    /// For Google Cloud Storage URIs: Each URI can contain one '\*' wildcard character
    /// and it must come after the 'bucket' name. Size limits related to load jobs apply
    /// to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be
    /// specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.
    /// For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\*' wildcard character is not allowed.
    #[builder(into)]
    #[serde(rename = "sourceUris")]
    pub r#source_uris: Box<Vec<String>>,
    /// Time-based partitioning specification for the destination table.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timePartitioning")]
    pub r#time_partitioning: Box<Option<super::super::types::bigquery::JobLoadTimePartitioning>>,
    /// Specifies the action that occurs if the destination table already exists. The following values are supported:
    /// WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.
    /// WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.
    /// Each action is atomic and only occurs if BigQuery is able to complete the job successfully.
    /// Creation, truncation and append actions occur as one atomic update upon job completion.
    /// Default value is `WRITE_EMPTY`.
    /// Possible values are: `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
    #[builder(into, default)]
    #[serde(rename = "writeDisposition")]
    pub r#write_disposition: Box<Option<String>>,
}
