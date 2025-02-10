/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder()
///             .bucket_prefix("example")
///             .force_destroy(true)
///             .build_struct(),
///     );
///     let exampleTable = table::create(
///         "exampleTable",
///         TableArgs::builder()
///             .attributes(
///                 vec![
///                     TableAttribute::builder().name("user_id"). type ("S").build_struct(),
///                 ],
///             )
///             .billing_mode("PAY_PER_REQUEST")
///             .hash_key("user_id")
///             .name("example-table-1")
///             .point_in_time_recovery(
///                 TablePointInTimeRecovery::builder().enabled(true).build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTableExport = table_export::create(
///         "exampleTableExport",
///         TableExportArgs::builder()
///             .s_3_bucket("${example.id}")
///             .table_arn("${exampleTable.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example with export time
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table_export::create(
///         "example",
///         TableExportArgs::builder()
///             .export_time("2023-04-02T11:30:13+01:00")
///             .s_3_bucket("${exampleAwsS3Bucket.id}")
///             .table_arn("${exampleAwsDynamodbTable.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB table exports using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/tableExport:TableExport example arn:aws:dynamodb:us-west-2:12345678911:table/my-table-1/export/01580735656614-2c2f422e
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableExportArgs {
        /// Format for the exported data. Valid values are `DYNAMODB_JSON` or `ION`. See the [AWS Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport.Output.html#S3DataExport.Output_Data) for more information on these export formats. Default is `DYNAMODB_JSON`.
        #[builder(into, default)]
        pub export_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time in RFC3339 format from which to export table data. The table export will be a snapshot of the table's state at this point in time. Omitting this value will result in a snapshot from the current time.
        #[builder(into, default)]
        pub export_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Amazon S3 bucket to export the snapshot to. See the [AWS Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport_Requesting.html#S3DataExport_Requesting_Permissions) for information on how configure this S3 bucket.
        #[builder(into)]
        pub s3_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the AWS account that owns the bucket the export will be stored in.
        #[builder(into, default)]
        pub s3_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon S3 bucket prefix to use as the file name and path of the exported snapshot.
        #[builder(into, default)]
        pub s3_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of encryption used on the bucket where export data will be stored. Valid values are: `AES256`, `KMS`.
        #[builder(into, default)]
        pub s3_sse_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the AWS KMS managed key used to encrypt the S3 bucket where export data will be stored (if applicable).
        #[builder(into, default)]
        pub s3_sse_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN associated with the table to export.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub table_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TableExportResult {
        /// ARN of the Table Export.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Billable size of the table export.
        pub billed_size_in_bytes: pulumi_gestalt_rust::Output<i32>,
        /// Time at which the export task completed.
        pub end_time: pulumi_gestalt_rust::Output<String>,
        /// Format for the exported data. Valid values are `DYNAMODB_JSON` or `ION`. See the [AWS Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport.Output.html#S3DataExport.Output_Data) for more information on these export formats. Default is `DYNAMODB_JSON`.
        pub export_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// Status of the export - export can be in one of the following states `IN_PROGRESS`, `COMPLETED`, or `FAILED`.
        pub export_status: pulumi_gestalt_rust::Output<String>,
        /// Time in RFC3339 format from which to export table data. The table export will be a snapshot of the table's state at this point in time. Omitting this value will result in a snapshot from the current time.
        pub export_time: pulumi_gestalt_rust::Output<String>,
        /// Number of items exported.
        pub item_count: pulumi_gestalt_rust::Output<i32>,
        /// Name of the manifest file for the export task. See the [AWS Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport.Output.html#S3DataExport.Output_Manifest) for more information on this manifest file.
        pub manifest_files_s3_key: pulumi_gestalt_rust::Output<String>,
        /// Name of the Amazon S3 bucket to export the snapshot to. See the [AWS Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/S3DataExport_Requesting.html#S3DataExport_Requesting_Permissions) for information on how configure this S3 bucket.
        pub s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the bucket the export will be stored in.
        pub s3_bucket_owner: pulumi_gestalt_rust::Output<String>,
        /// Amazon S3 bucket prefix to use as the file name and path of the exported snapshot.
        pub s3_prefix: pulumi_gestalt_rust::Output<String>,
        /// Type of encryption used on the bucket where export data will be stored. Valid values are: `AES256`, `KMS`.
        pub s3_sse_algorithm: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS KMS managed key used to encrypt the S3 bucket where export data will be stored (if applicable).
        pub s3_sse_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time at which the export task began.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// ARN associated with the table to export.
        ///
        /// The following arguments are optional:
        pub table_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableExportArgs,
    ) -> TableExportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let export_format_binding = args.export_format.get_output(context);
        let export_time_binding = args.export_time.get_output(context);
        let s3_bucket_binding = args.s3_bucket.get_output(context);
        let s3_bucket_owner_binding = args.s3_bucket_owner.get_output(context);
        let s3_prefix_binding = args.s3_prefix.get_output(context);
        let s3_sse_algorithm_binding = args.s3_sse_algorithm.get_output(context);
        let s3_sse_kms_key_id_binding = args.s3_sse_kms_key_id.get_output(context);
        let table_arn_binding = args.table_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/tableExport:TableExport".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportFormat".into(),
                    value: export_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportTime".into(),
                    value: export_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Bucket".into(),
                    value: s3_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3BucketOwner".into(),
                    value: s3_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Prefix".into(),
                    value: s3_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3SseAlgorithm".into(),
                    value: s3_sse_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3SseKmsKeyId".into(),
                    value: s3_sse_kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableArn".into(),
                    value: table_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableExportResult {
            arn: o.get_field("arn"),
            billed_size_in_bytes: o.get_field("billedSizeInBytes"),
            end_time: o.get_field("endTime"),
            export_format: o.get_field("exportFormat"),
            export_status: o.get_field("exportStatus"),
            export_time: o.get_field("exportTime"),
            item_count: o.get_field("itemCount"),
            manifest_files_s3_key: o.get_field("manifestFilesS3Key"),
            s3_bucket: o.get_field("s3Bucket"),
            s3_bucket_owner: o.get_field("s3BucketOwner"),
            s3_prefix: o.get_field("s3Prefix"),
            s3_sse_algorithm: o.get_field("s3SseAlgorithm"),
            s3_sse_kms_key_id: o.get_field("s3SseKmsKeyId"),
            start_time: o.get_field("startTime"),
            table_arn: o.get_field("tableArn"),
        }
    }
}
