/// Provides a ELBv2 Trust Store for use with Application Load Balancer Listener resources.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Target Groups using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lb/trustStore:TrustStore example arn:aws:elasticloadbalancing:us-west-2:187416307283:truststore/my-trust-store/20cfe21448b66314
/// ```
pub mod trust_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_bucket: pulumi_wasm_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_key: pulumi_wasm_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub ca_certificates_bundle_s3_object_version: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustStoreResult {
        /// ARN of the Trust Store (matches `id`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN suffix for use with CloudWatch Metrics.
        pub arn_suffix: pulumi_wasm_rust::Output<String>,
        /// S3 Bucket name holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_bucket: pulumi_wasm_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_key: pulumi_wasm_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        pub ca_certificates_bundle_s3_object_version: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: TrustStoreArgs) -> TrustStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ca_certificates_bundle_s3_bucket_binding = args
            .ca_certificates_bundle_s3_bucket
            .get_inner();
        let ca_certificates_bundle_s3_key_binding = args
            .ca_certificates_bundle_s3_key
            .get_inner();
        let ca_certificates_bundle_s3_object_version_binding = args
            .ca_certificates_bundle_s3_object_version
            .get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lb/trustStore:TrustStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "caCertificatesBundleS3Bucket".into(),
                    value: &ca_certificates_bundle_s3_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "caCertificatesBundleS3Key".into(),
                    value: &ca_certificates_bundle_s3_key_binding,
                },
                register_interface::ObjectField {
                    name: "caCertificatesBundleS3ObjectVersion".into(),
                    value: &ca_certificates_bundle_s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
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
                    name: "arnSuffix".into(),
                },
                register_interface::ResultField {
                    name: "caCertificatesBundleS3Bucket".into(),
                },
                register_interface::ResultField {
                    name: "caCertificatesBundleS3Key".into(),
                },
                register_interface::ResultField {
                    name: "caCertificatesBundleS3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
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
        TrustStoreResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            arn_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arnSuffix").unwrap(),
            ),
            ca_certificates_bundle_s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertificatesBundleS3Bucket").unwrap(),
            ),
            ca_certificates_bundle_s3_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertificatesBundleS3Key").unwrap(),
            ),
            ca_certificates_bundle_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertificatesBundleS3ObjectVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
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
