#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableExternalDataConfiguration {
    /// Let BigQuery try to autodetect the schema
    /// and format of the table.
    #[builder(into)]
    #[serde(rename = "autodetect")]
    pub r#autodetect: Box<bool>,
    /// Additional options if `source_format` is set to
    /// "AVRO".  Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "avroOptions")]
    pub r#avro_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationAvroOptions>>,
    /// Additional properties to set if
    /// `source_format` is set to "BIGTABLE". Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bigtableOptions")]
    pub r#bigtable_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationBigtableOptions>>,
    /// The compression type of the data source.
    /// Valid values are "NONE" or "GZIP".
    #[builder(into, default)]
    #[serde(rename = "compression")]
    pub r#compression: Box<Option<String>>,
    /// The connection specifying the credentials to be used to read
    /// external storage, such as Azure Blob, Cloud Storage, or S3. The `connection_id` can have
    /// the form `{{project}}.{{location}}.{{connection_id}}`
    /// or `projects/{{project}}/locations/{{location}}/connections/{{connection_id}}`.
    /// 
    /// ~>**NOTE:** If you set `external_data_configuration.connection_id`, the
    /// table schema must be specified using the top-level `schema` field
    /// documented above.
    #[builder(into, default)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<Option<String>>,
    /// Additional properties to set if
    /// `source_format` is set to "CSV". Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "csvOptions")]
    pub r#csv_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationCsvOptions>>,
    /// Specifies how source URIs are interpreted for constructing the file set to load.
    /// By default source URIs are expanded against the underlying storage.
    /// Other options include specifying manifest files. Only applicable to object storage systems. Docs
    #[builder(into, default)]
    #[serde(rename = "fileSetSpecType")]
    pub r#file_set_spec_type: Box<Option<String>>,
    /// Additional options if
    /// `source_format` is set to "GOOGLE_SHEETS". Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "googleSheetsOptions")]
    pub r#google_sheets_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationGoogleSheetsOptions>>,
    /// When set, configures hive partitioning
    /// support. Not all storage formats support hive partitioning -- requesting hive
    /// partitioning on an unsupported format will lead to an error, as will providing
    /// an invalid specification. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hivePartitioningOptions")]
    pub r#hive_partitioning_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationHivePartitioningOptions>>,
    /// Indicates if BigQuery should
    /// allow extra values that are not represented in the table schema.
    /// If true, the extra values are ignored. If false, records with
    /// extra columns are treated as bad records, and if there are too
    /// many bad records, an invalid error is returned in the job result.
    /// The default value is false.
    #[builder(into, default)]
    #[serde(rename = "ignoreUnknownValues")]
    pub r#ignore_unknown_values: Box<Option<bool>>,
    /// Used to indicate that a JSON variant, rather than normal JSON, is being used as the sourceFormat. This should only be used in combination with the `JSON` source format. Valid values are: `GEOJSON`.
    #[builder(into, default)]
    #[serde(rename = "jsonExtension")]
    pub r#json_extension: Box<Option<String>>,
    /// Additional properties to set if
    /// `source_format` is set to "JSON". Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "jsonOptions")]
    pub r#json_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationJsonOptions>>,
    /// The maximum number of bad records that
    /// BigQuery can ignore when reading data.
    #[builder(into, default)]
    #[serde(rename = "maxBadRecords")]
    pub r#max_bad_records: Box<Option<i32>>,
    /// Metadata Cache Mode for the table. Set this to enable caching of metadata from external data source. Valid values are `AUTOMATIC` and `MANUAL`.
    #[builder(into, default)]
    #[serde(rename = "metadataCacheMode")]
    pub r#metadata_cache_mode: Box<Option<String>>,
    /// Object Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If `object_metadata` is set, `source_format` should be omitted.
    #[builder(into, default)]
    #[serde(rename = "objectMetadata")]
    pub r#object_metadata: Box<Option<String>>,
    /// Additional properties to set if
    /// `source_format` is set to "PARQUET". Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "parquetOptions")]
    pub r#parquet_options: Box<Option<super::super::types::bigquery::TableExternalDataConfigurationParquetOptions>>,
    /// When creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC.
    #[builder(into, default)]
    #[serde(rename = "referenceFileSchemaUri")]
    pub r#reference_file_schema_uri: Box<Option<String>>,
    /// A JSON schema for the external table. Schema is required
    /// for CSV and JSON formats if autodetect is not on. Schema is disallowed
    /// for Google Cloud Bigtable, Cloud Datastore backups, Avro, Iceberg, ORC and Parquet formats.
    /// ~>**NOTE:** Because this field expects a JSON string, any changes to the
    /// string will create a diff, even if the JSON itself hasn't changed.
    /// Furthermore drift for this field cannot not be detected because BigQuery
    /// only uses this schema to compute the effective schema for the table, therefore
    /// any changes on the configured value will force the table to be recreated.
    /// This schema is effectively only applied when creating a table from an external
    /// datasource, after creation the computed schema will be stored in
    /// `google_bigquery_table.schema`
    /// 
    /// ~>**NOTE:** If you set `external_data_configuration.connection_id`, the
    /// table schema must be specified using the top-level `schema` field
    /// documented above.
    #[builder(into, default)]
    #[serde(rename = "schema")]
    pub r#schema: Box<Option<String>>,
    /// The data format. Please see sourceFormat under
    /// [ExternalDataConfiguration](https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration)
    /// in Bigquery's public API documentation for supported formats. To use "GOOGLE_SHEETS"
    /// the `scopes` must include "https://www.googleapis.com/auth/drive.readonly".
    #[builder(into, default)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Box<Option<String>>,
    /// A list of the fully-qualified URIs that point to
    /// your data in Google Cloud.
    #[builder(into)]
    #[serde(rename = "sourceUris")]
    pub r#source_uris: Box<Vec<String>>,
}
