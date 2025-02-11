/// Provides an S3 bucket Object Lock configuration resource. For more information about Object Locking, go to [Using S3 Object Lock](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html) in the Amazon S3 User Guide.
///
/// > This resource can be used enable Object Lock for **new** and **existing** buckets.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Object Lock configuration for new or existing buckets
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("mybucket").build_struct(),
///     );
///     let exampleBucketObjectLockConfigurationV2 = bucket_object_lock_configuration_v_2::create(
///         "exampleBucketObjectLockConfigurationV2",
///         BucketObjectLockConfigurationV2Args::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketObjectLockConfigurationV2Rule::builder()
///                     .defaultRetention(
///                         BucketObjectLockConfigurationV2RuleDefaultRetention::builder()
///                             .days(5)
///                             .mode("COMPLIANCE")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleBucketVersioningV2 = bucket_versioning_v_2::create(
///         "exampleBucketVersioningV2",
///         BucketVersioningV2Args::builder()
///             .bucket("${example.id}")
///             .versioning_configuration(
///                 BucketVersioningV2VersioningConfiguration::builder()
///                     .status("Enabled")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner`, separated by a comma (`,`). For example:
///
/// __Using `pulumi import`__, import an S3 bucket Object Lock Configuration using one of two forms. If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2 example bucket-name,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_object_lock_configuration_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketObjectLockConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Defaults to `Enabled`. Valid values: `Enabled`.
        #[builder(into, default)]
        pub object_lock_enabled: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for specifying the Object Lock rule for the specified object. See below.
        #[builder(into, default)]
        pub rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketObjectLockConfigurationV2Rule>,
        >,
        /// This argument is deprecated and no longer needed to enable Object Lock.
        /// To enable Object Lock for an existing bucket, you must first enable versioning on the bucket and then enable Object Lock. For more details on versioning, see the `aws.s3.BucketVersioningV2` resource.
        #[builder(into, default)]
        pub token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketObjectLockConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether this bucket has an Object Lock configuration enabled. Defaults to `Enabled`. Valid values: `Enabled`.
        pub object_lock_enabled: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for specifying the Object Lock rule for the specified object. See below.
        pub rule: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketObjectLockConfigurationV2Rule>,
        >,
        /// This argument is deprecated and no longer needed to enable Object Lock.
        /// To enable Object Lock for an existing bucket, you must first enable versioning on the bucket and then enable Object Lock. For more details on versioning, see the `aws.s3.BucketVersioningV2` resource.
        pub token: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketObjectLockConfigurationV2Args,
    ) -> BucketObjectLockConfigurationV2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let object_lock_enabled_binding = args.object_lock_enabled.get_output(context);
        let rule_binding = args.rule.get_output(context);
        let token_binding = args.token.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketObjectLockConfigurationV2:BucketObjectLockConfigurationV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectLockEnabled".into(),
                    value: &object_lock_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rule".into(),
                    value: &rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "token".into(),
                    value: &token_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketObjectLockConfigurationV2Result {
            bucket: o.get_field("bucket"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            object_lock_enabled: o.get_field("objectLockEnabled"),
            rule: o.get_field("rule"),
            token: o.get_field("token"),
        }
    }
}
