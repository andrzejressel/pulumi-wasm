/// Manages S3 bucket-level Public Access Block configuration. For more information about these settings, see the [AWS S3 Block Public Access documentation](https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html).
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
///     let exampleBucketPublicAccessBlock = bucket_public_access_block::create(
///         "exampleBucketPublicAccessBlock",
///         BucketPublicAccessBlockArgs::builder()
///             .block_public_acls(true)
///             .block_public_policy(true)
///             .bucket("${example.id}")
///             .ignore_public_acls(true)
///             .restrict_public_buckets(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_s3_bucket_public_access_block` using the bucket name. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketPublicAccessBlock:BucketPublicAccessBlock example my-bucket
/// ```
pub mod bucket_public_access_block {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketPublicAccessBlockArgs {
        /// Whether Amazon S3 should block public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls will fail if the request includes an object ACL.
        #[builder(into, default)]
        pub block_public_acls: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the existing bucket policy. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        #[builder(into, default)]
        pub block_public_policy: pulumi_wasm_rust::Output<Option<bool>>,
        /// S3 Bucket to which this Public Access Block configuration should be applied.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Whether Amazon S3 should ignore public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore public ACLs on this bucket and any objects that it contains.
        #[builder(into, default)]
        pub ignore_public_acls: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the previously stored bucket policy, except that public and cross-account access within the public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access this buckets if it has a public policy.
        #[builder(into, default)]
        pub restrict_public_buckets: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct BucketPublicAccessBlockResult {
        /// Whether Amazon S3 should block public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls will fail if the request includes an object ACL.
        pub block_public_acls: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the existing bucket policy. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        pub block_public_policy: pulumi_wasm_rust::Output<Option<bool>>,
        /// S3 Bucket to which this Public Access Block configuration should be applied.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Whether Amazon S3 should ignore public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore public ACLs on this bucket and any objects that it contains.
        pub ignore_public_acls: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the previously stored bucket policy, except that public and cross-account access within the public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access this buckets if it has a public policy.
        pub restrict_public_buckets: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketPublicAccessBlockArgs,
    ) -> BucketPublicAccessBlockResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let block_public_acls_binding = args.block_public_acls.get_inner();
        let block_public_policy_binding = args.block_public_policy.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let ignore_public_acls_binding = args.ignore_public_acls.get_inner();
        let restrict_public_buckets_binding = args.restrict_public_buckets.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketPublicAccessBlock:BucketPublicAccessBlock".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockPublicAcls".into(),
                    value: &block_public_acls_binding,
                },
                register_interface::ObjectField {
                    name: "blockPublicPolicy".into(),
                    value: &block_public_policy_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "ignorePublicAcls".into(),
                    value: &ignore_public_acls_binding,
                },
                register_interface::ObjectField {
                    name: "restrictPublicBuckets".into(),
                    value: &restrict_public_buckets_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blockPublicAcls".into(),
                },
                register_interface::ResultField {
                    name: "blockPublicPolicy".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "ignorePublicAcls".into(),
                },
                register_interface::ResultField {
                    name: "restrictPublicBuckets".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketPublicAccessBlockResult {
            block_public_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPublicAcls").unwrap(),
            ),
            block_public_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockPublicPolicy").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            ignore_public_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignorePublicAcls").unwrap(),
            ),
            restrict_public_buckets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restrictPublicBuckets").unwrap(),
            ),
        }
    }
}