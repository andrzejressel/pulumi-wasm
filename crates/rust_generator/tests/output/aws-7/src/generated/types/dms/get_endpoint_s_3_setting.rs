#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointS3Setting {
    #[builder(into)]
    #[serde(rename = "addColumnName")]
    pub r#add_column_name: Box<bool>,
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: Box<String>,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "cannedAclForObjects")]
    pub r#canned_acl_for_objects: Box<String>,
    #[builder(into)]
    #[serde(rename = "cdcInsertsAndUpdates")]
    pub r#cdc_inserts_and_updates: Box<bool>,
    #[builder(into)]
    #[serde(rename = "cdcInsertsOnly")]
    pub r#cdc_inserts_only: Box<bool>,
    #[builder(into)]
    #[serde(rename = "cdcMaxBatchInterval")]
    pub r#cdc_max_batch_interval: Box<i32>,
    #[builder(into)]
    #[serde(rename = "cdcMinFileSize")]
    pub r#cdc_min_file_size: Box<i32>,
    #[builder(into)]
    #[serde(rename = "cdcPath")]
    pub r#cdc_path: Box<String>,
    #[builder(into)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "csvDelimiter")]
    pub r#csv_delimiter: Box<String>,
    #[builder(into)]
    #[serde(rename = "csvNoSupValue")]
    pub r#csv_no_sup_value: Box<String>,
    #[builder(into)]
    #[serde(rename = "csvNullValue")]
    pub r#csv_null_value: Box<String>,
    #[builder(into)]
    #[serde(rename = "csvRowDelimiter")]
    pub r#csv_row_delimiter: Box<String>,
    #[builder(into)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Box<String>,
    #[builder(into)]
    #[serde(rename = "dataPageSize")]
    pub r#data_page_size: Box<i32>,
    #[builder(into)]
    #[serde(rename = "datePartitionDelimiter")]
    pub r#date_partition_delimiter: Box<String>,
    #[builder(into)]
    #[serde(rename = "datePartitionEnabled")]
    pub r#date_partition_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "datePartitionSequence")]
    pub r#date_partition_sequence: Box<String>,
    #[builder(into)]
    #[serde(rename = "dictPageSizeLimit")]
    pub r#dict_page_size_limit: Box<i32>,
    #[builder(into)]
    #[serde(rename = "enableStatistics")]
    pub r#enable_statistics: Box<bool>,
    #[builder(into)]
    #[serde(rename = "encodingType")]
    pub r#encoding_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "externalTableDefinition")]
    pub r#external_table_definition: Box<String>,
    #[builder(into)]
    #[serde(rename = "glueCatalogGeneration")]
    pub r#glue_catalog_generation: Box<bool>,
    #[builder(into)]
    #[serde(rename = "ignoreHeaderRows")]
    pub r#ignore_header_rows: Box<i32>,
    #[builder(into)]
    #[serde(rename = "ignoreHeadersRow")]
    pub r#ignore_headers_row: Box<i32>,
    #[builder(into)]
    #[serde(rename = "includeOpForFullLoad")]
    pub r#include_op_for_full_load: Box<bool>,
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: Box<i32>,
    #[builder(into)]
    #[serde(rename = "parquetTimestampInMillisecond")]
    pub r#parquet_timestamp_in_millisecond: Box<bool>,
    #[builder(into)]
    #[serde(rename = "parquetVersion")]
    pub r#parquet_version: Box<String>,
    #[builder(into)]
    #[serde(rename = "preserveTransactions")]
    pub r#preserve_transactions: Box<bool>,
    #[builder(into)]
    #[serde(rename = "rfc4180")]
    pub r#rfc_4180: Box<bool>,
    #[builder(into)]
    #[serde(rename = "rowGroupLength")]
    pub r#row_group_length: Box<i32>,
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "timestampColumnName")]
    pub r#timestamp_column_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "useCsvNoSupValue")]
    pub r#use_csv_no_sup_value: Box<bool>,
    #[builder(into)]
    #[serde(rename = "useTaskStartTimeForFullLoadTimestamp")]
    pub r#use_task_start_time_for_full_load_timestamp: Box<bool>,
}
