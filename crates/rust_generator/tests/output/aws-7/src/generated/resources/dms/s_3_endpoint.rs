/// Provides a DMS (Data Migration Service) S3 endpoint resource. DMS S3 endpoints can be created, updated, deleted, and imported.
///
/// > **Note:** AWS is deprecating `extra_connection_attributes`, such as used with `aws.dms.Endpoint`. This resource is an alternative to `aws.dms.Endpoint` and does not use `extra_connection_attributes`. (AWS currently includes `extra_connection_attributes` in the raw responses to the AWS Provider requests and so they may be visible in the logs.)
///
/// > **Note:** Some of this resource's arguments have default values that come from the AWS Provider. Other default values are provided by AWS and subject to change without notice. When relying on AWS defaults, the provider state will often have a zero value. For example, the AWS Provider does not provide a default for `cdc_max_batch_interval` but the AWS default is `60` (seconds). However, the provider state will show `0` since this is the value return by AWS when no value is present. Below, we aim to flag the defaults that come from AWS (_e.g._, "AWS default...").
///
/// ## Example Usage
///
/// ### Minimal Configuration
///
/// This is the minimal configuration for an `aws.dms.S3Endpoint`. This endpoint will rely on the AWS Provider and AWS defaults.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = s_3_endpoint::create(
///         "example",
///         S3EndpointArgs::builder()
///             .bucket_name("beckut_name")
///             .endpoint_id("donnedtipi")
///             .endpoint_type("target")
///             .service_access_role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Complete Configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:dms:S3Endpoint
///     properties:
///       endpointId: donnedtipi
///       endpointType: target
///       sslMode: none
///       tags:
///         Name: donnedtipi
///         Update: to-update
///         Remove: to-remove
///       addColumnName: true
///       addTrailingPaddingCharacter: false
///       bucketFolder: folder
///       bucketName: bucket_name
///       cannedAclForObjects: private
///       cdcInsertsAndUpdates: true
///       cdcInsertsOnly: false
///       cdcMaxBatchInterval: 100
///       cdcMinFileSize: 16
///       cdcPath: cdc/path
///       compressionType: GZIP
///       csvDelimiter: ;
///       csvNoSupValue: x
///       csvNullValue: '?'
///       csvRowDelimiter: \r\n
///       dataFormat: parquet
///       dataPageSize: 1.1e+06
///       datePartitionDelimiter: UNDERSCORE
///       datePartitionEnabled: true
///       datePartitionSequence: yyyymmddhh
///       datePartitionTimezone: Asia/Seoul
///       dictPageSizeLimit: 1e+06
///       enableStatistics: false
///       encodingType: plain
///       encryptionMode: SSE_S3
///       expectedBucketOwner: ${current.accountId}
///       externalTableDefinition: etd
///       ignoreHeaderRows: 1
///       includeOpForFullLoad: true
///       maxFileSize: 1e+06
///       parquetTimestampInMillisecond: true
///       parquetVersion: parquet-2-0
///       preserveTransactions: false
///       rfc4180: false
///       rowGroupLength: 11000
///       serverSideEncryptionKmsKeyId: ${exampleAwsKmsKey.arn}
///       serviceAccessRoleArn: ${exampleAwsIamRole.arn}
///       timestampColumnName: tx_commit_time
///       useCsvNoSupValue: false
///       useTaskStartTimeForFullLoadTimestamp: true
///       glueCatalogGeneration: true
///     options:
///       dependsOn:
///         - ${exampleAwsIamRolePolicy}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import endpoints using the `endpoint_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/s3Endpoint:S3Endpoint example example-dms-endpoint-tf
/// ```
pub mod s_3_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct S3EndpointArgs {
        /// Whether to add column name information to the .csv output file. Default is `false`.
        #[builder(into, default)]
        pub add_column_name: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to add padding. Default is `false`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub add_trailing_padding_character: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// S3 object prefix.
        #[builder(into, default)]
        pub bucket_folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 bucket name.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Predefined (canned) access control list for objects created in an S3 bucket. Valid values include `none`, `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Default is `none`.
        #[builder(into, default)]
        pub canned_acl_for_objects: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to write insert and update operations to .csv or .parquet output files. Default is `false`.
        #[builder(into, default)]
        pub cdc_inserts_and_updates: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to write insert operations to .csv or .parquet output files. Default is `false`.
        #[builder(into, default)]
        pub cdc_inserts_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Maximum length of the interval, defined in seconds, after which to output a file to Amazon S3. (AWS default is `60`.)
        #[builder(into, default)]
        pub cdc_max_batch_interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum file size condition as defined in kilobytes to output a file to Amazon S3. (AWS default is 32000 KB.)
        #[builder(into, default)]
        pub cdc_min_file_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Folder path of CDC files. If `cdc_path` is set, AWS DMS reads CDC files from this path and replicates the data changes to the target endpoint. Supported in AWS DMS versions 3.4.2 and later.
        #[builder(into, default)]
        pub cdc_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN for the certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to compress target files. Valid values are `GZIP` and `NONE`. Default is `NONE`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub compression_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Delimiter used to separate columns in the source files. Default is `,`.
        #[builder(into, default)]
        pub csv_delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Only applies if output files for a CDC load are written in .csv format. If `use_csv_no_sup_value` is set to `true`, string to use for all columns not included in the supplemental log. If you do not specify a string value, DMS uses the null value for these columns regardless of `use_csv_no_sup_value`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub csv_no_sup_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// String to as null when writing to the target. (AWS default is `NULL`.)
        #[builder(into, default)]
        pub csv_null_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Delimiter used to separate rows in the source files. Default is newline (_i.e._, `\n`).
        #[builder(into, default)]
        pub csv_row_delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Output format for the files that AWS DMS uses to create S3 objects. Valid values are `csv` and `parquet`.  (Ignored for source endpoints -- only `csv` is valid.)
        #[builder(into, default)]
        pub data_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Size of one data page in bytes. (AWS default is 1 MiB, _i.e._, `1048576`.)
        #[builder(into, default)]
        pub data_page_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Date separating delimiter to use during folder partitioning. Valid values are `SLASH`, `UNDERSCORE`, `DASH`, and `NONE`. (AWS default is `SLASH`.) (Ignored for source endpoints.)
        #[builder(into, default)]
        pub date_partition_delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Partition S3 bucket folders based on transaction commit dates. Default is `false`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub date_partition_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Date format to use during folder partitioning. Use this parameter when `date_partition_enabled` is set to true. Valid values are `YYYYMMDD`, `YYYYMMDDHH`, `YYYYMM`, `MMYYYYDD`, and `DDMMYYYY`. (AWS default is `YYYYMMDD`.) (Ignored for source endpoints.)
        #[builder(into, default)]
        pub date_partition_sequence: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Convert the current UTC time to a timezone. The conversion occurs when a date partition folder is created and a CDC filename is generated. The timezone format is Area/Location (_e.g._, `Europe/Paris`). Use this when `date_partition_enabled` is `true`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub date_partition_timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Undocumented argument for use as directed by AWS Support.
        #[builder(into, default)]
        pub detach_target_on_lob_lookup_failure_parquet: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Maximum size in bytes of an encoded dictionary page of a column. (AWS default is 1 MiB, _i.e._, `1048576`.)
        #[builder(into, default)]
        pub dict_page_size_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether to enable statistics for Parquet pages and row groups. Default is `true`.
        #[builder(into, default)]
        pub enable_statistics: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of encoding to use. Value values are `rle_dictionary`, `plain`, and `plain_dictionary`. (AWS default is `rle_dictionary`.)
        #[builder(into, default)]
        pub encoding_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Server-side encryption mode that you want to encrypt your .csv or .parquet object files copied to S3. Valid values are `SSE_S3` and `SSE_KMS`. (AWS default is `SSE_S3`.) (Ignored for source endpoints -- only `SSE_S3` is valid.)
        #[builder(into, default)]
        pub encryption_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        #[builder(into)]
        pub endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of endpoint. Valid values are `source`, `target`.
        #[builder(into)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Bucket owner to prevent sniping. Value is an AWS account ID.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON document that describes how AWS DMS should interpret the data.
        #[builder(into, default)]
        pub external_table_definition: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether to integrate AWS Glue Data Catalog with an Amazon S3 target. See [Using AWS Glue Data Catalog with an Amazon S3 target for AWS DMS](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.GlueCatalog) for more information. Default is `false`.
        #[builder(into, default)]
        pub glue_catalog_generation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When this value is set to `1`, DMS ignores the first row header in a .csv file. (AWS default is `0`.)
        #[builder(into, default)]
        pub ignore_header_rows: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether to enable a full load to write INSERT operations to the .csv output files only to indicate how the rows were added to the source database. Default is `false`.
        #[builder(into, default)]
        pub include_op_for_full_load: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ARN for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum size (in KB) of any .csv file to be created while migrating to an S3 target during full load. Valid values are from `1` to `1048576`. (AWS default is 1 GB, _i.e._, `1048576`.)
        #[builder(into, default)]
        pub max_file_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the precision of any TIMESTAMP column values written to an S3 object file in .parquet format. Default is `false`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub parquet_timestamp_in_millisecond: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Version of the .parquet file format. Valid values are `parquet-1-0` and `parquet-2-0`. (AWS default is `parquet-1-0`.) (Ignored for source endpoints.)
        #[builder(into, default)]
        pub parquet_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether DMS saves the transaction order for a CDC load on the S3 target specified by `cdc_path`. Default is `false`. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub preserve_transactions: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// For an S3 source, whether each leading double quotation mark has to be followed by an ending double quotation mark. Default is `true`.
        #[builder(into, default)]
        pub rfc4180: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Number of rows in a row group. (AWS default is `10000`.)
        #[builder(into, default)]
        pub row_group_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// When `encryption_mode` is `SSE_KMS`, ARN for the AWS KMS key. (Ignored for source endpoints -- only `SSE_S3` `encryption_mode` is valid.)
        #[builder(into, default)]
        pub server_side_encryption_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ARN of the IAM role with permissions to the S3 Bucket.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_access_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SSL mode to use for the connection. Valid values are `none`, `require`, `verify-ca`, `verify-full`. (AWS default is `none`.)
        #[builder(into, default)]
        pub ssl_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Column to add with timestamp information to the endpoint data for an Amazon S3 target.
        #[builder(into, default)]
        pub timestamp_column_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to use `csv_no_sup_value` for columns not included in the supplemental log. (Ignored for source endpoints.)
        #[builder(into, default)]
        pub use_csv_no_sup_value: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When set to `true`, uses the task start time as the timestamp column value instead of the time data is written to target. For full load, when set to `true`, each row of the timestamp column contains the task start time. For CDC loads, each row of the timestamp column contains the transaction commit time.When set to false, the full load timestamp in the timestamp column increments with the time data arrives at the target. Default is `false`.
        #[builder(into, default)]
        pub use_task_start_time_for_full_load_timestamp: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct S3EndpointResult {
        /// Whether to add column name information to the .csv output file. Default is `false`.
        pub add_column_name: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to add padding. Default is `false`. (Ignored for source endpoints.)
        pub add_trailing_padding_character: pulumi_gestalt_rust::Output<Option<bool>>,
        /// S3 object prefix.
        pub bucket_folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 bucket name.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// Predefined (canned) access control list for objects created in an S3 bucket. Valid values include `none`, `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Default is `none`.
        pub canned_acl_for_objects: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to write insert and update operations to .csv or .parquet output files. Default is `false`.
        pub cdc_inserts_and_updates: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to write insert operations to .csv or .parquet output files. Default is `false`.
        pub cdc_inserts_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Maximum length of the interval, defined in seconds, after which to output a file to Amazon S3. (AWS default is `60`.)
        pub cdc_max_batch_interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum file size condition as defined in kilobytes to output a file to Amazon S3. (AWS default is 32000 KB.)
        pub cdc_min_file_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Folder path of CDC files. If `cdc_path` is set, AWS DMS reads CDC files from this path and replicates the data changes to the target endpoint. Supported in AWS DMS versions 3.4.2 and later.
        pub cdc_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN for the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Set to compress target files. Valid values are `GZIP` and `NONE`. Default is `NONE`. (Ignored for source endpoints.)
        pub compression_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Delimiter used to separate columns in the source files. Default is `,`.
        pub csv_delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        /// Only applies if output files for a CDC load are written in .csv format. If `use_csv_no_sup_value` is set to `true`, string to use for all columns not included in the supplemental log. If you do not specify a string value, DMS uses the null value for these columns regardless of `use_csv_no_sup_value`. (Ignored for source endpoints.)
        pub csv_no_sup_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// String to as null when writing to the target. (AWS default is `NULL`.)
        pub csv_null_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// Delimiter used to separate rows in the source files. Default is newline (_i.e._, `\n`).
        pub csv_row_delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output format for the files that AWS DMS uses to create S3 objects. Valid values are `csv` and `parquet`.  (Ignored for source endpoints -- only `csv` is valid.)
        pub data_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// Size of one data page in bytes. (AWS default is 1 MiB, _i.e._, `1048576`.)
        pub data_page_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Date separating delimiter to use during folder partitioning. Valid values are `SLASH`, `UNDERSCORE`, `DASH`, and `NONE`. (AWS default is `SLASH`.) (Ignored for source endpoints.)
        pub date_partition_delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        /// Partition S3 bucket folders based on transaction commit dates. Default is `false`. (Ignored for source endpoints.)
        pub date_partition_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Date format to use during folder partitioning. Use this parameter when `date_partition_enabled` is set to true. Valid values are `YYYYMMDD`, `YYYYMMDDHH`, `YYYYMM`, `MMYYYYDD`, and `DDMMYYYY`. (AWS default is `YYYYMMDD`.) (Ignored for source endpoints.)
        pub date_partition_sequence: pulumi_gestalt_rust::Output<Option<String>>,
        /// Convert the current UTC time to a timezone. The conversion occurs when a date partition folder is created and a CDC filename is generated. The timezone format is Area/Location (_e.g._, `Europe/Paris`). Use this when `date_partition_enabled` is `true`. (Ignored for source endpoints.)
        pub date_partition_timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Undocumented argument for use as directed by AWS Support.
        pub detach_target_on_lob_lookup_failure_parquet: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Maximum size in bytes of an encoded dictionary page of a column. (AWS default is 1 MiB, _i.e._, `1048576`.)
        pub dict_page_size_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether to enable statistics for Parquet pages and row groups. Default is `true`.
        pub enable_statistics: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Type of encoding to use. Value values are `rle_dictionary`, `plain`, and `plain_dictionary`. (AWS default is `rle_dictionary`.)
        pub encoding_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Server-side encryption mode that you want to encrypt your .csv or .parquet object files copied to S3. Valid values are `SSE_S3` and `SSE_KMS`. (AWS default is `SSE_S3`.) (Ignored for source endpoints -- only `SSE_S3` is valid.)
        pub encryption_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN for the endpoint.
        pub endpoint_arn: pulumi_gestalt_rust::Output<String>,
        /// Database endpoint identifier. Identifiers must contain from 1 to 255 alphanumeric characters or hyphens, begin with a letter, contain only ASCII letters, digits, and hyphens, not end with a hyphen, and not contain two consecutive hyphens.
        pub endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Type of endpoint. Valid values are `source`, `target`.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// Expanded name for the engine name.
        pub engine_display_name: pulumi_gestalt_rust::Output<String>,
        /// Bucket owner to prevent sniping. Value is an AWS account ID.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Can be used for cross-account validation. Use it in another account with `aws.dms.S3Endpoint` to create the endpoint cross-account.
        pub external_id: pulumi_gestalt_rust::Output<String>,
        /// JSON document that describes how AWS DMS should interpret the data.
        pub external_table_definition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to integrate AWS Glue Data Catalog with an Amazon S3 target. See [Using AWS Glue Data Catalog with an Amazon S3 target for AWS DMS](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.GlueCatalog) for more information. Default is `false`.
        pub glue_catalog_generation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When this value is set to `1`, DMS ignores the first row header in a .csv file. (AWS default is `0`.)
        pub ignore_header_rows: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether to enable a full load to write INSERT operations to the .csv output files only to indicate how the rows were added to the source database. Default is `false`.
        pub include_op_for_full_load: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN for the KMS key that will be used to encrypt the connection parameters. If you do not specify a value for `kms_key_arn`, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Maximum size (in KB) of any .csv file to be created while migrating to an S3 target during full load. Valid values are from `1` to `1048576`. (AWS default is 1 GB, _i.e._, `1048576`.)
        pub max_file_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the precision of any TIMESTAMP column values written to an S3 object file in .parquet format. Default is `false`. (Ignored for source endpoints.)
        pub parquet_timestamp_in_millisecond: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Version of the .parquet file format. Valid values are `parquet-1-0` and `parquet-2-0`. (AWS default is `parquet-1-0`.) (Ignored for source endpoints.)
        pub parquet_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether DMS saves the transaction order for a CDC load on the S3 target specified by `cdc_path`. Default is `false`. (Ignored for source endpoints.)
        pub preserve_transactions: pulumi_gestalt_rust::Output<Option<bool>>,
        /// For an S3 source, whether each leading double quotation mark has to be followed by an ending double quotation mark. Default is `true`.
        pub rfc4180: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Number of rows in a row group. (AWS default is `10000`.)
        pub row_group_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// When `encryption_mode` is `SSE_KMS`, ARN for the AWS KMS key. (Ignored for source endpoints -- only `SSE_S3` `encryption_mode` is valid.)
        pub server_side_encryption_kms_key_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// ARN of the IAM role with permissions to the S3 Bucket.
        ///
        /// The following arguments are optional:
        pub service_access_role_arn: pulumi_gestalt_rust::Output<String>,
        /// SSL mode to use for the connection. Valid values are `none`, `require`, `verify-ca`, `verify-full`. (AWS default is `none`.)
        pub ssl_mode: pulumi_gestalt_rust::Output<String>,
        /// Status of the endpoint.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Column to add with timestamp information to the endpoint data for an Amazon S3 target.
        pub timestamp_column_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to use `csv_no_sup_value` for columns not included in the supplemental log. (Ignored for source endpoints.)
        pub use_csv_no_sup_value: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When set to `true`, uses the task start time as the timestamp column value instead of the time data is written to target. For full load, when set to `true`, each row of the timestamp column contains the task start time. For CDC loads, each row of the timestamp column contains the transaction commit time.When set to false, the full load timestamp in the timestamp column increments with the time data arrives at the target. Default is `false`.
        pub use_task_start_time_for_full_load_timestamp: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: S3EndpointArgs,
    ) -> S3EndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let add_column_name_binding = args
            .add_column_name
            .get_output(context)
            .get_inner();
        let add_trailing_padding_character_binding = args
            .add_trailing_padding_character
            .get_output(context)
            .get_inner();
        let bucket_folder_binding = args.bucket_folder.get_output(context).get_inner();
        let bucket_name_binding = args.bucket_name.get_output(context).get_inner();
        let canned_acl_for_objects_binding = args
            .canned_acl_for_objects
            .get_output(context)
            .get_inner();
        let cdc_inserts_and_updates_binding = args
            .cdc_inserts_and_updates
            .get_output(context)
            .get_inner();
        let cdc_inserts_only_binding = args
            .cdc_inserts_only
            .get_output(context)
            .get_inner();
        let cdc_max_batch_interval_binding = args
            .cdc_max_batch_interval
            .get_output(context)
            .get_inner();
        let cdc_min_file_size_binding = args
            .cdc_min_file_size
            .get_output(context)
            .get_inner();
        let cdc_path_binding = args.cdc_path.get_output(context).get_inner();
        let certificate_arn_binding = args
            .certificate_arn
            .get_output(context)
            .get_inner();
        let compression_type_binding = args
            .compression_type
            .get_output(context)
            .get_inner();
        let csv_delimiter_binding = args.csv_delimiter.get_output(context).get_inner();
        let csv_no_sup_value_binding = args
            .csv_no_sup_value
            .get_output(context)
            .get_inner();
        let csv_null_value_binding = args.csv_null_value.get_output(context).get_inner();
        let csv_row_delimiter_binding = args
            .csv_row_delimiter
            .get_output(context)
            .get_inner();
        let data_format_binding = args.data_format.get_output(context).get_inner();
        let data_page_size_binding = args.data_page_size.get_output(context).get_inner();
        let date_partition_delimiter_binding = args
            .date_partition_delimiter
            .get_output(context)
            .get_inner();
        let date_partition_enabled_binding = args
            .date_partition_enabled
            .get_output(context)
            .get_inner();
        let date_partition_sequence_binding = args
            .date_partition_sequence
            .get_output(context)
            .get_inner();
        let date_partition_timezone_binding = args
            .date_partition_timezone
            .get_output(context)
            .get_inner();
        let detach_target_on_lob_lookup_failure_parquet_binding = args
            .detach_target_on_lob_lookup_failure_parquet
            .get_output(context)
            .get_inner();
        let dict_page_size_limit_binding = args
            .dict_page_size_limit
            .get_output(context)
            .get_inner();
        let enable_statistics_binding = args
            .enable_statistics
            .get_output(context)
            .get_inner();
        let encoding_type_binding = args.encoding_type.get_output(context).get_inner();
        let encryption_mode_binding = args
            .encryption_mode
            .get_output(context)
            .get_inner();
        let endpoint_id_binding = args.endpoint_id.get_output(context).get_inner();
        let endpoint_type_binding = args.endpoint_type.get_output(context).get_inner();
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context)
            .get_inner();
        let external_table_definition_binding = args
            .external_table_definition
            .get_output(context)
            .get_inner();
        let glue_catalog_generation_binding = args
            .glue_catalog_generation
            .get_output(context)
            .get_inner();
        let ignore_header_rows_binding = args
            .ignore_header_rows
            .get_output(context)
            .get_inner();
        let include_op_for_full_load_binding = args
            .include_op_for_full_load
            .get_output(context)
            .get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let max_file_size_binding = args.max_file_size.get_output(context).get_inner();
        let parquet_timestamp_in_millisecond_binding = args
            .parquet_timestamp_in_millisecond
            .get_output(context)
            .get_inner();
        let parquet_version_binding = args
            .parquet_version
            .get_output(context)
            .get_inner();
        let preserve_transactions_binding = args
            .preserve_transactions
            .get_output(context)
            .get_inner();
        let rfc4180_binding = args.rfc4180.get_output(context).get_inner();
        let row_group_length_binding = args
            .row_group_length
            .get_output(context)
            .get_inner();
        let server_side_encryption_kms_key_id_binding = args
            .server_side_encryption_kms_key_id
            .get_output(context)
            .get_inner();
        let service_access_role_arn_binding = args
            .service_access_role_arn
            .get_output(context)
            .get_inner();
        let ssl_mode_binding = args.ssl_mode.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timestamp_column_name_binding = args
            .timestamp_column_name
            .get_output(context)
            .get_inner();
        let use_csv_no_sup_value_binding = args
            .use_csv_no_sup_value
            .get_output(context)
            .get_inner();
        let use_task_start_time_for_full_load_timestamp_binding = args
            .use_task_start_time_for_full_load_timestamp
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/s3Endpoint:S3Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addColumnName".into(),
                    value: &add_column_name_binding,
                },
                register_interface::ObjectField {
                    name: "addTrailingPaddingCharacter".into(),
                    value: &add_trailing_padding_character_binding,
                },
                register_interface::ObjectField {
                    name: "bucketFolder".into(),
                    value: &bucket_folder_binding,
                },
                register_interface::ObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "cannedAclForObjects".into(),
                    value: &canned_acl_for_objects_binding,
                },
                register_interface::ObjectField {
                    name: "cdcInsertsAndUpdates".into(),
                    value: &cdc_inserts_and_updates_binding,
                },
                register_interface::ObjectField {
                    name: "cdcInsertsOnly".into(),
                    value: &cdc_inserts_only_binding,
                },
                register_interface::ObjectField {
                    name: "cdcMaxBatchInterval".into(),
                    value: &cdc_max_batch_interval_binding,
                },
                register_interface::ObjectField {
                    name: "cdcMinFileSize".into(),
                    value: &cdc_min_file_size_binding,
                },
                register_interface::ObjectField {
                    name: "cdcPath".into(),
                    value: &cdc_path_binding,
                },
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "compressionType".into(),
                    value: &compression_type_binding,
                },
                register_interface::ObjectField {
                    name: "csvDelimiter".into(),
                    value: &csv_delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "csvNoSupValue".into(),
                    value: &csv_no_sup_value_binding,
                },
                register_interface::ObjectField {
                    name: "csvNullValue".into(),
                    value: &csv_null_value_binding,
                },
                register_interface::ObjectField {
                    name: "csvRowDelimiter".into(),
                    value: &csv_row_delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "dataFormat".into(),
                    value: &data_format_binding,
                },
                register_interface::ObjectField {
                    name: "dataPageSize".into(),
                    value: &data_page_size_binding,
                },
                register_interface::ObjectField {
                    name: "datePartitionDelimiter".into(),
                    value: &date_partition_delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "datePartitionEnabled".into(),
                    value: &date_partition_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "datePartitionSequence".into(),
                    value: &date_partition_sequence_binding,
                },
                register_interface::ObjectField {
                    name: "datePartitionTimezone".into(),
                    value: &date_partition_timezone_binding,
                },
                register_interface::ObjectField {
                    name: "detachTargetOnLobLookupFailureParquet".into(),
                    value: &detach_target_on_lob_lookup_failure_parquet_binding,
                },
                register_interface::ObjectField {
                    name: "dictPageSizeLimit".into(),
                    value: &dict_page_size_limit_binding,
                },
                register_interface::ObjectField {
                    name: "enableStatistics".into(),
                    value: &enable_statistics_binding,
                },
                register_interface::ObjectField {
                    name: "encodingType".into(),
                    value: &encoding_type_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionMode".into(),
                    value: &encryption_mode_binding,
                },
                register_interface::ObjectField {
                    name: "endpointId".into(),
                    value: &endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "externalTableDefinition".into(),
                    value: &external_table_definition_binding,
                },
                register_interface::ObjectField {
                    name: "glueCatalogGeneration".into(),
                    value: &glue_catalog_generation_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreHeaderRows".into(),
                    value: &ignore_header_rows_binding,
                },
                register_interface::ObjectField {
                    name: "includeOpForFullLoad".into(),
                    value: &include_op_for_full_load_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "maxFileSize".into(),
                    value: &max_file_size_binding,
                },
                register_interface::ObjectField {
                    name: "parquetTimestampInMillisecond".into(),
                    value: &parquet_timestamp_in_millisecond_binding,
                },
                register_interface::ObjectField {
                    name: "parquetVersion".into(),
                    value: &parquet_version_binding,
                },
                register_interface::ObjectField {
                    name: "preserveTransactions".into(),
                    value: &preserve_transactions_binding,
                },
                register_interface::ObjectField {
                    name: "rfc4180".into(),
                    value: &rfc4180_binding,
                },
                register_interface::ObjectField {
                    name: "rowGroupLength".into(),
                    value: &row_group_length_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryptionKmsKeyId".into(),
                    value: &server_side_encryption_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccessRoleArn".into(),
                    value: &service_access_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sslMode".into(),
                    value: &ssl_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timestampColumnName".into(),
                    value: &timestamp_column_name_binding,
                },
                register_interface::ObjectField {
                    name: "useCsvNoSupValue".into(),
                    value: &use_csv_no_sup_value_binding,
                },
                register_interface::ObjectField {
                    name: "useTaskStartTimeForFullLoadTimestamp".into(),
                    value: &use_task_start_time_for_full_load_timestamp_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        S3EndpointResult {
            add_column_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addColumnName"),
            ),
            add_trailing_padding_character: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addTrailingPaddingCharacter"),
            ),
            bucket_folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketFolder"),
            ),
            bucket_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketName"),
            ),
            canned_acl_for_objects: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cannedAclForObjects"),
            ),
            cdc_inserts_and_updates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdcInsertsAndUpdates"),
            ),
            cdc_inserts_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdcInsertsOnly"),
            ),
            cdc_max_batch_interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdcMaxBatchInterval"),
            ),
            cdc_min_file_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdcMinFileSize"),
            ),
            cdc_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdcPath"),
            ),
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            compression_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compressionType"),
            ),
            csv_delimiter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("csvDelimiter"),
            ),
            csv_no_sup_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("csvNoSupValue"),
            ),
            csv_null_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("csvNullValue"),
            ),
            csv_row_delimiter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("csvRowDelimiter"),
            ),
            data_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFormat"),
            ),
            data_page_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataPageSize"),
            ),
            date_partition_delimiter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datePartitionDelimiter"),
            ),
            date_partition_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datePartitionEnabled"),
            ),
            date_partition_sequence: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datePartitionSequence"),
            ),
            date_partition_timezone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datePartitionTimezone"),
            ),
            detach_target_on_lob_lookup_failure_parquet: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detachTargetOnLobLookupFailureParquet"),
            ),
            dict_page_size_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dictPageSizeLimit"),
            ),
            enable_statistics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableStatistics"),
            ),
            encoding_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encodingType"),
            ),
            encryption_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionMode"),
            ),
            endpoint_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointArn"),
            ),
            endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointId"),
            ),
            endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointType"),
            ),
            engine_display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineDisplayName"),
            ),
            expected_bucket_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expectedBucketOwner"),
            ),
            external_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalId"),
            ),
            external_table_definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalTableDefinition"),
            ),
            glue_catalog_generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("glueCatalogGeneration"),
            ),
            ignore_header_rows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignoreHeaderRows"),
            ),
            include_op_for_full_load: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeOpForFullLoad"),
            ),
            kms_key_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            max_file_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxFileSize"),
            ),
            parquet_timestamp_in_millisecond: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parquetTimestampInMillisecond"),
            ),
            parquet_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parquetVersion"),
            ),
            preserve_transactions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preserveTransactions"),
            ),
            rfc4180: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rfc4180"),
            ),
            row_group_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rowGroupLength"),
            ),
            server_side_encryption_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverSideEncryptionKmsKeyId"),
            ),
            service_access_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccessRoleArn"),
            ),
            ssl_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslMode"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timestamp_column_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timestampColumnName"),
            ),
            use_csv_no_sup_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useCsvNoSupValue"),
            ),
            use_task_start_time_for_full_load_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("useTaskStartTimeForFullLoadTimestamp"),
            ),
        }
    }
}
