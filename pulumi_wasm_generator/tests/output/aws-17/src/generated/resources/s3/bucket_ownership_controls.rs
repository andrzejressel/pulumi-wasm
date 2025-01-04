/// Provides a resource to manage S3 Bucket Ownership Controls. For more information, see the [S3 Developer Guide](https://docs.aws.amazon.com/AmazonS3/latest/dev/about-object-ownership.html).
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
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleBucketOwnershipControls = bucket_ownership_controls::create(
///         "exampleBucketOwnershipControls",
///         BucketOwnershipControlsArgs::builder()
///             .bucket("${example.id}")
///             .rule(
///                 BucketOwnershipControlsRule::builder()
///                     .objectOwnership("BucketOwnerPreferred")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Bucket Ownership Controls using S3 Bucket name. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketOwnershipControls:BucketOwnershipControls example my-bucket
/// ```
pub mod bucket_ownership_controls {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketOwnershipControlsArgs {
        /// Name of the bucket that you want to associate this access point with.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with Ownership Controls rules. Detailed below.
        #[builder(into)]
        pub rule: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketOwnershipControlsRule,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketOwnershipControlsResult {
        /// Name of the bucket that you want to associate this access point with.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with Ownership Controls rules. Detailed below.
        pub rule: pulumi_wasm_rust::Output<
            super::super::types::s3::BucketOwnershipControlsRule,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketOwnershipControlsArgs,
    ) -> BucketOwnershipControlsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let rule_binding = args.rule.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketOwnershipControls:BucketOwnershipControls".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "rule".into(),
                    value: &rule_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "rule".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketOwnershipControlsResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rule").unwrap(),
            ),
        }
    }
}
