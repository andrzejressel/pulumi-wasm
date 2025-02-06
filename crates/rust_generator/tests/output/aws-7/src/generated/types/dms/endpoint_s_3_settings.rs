#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointS3Settings {
    /// Whether to add column name information to the .csv output file. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "addColumnName")]
    pub r#add_column_name: Box<Option<bool>>,
    /// S3 object prefix.
    #[builder(into, default)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: Box<Option<String>>,
    /// S3 bucket name.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// Predefined (canned) access control list for objects created in an S3 bucket. Valid values include `none`, `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Default is `none`.
    #[builder(into, default)]
    #[serde(rename = "cannedAclForObjects")]
    pub r#canned_acl_for_objects: Box<Option<String>>,
    /// Whether to write insert and update operations to .csv or .parquet output files. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "cdcInsertsAndUpdates")]
    pub r#cdc_inserts_and_updates: Box<Option<bool>>,
    /// Whether to write insert operations to .csv or .parquet output files. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "cdcInsertsOnly")]
    pub r#cdc_inserts_only: Box<Option<bool>>,
    /// Maximum length of the interval, defined in seconds, after which to output a file to Amazon S3. Default is `60`.
    #[builder(into, default)]
    #[serde(rename = "cdcMaxBatchInterval")]
    pub r#cdc_max_batch_interval: Box<Option<i32>>,
    /// Minimum file size condition as defined in kilobytes to output a file to Amazon S3. Default is `32000`. **NOTE:** Previously, this setting was measured in megabytes but now represents kilobytes. Update configurations accordingly.
    #[builder(into, default)]
    #[serde(rename = "cdcMinFileSize")]
    pub r#cdc_min_file_size: Box<Option<i32>>,
    /// Folder path of CDC files. For an S3 source, this setting is required if a task captures change data; otherwise, it's optional. If `cdc_path` is set, AWS DMS reads CDC files from this path and replicates the data changes to the target endpoint. Supported in AWS DMS versions 3.4.2 and later.
    #[builder(into, default)]
    #[serde(rename = "cdcPath")]
    pub r#cdc_path: Box<Option<String>>,
    /// Set to compress target files. Default is `NONE`. Valid values are `GZIP` and `NONE`.
    #[builder(into, default)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: Box<Option<String>>,
    /// Delimiter used to separate columns in the source files. Default is `,`.
    #[builder(into, default)]
    #[serde(rename = "csvDelimiter")]
    pub r#csv_delimiter: Box<Option<String>>,
    /// String to use for all columns not included in the supplemental log.
    #[builder(into, default)]
    #[serde(rename = "csvNoSupValue")]
    pub r#csv_no_sup_value: Box<Option<String>>,
    /// String to as null when writing to the target.
    #[builder(into, default)]
    #[serde(rename = "csvNullValue")]
    pub r#csv_null_value: Box<Option<String>>,
    /// Delimiter used to separate rows in the source files. Default is `\n`.
    #[builder(into, default)]
    #[serde(rename = "csvRowDelimiter")]
    pub r#csv_row_delimiter: Box<Option<String>>,
    /// Output format for the files that AWS DMS uses to create S3 objects. Valid values are `csv` and `parquet`. Default is `csv`.
    #[builder(into, default)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Box<Option<String>>,
    /// Size of one data page in bytes. Default is `1048576` (1 MiB).
    #[builder(into, default)]
    #[serde(rename = "dataPageSize")]
    pub r#data_page_size: Box<Option<i32>>,
    /// Date separating delimiter to use during folder partitioning. Valid values are `SLASH`, `UNDERSCORE`, `DASH`, and `NONE`. Default is `SLASH`.
    #[builder(into, default)]
    #[serde(rename = "datePartitionDelimiter")]
    pub r#date_partition_delimiter: Box<Option<String>>,
    /// Partition S3 bucket folders based on transaction commit dates. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "datePartitionEnabled")]
    pub r#date_partition_enabled: Box<Option<bool>>,
    /// Date format to use during folder partitioning. Use this parameter when `date_partition_enabled` is set to true. Valid values are `YYYYMMDD`, `YYYYMMDDHH`, `YYYYMM`, `MMYYYYDD`, and `DDMMYYYY`. Default is `YYYYMMDD`.
    #[builder(into, default)]
    #[serde(rename = "datePartitionSequence")]
    pub r#date_partition_sequence: Box<Option<String>>,
    /// Maximum size in bytes of an encoded dictionary page of a column. Default is `1048576` (1 MiB).
    #[builder(into, default)]
    #[serde(rename = "dictPageSizeLimit")]
    pub r#dict_page_size_limit: Box<Option<i32>>,
    /// Whether to enable statistics for Parquet pages and row groups. Default is `true`.
    #[builder(into, default)]
    #[serde(rename = "enableStatistics")]
    pub r#enable_statistics: Box<Option<bool>>,
    /// Type of encoding to use. Value values are `rle_dictionary`, `plain`, and `plain_dictionary`. Default is `rle_dictionary`.
    #[builder(into, default)]
    #[serde(rename = "encodingType")]
    pub r#encoding_type: Box<Option<String>>,
    /// Server-side encryption mode that you want to encrypt your .csv or .parquet object files copied to S3. Valid values are `SSE_S3` and `SSE_KMS`. Default is `SSE_S3`.
    #[builder(into, default)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Box<Option<String>>,
    /// JSON document that describes how AWS DMS should interpret the data.
    #[builder(into, default)]
    #[serde(rename = "externalTableDefinition")]
    pub r#external_table_definition: Box<Option<String>>,
    /// Whether to integrate AWS Glue Data Catalog with an Amazon S3 target. See [Using AWS Glue Data Catalog with an Amazon S3 target for AWS DMS](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.GlueCatalog) for more information. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "glueCatalogGeneration")]
    pub r#glue_catalog_generation: Box<Option<bool>>,
    /// When this value is set to `1`, DMS ignores the first row header in a .csv file. Default is `0`.
    #[builder(into, default)]
    #[serde(rename = "ignoreHeaderRows")]
    pub r#ignore_header_rows: Box<Option<i32>>,
    /// Whether to enable a full load to write INSERT operations to the .csv output files only to indicate how the rows were added to the source database. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeOpForFullLoad")]
    pub r#include_op_for_full_load: Box<Option<bool>>,
    /// Maximum size (in KB) of any .csv file to be created while migrating to an S3 target during full load. Valid values are from `1` to `1048576`. Default is `1048576` (1 GB).
    #[builder(into, default)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: Box<Option<i32>>,
    /// Specifies the precision of any TIMESTAMP column values written to an S3 object file in .parquet format. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "parquetTimestampInMillisecond")]
    pub r#parquet_timestamp_in_millisecond: Box<Option<bool>>,
    /// Version of the .parquet file format. Default is `parquet-1-0`. Valid values are `parquet-1-0` and `parquet-2-0`.
    #[builder(into, default)]
    #[serde(rename = "parquetVersion")]
    pub r#parquet_version: Box<Option<String>>,
    /// Whether DMS saves the transaction order for a CDC load on the S3 target specified by `cdc_path`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "preserveTransactions")]
    pub r#preserve_transactions: Box<Option<bool>>,
    /// For an S3 source, whether each leading double quotation mark has to be followed by an ending double quotation mark. Default is `true`.
    #[builder(into, default)]
    #[serde(rename = "rfc4180")]
    pub r#rfc_4180: Box<Option<bool>>,
    /// Number of rows in a row group. Default is `10000`.
    #[builder(into, default)]
    #[serde(rename = "rowGroupLength")]
    pub r#row_group_length: Box<Option<i32>>,
    /// ARN or Id of KMS Key to use when `encryption_mode` is `SSE_KMS`.
    #[builder(into, default)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: Box<Option<String>>,
    /// ARN of the IAM Role with permissions to read from or write to the S3 Bucket.
    #[builder(into, default)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<Option<String>>,
    /// Column to add with timestamp information to the endpoint data for an Amazon S3 target.
    #[builder(into, default)]
    #[serde(rename = "timestampColumnName")]
    pub r#timestamp_column_name: Box<Option<String>>,
    /// Whether to use `csv_no_sup_value` for columns not included in the supplemental log.
    #[builder(into, default)]
    #[serde(rename = "useCsvNoSupValue")]
    pub r#use_csv_no_sup_value: Box<Option<bool>>,
    /// When set to true, uses the task start time as the timestamp column value instead of the time data is written to target. For full load, when set to true, each row of the timestamp column contains the task start time. For CDC loads, each row of the timestamp column contains the transaction commit time. When set to false, the full load timestamp in the timestamp column increments with the time data arrives at the target. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "useTaskStartTimeForFullLoadTimestamp")]
    pub r#use_task_start_time_for_full_load_timestamp: Box<Option<bool>>,
}
