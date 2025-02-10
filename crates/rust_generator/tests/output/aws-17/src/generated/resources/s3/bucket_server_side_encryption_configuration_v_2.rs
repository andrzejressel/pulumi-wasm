/// Provides a S3 bucket server-side encryption configuration resource.
///
/// > **NOTE:** Destroying an `aws.s3.BucketServerSideEncryptionConfigurationV2` resource resets the bucket to [Amazon S3 bucket default encryption](https://docs.aws.amazon.com/AmazonS3/latest/userguide/default-encryption-faq.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_server_side_encryption_configuration_v_2::create(
///         "example",
///         BucketServerSideEncryptionConfigurationV2Args::builder()
///             .bucket("${mybucket.id}")
///             .rules(
///                 vec![
///                     BucketServerSideEncryptionConfigurationV2Rule::builder()
///                     .applyServerSideEncryptionByDefault(BucketServerSideEncryptionConfigurationV2RuleApplyServerSideEncryptionByDefault::builder()
///                     .kmsMasterKeyId("${mykey.arn}").sseAlgorithm("aws:kms")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let mybucket = bucket_v_2::create(
///         "mybucket",
///         BucketV2Args::builder().bucket("mybucket").build_struct(),
///     );
///     let mykey = key::create(
///         "mykey",
///         KeyArgs::builder()
///             .deletion_window_in_days(10)
///             .description("This key is used to encrypt bucket objects")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket server-side encryption configuration using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2 example bucket-name,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_server_side_encryption_configuration_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketServerSideEncryptionConfigurationV2Args {
        /// ID (name) of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of server-side encryption configuration rules. See below. Currently, only a single rule is supported.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::s3::BucketServerSideEncryptionConfigurationV2Rule>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketServerSideEncryptionConfigurationV2Result {
        /// ID (name) of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of server-side encryption configuration rules. See below. Currently, only a single rule is supported.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3::BucketServerSideEncryptionConfigurationV2Rule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketServerSideEncryptionConfigurationV2Args,
    ) -> BucketServerSideEncryptionConfigurationV2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketServerSideEncryptionConfigurationV2:BucketServerSideEncryptionConfigurationV2"
                .into(),
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
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketServerSideEncryptionConfigurationV2Result {
            bucket: o.get_field("bucket"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            rules: o.get_field("rules"),
        }
    }
}
