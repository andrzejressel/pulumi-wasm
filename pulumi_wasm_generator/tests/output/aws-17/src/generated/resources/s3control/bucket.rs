/// Provides a resource to manage an S3 Control Bucket.
///
/// > This functionality is for managing [S3 on Outposts](https://docs.aws.amazon.com/AmazonS3/latest/dev/S3onOutposts.html). To manage S3 Buckets in an AWS Partition, see the `aws.s3.BucketV2` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket::create(
///         "example",
///         BucketArgs::builder()
///             .bucket("example")
///             .outpost_id("${exampleAwsOutpostsOutpost.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Control Buckets using Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/bucket:Bucket example arn:aws:s3-outposts:us-east-1:123456789012:outpost/op-12345678/bucket/example
/// ```
pub mod bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketArgs {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Identifier of the Outpost to contain this bucket.
        #[builder(into)]
        pub outpost_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketResult {
        /// Amazon Resource Name (ARN) of the bucket.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// UTC creation date in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// Identifier of the Outpost to contain this bucket.
        pub outpost_id: pulumi_wasm_rust::Output<String>,
        /// Boolean whether Public Access Block is enabled.
        pub public_access_block_enabled: pulumi_wasm_rust::Output<bool>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketArgs) -> BucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let outpost_id_binding = args.outpost_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/bucket:Bucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "outpostId".into(),
                    value: &outpost_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "outpostId".into(),
                },
                register_interface::ResultField {
                    name: "publicAccessBlockEnabled".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            outpost_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostId").unwrap(),
            ),
            public_access_block_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicAccessBlockEnabled").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
