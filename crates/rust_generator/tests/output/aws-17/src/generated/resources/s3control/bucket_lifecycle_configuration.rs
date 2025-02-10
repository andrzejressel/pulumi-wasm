/// Provides a resource to manage an S3 Control Bucket Lifecycle Configuration.
///
/// > **NOTE:** Each S3 Control Bucket can only have one Lifecycle Configuration. Using multiple of this resource against the same S3 Control Bucket will result in perpetual differences each provider run.
///
/// > This functionality is for managing [S3 on Outposts](https://docs.aws.amazon.com/AmazonS3/latest/dev/S3onOutposts.html). To manage S3 Bucket Lifecycle Configurations in an AWS Partition, see the `aws.s3.BucketV2` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_lifecycle_configuration::create(
///         "example",
///         BucketLifecycleConfigurationArgs::builder()
///             .bucket("${exampleAwsS3controlBucket.arn}")
///             .rules(
///                 vec![
///                     BucketLifecycleConfigurationRule::builder()
///                     .expiration(BucketLifecycleConfigurationRuleExpiration::builder()
///                     .days(365).build_struct())
///                     .filter(BucketLifecycleConfigurationRuleFilter::builder()
///                     .prefix("logs/").build_struct()).id("logs").build_struct(),
///                     BucketLifecycleConfigurationRule::builder()
///                     .expiration(BucketLifecycleConfigurationRuleExpiration::builder()
///                     .days(7).build_struct())
///                     .filter(BucketLifecycleConfigurationRuleFilter::builder()
///                     .prefix("temp/").build_struct()).id("temp").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Control Bucket Lifecycle Configurations using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/bucketLifecycleConfiguration:BucketLifecycleConfiguration example arn:aws:s3-outposts:us-east-1:123456789012:outpost/op-12345678/bucket/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_lifecycle_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationArgs {
        /// Amazon Resource Name (ARN) of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block(s) containing lifecycle rules for the bucket.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::s3control::BucketLifecycleConfigurationRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationResult {
        /// Amazon Resource Name (ARN) of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Configuration block(s) containing lifecycle rules for the bucket.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3control::BucketLifecycleConfigurationRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketLifecycleConfigurationArgs,
    ) -> BucketLifecycleConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/bucketLifecycleConfiguration:BucketLifecycleConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketLifecycleConfigurationResult {
            bucket: o.get_field("bucket"),
            rules: o.get_field("rules"),
        }
    }
}
