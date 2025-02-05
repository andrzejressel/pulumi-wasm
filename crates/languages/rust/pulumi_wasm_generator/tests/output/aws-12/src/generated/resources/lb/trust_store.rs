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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub ca_certificates_bundle_s3_object_version: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrustStoreArgs,
    ) -> TrustStoreResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ca_certificates_bundle_s3_bucket_binding = args
            .ca_certificates_bundle_s3_bucket
            .get_output(context)
            .get_inner();
        let ca_certificates_bundle_s3_key_binding = args
            .ca_certificates_bundle_s3_key
            .get_output(context)
            .get_inner();
        let ca_certificates_bundle_s3_object_version_binding = args
            .ca_certificates_bundle_s3_object_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TrustStoreResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            arn_suffix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arnSuffix"),
            ),
            ca_certificates_bundle_s3_bucket: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caCertificatesBundleS3Bucket"),
            ),
            ca_certificates_bundle_s3_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caCertificatesBundleS3Key"),
            ),
            ca_certificates_bundle_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caCertificatesBundleS3ObjectVersion"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
