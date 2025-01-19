/// Provides a resource to manage an S3 Control Bucket Lifecycle Configuration.
///
/// > **NOTE:** Each S3 Control Bucket can only have one Lifecycle Configuration. Using multiple of this resource against the same S3 Control Bucket will result in perpetual differences each provider run.
///
/// > This functionality is for managing [S3 on Outposts](https://docs.aws.amazon.com/AmazonS3/latest/dev/S3onOutposts.html). To manage S3 Bucket Lifecycle Configurations in an AWS Partition, see the `aws.s3.BucketV2` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod bucket_lifecycle_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationArgs {
        /// Amazon Resource Name (ARN) of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) containing lifecycle rules for the bucket.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3control::BucketLifecycleConfigurationRule>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketLifecycleConfigurationResult {
        /// Amazon Resource Name (ARN) of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) containing lifecycle rules for the bucket.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3control::BucketLifecycleConfigurationRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketLifecycleConfigurationArgs,
    ) -> BucketLifecycleConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let rules_binding = args.rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/bucketLifecycleConfiguration:BucketLifecycleConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketLifecycleConfigurationResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}
