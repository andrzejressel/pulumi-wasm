/// Provides an S3 bucket CORS configuration resource. For more information about CORS, go to [Enabling Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/userguide/cors.html) in the Amazon S3 User Guide.
///
/// > **NOTE:** S3 Buckets only support a single CORS configuration. Declaring multiple `aws.s3.BucketCorsConfigurationV2` resources to the same S3 Bucket will cause a perpetual difference in configuration.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("mybucket").build_struct(),
///     );
///     let exampleBucketCorsConfigurationV2 = bucket_cors_configuration_v_2::create(
///         "exampleBucketCorsConfigurationV2",
///         BucketCorsConfigurationV2Args::builder()
///             .bucket("${example.id}")
///             .cors_rules(
///                 vec![
///                     BucketCorsConfigurationV2CorsRule::builder()
///                     .allowedHeaders(vec!["*",]).allowedMethods(vec!["PUT", "POST",])
///                     .allowedOrigins(vec!["https://s3-website-test.domain.example",])
///                     .exposeHeaders(vec!["ETag",]).maxAgeSeconds(3000).build_struct(),
///                     BucketCorsConfigurationV2CorsRule::builder()
///                     .allowedMethods(vec!["GET",]).allowedOrigins(vec!["*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket CORS configuration using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketCorsConfigurationV2:BucketCorsConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketCorsConfigurationV2:BucketCorsConfigurationV2 example bucket-name,123456789012
/// ```
pub mod bucket_cors_configuration_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketCorsConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Set of origins and methods (cross-origin access that you want to allow). See below. You can configure up to 100 rules.
        #[builder(into)]
        pub cors_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketCorsConfigurationV2CorsRule>,
        >,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketCorsConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Set of origins and methods (cross-origin access that you want to allow). See below. You can configure up to 100 rules.
        pub cors_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketCorsConfigurationV2CorsRule>,
        >,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketCorsConfigurationV2Args,
    ) -> BucketCorsConfigurationV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let cors_rules_binding = args.cors_rules.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketCorsConfigurationV2:BucketCorsConfigurationV2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "corsRules".into(),
                    value: &cors_rules_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "corsRules".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketCorsConfigurationV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            cors_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsRules").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
        }
    }
}
