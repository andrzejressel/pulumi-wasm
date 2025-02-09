/// Provides an S3 bucket CORS configuration resource. For more information about CORS, go to [Enabling Cross-Origin Resource Sharing](https://docs.aws.amazon.com/AmazonS3/latest/userguide/cors.html) in the Amazon S3 User Guide.
///
/// > **NOTE:** S3 Buckets only support a single CORS configuration. Declaring multiple `aws.s3.BucketCorsConfigurationV2` resources to the same S3 Bucket will cause a perpetual difference in configuration.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_cors_configuration_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketCorsConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of origins and methods (cross-origin access that you want to allow). See below. You can configure up to 100 rules.
        #[builder(into)]
        pub cors_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::s3::BucketCorsConfigurationV2CorsRule>,
        >,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketCorsConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Set of origins and methods (cross-origin access that you want to allow). See below. You can configure up to 100 rules.
        pub cors_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3::BucketCorsConfigurationV2CorsRule>,
        >,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketCorsConfigurationV2Args,
    ) -> BucketCorsConfigurationV2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let cors_rules_binding = args.cors_rules.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketCorsConfigurationV2:BucketCorsConfigurationV2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "corsRules".into(),
                    value: cors_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: expected_bucket_owner_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketCorsConfigurationV2Result {
            bucket: o.get_field("bucket"),
            cors_rules: o.get_field("corsRules"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
        }
    }
}
