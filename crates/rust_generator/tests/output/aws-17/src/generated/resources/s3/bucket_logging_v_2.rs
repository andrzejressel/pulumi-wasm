/// Provides an S3 bucket (server access) logging resource. For more information, see [Logging requests using server access logging](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerLogs.html)
/// in the AWS S3 User Guide.
///
/// > **Note:** Amazon S3 supports server access logging, AWS CloudTrail, or a combination of both. Refer to the [Logging options for Amazon S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/logging-with-S3.html)
/// to decide which method meets your requirements.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("my-tf-example-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder().acl("private").bucket("${example.id}").build_struct(),
///     );
///     let exampleBucketLoggingV2 = bucket_logging_v_2::create(
///         "exampleBucketLoggingV2",
///         BucketLoggingV2Args::builder()
///             .bucket("${example.id}")
///             .target_bucket("${logBucket.id}")
///             .target_prefix("log/")
///             .build_struct(),
///     );
///     let logBucket = bucket_v_2::create(
///         "logBucket",
///         BucketV2Args::builder().bucket("my-tf-log-bucket").build_struct(),
///     );
///     let logBucketAcl = bucket_acl_v_2::create(
///         "logBucketAcl",
///         BucketAclV2Args::builder()
///             .acl("log-delivery-write")
///             .bucket("${logBucket.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket logging using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketLoggingV2:BucketLoggingV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketLoggingV2:BucketLoggingV2 example bucket-name,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_logging_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketLoggingV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the bucket where you want Amazon S3 to store server access logs.
        #[builder(into)]
        pub target_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of configuration blocks with information for granting permissions. See below.
        #[builder(into, default)]
        pub target_grants: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketLoggingV2TargetGrant>>,
        >,
        /// Amazon S3 key format for log objects. See below.
        #[builder(into, default)]
        pub target_object_key_format: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormat>,
        >,
        /// Prefix for all log object keys.
        #[builder(into)]
        pub target_prefix: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketLoggingV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the bucket where you want Amazon S3 to store server access logs.
        pub target_bucket: pulumi_gestalt_rust::Output<String>,
        /// Set of configuration blocks with information for granting permissions. See below.
        pub target_grants: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::s3::BucketLoggingV2TargetGrant>>,
        >,
        /// Amazon S3 key format for log objects. See below.
        pub target_object_key_format: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormat>,
        >,
        /// Prefix for all log object keys.
        pub target_prefix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketLoggingV2Args,
    ) -> BucketLoggingV2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let target_bucket_binding = args.target_bucket.get_output(context);
        let target_grants_binding = args.target_grants.get_output(context);
        let target_object_key_format_binding = args
            .target_object_key_format
            .get_output(context);
        let target_prefix_binding = args.target_prefix.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketLoggingV2:BucketLoggingV2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: expected_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetBucket".into(),
                    value: target_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetGrants".into(),
                    value: target_grants_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetObjectKeyFormat".into(),
                    value: target_object_key_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetPrefix".into(),
                    value: target_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketLoggingV2Result {
            bucket: o.get_field("bucket"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            target_bucket: o.get_field("targetBucket"),
            target_grants: o.get_field("targetGrants"),
            target_object_key_format: o.get_field("targetObjectKeyFormat"),
            target_prefix: o.get_field("targetPrefix"),
        }
    }
}
