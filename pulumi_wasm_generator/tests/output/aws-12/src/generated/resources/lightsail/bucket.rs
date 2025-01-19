/// Provides a lightsail bucket.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = bucket::create(
///         "test",
///         BucketArgs::builder().bundle_id("small_1_0").name("mytestbucket").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket` using the `name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucket:Bucket test example-bucket
/// ```
pub mod bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketArgs {
        /// The ID of the bundle to use for the bucket. A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a bucket. Use the [get-bucket-bundles](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-bucket-bundles.html) cli command to get a list of bundle IDs that you can specify.
        #[builder(into)]
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// Force Delete non-empty buckets using `pulumi destroy`. AWS by default will not delete an s3 bucket which is not empty, to prevent losing bucket data and affecting other resources in lightsail. If `force_delete` is set to `true` the bucket will be deleted even when not empty.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name for the bucket.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketResult {
        /// The ARN of the lightsail bucket.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The resource Availability Zone. Follows the format us-east-2a (case-sensitive).
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The ID of the bundle to use for the bucket. A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a bucket. Use the [get-bucket-bundles](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-bucket-bundles.html) cli command to get a list of bundle IDs that you can specify.
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the bucket was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Force Delete non-empty buckets using `pulumi destroy`. AWS by default will not delete an s3 bucket which is not empty, to prevent losing bucket data and affecting other resources in lightsail. If `force_delete` is set to `true` the bucket will be deleted even when not empty.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name for the bucket.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Web Services Region name.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The support code for the resource. Include this code in your email to support when you have questions about a resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BucketArgs) -> BucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_id_binding = args.bundle_id.get_inner();
        let force_delete_binding = args.force_delete.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/bucket:Bucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "forceDelete".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
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
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDelete").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
