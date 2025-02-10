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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub ca_certificates_bundle_s3_object_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustStoreResult {
        /// ARN of the Trust Store (matches `id`).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN suffix for use with CloudWatch Metrics.
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        /// S3 Bucket name holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_key: pulumi_gestalt_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        pub ca_certificates_bundle_s3_object_version: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustStoreArgs,
    ) -> TrustStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ca_certificates_bundle_s3_bucket_binding = args
            .ca_certificates_bundle_s3_bucket
            .get_output(context);
        let ca_certificates_bundle_s3_key_binding = args
            .ca_certificates_bundle_s3_key
            .get_output(context);
        let ca_certificates_bundle_s3_object_version_binding = args
            .ca_certificates_bundle_s3_object_version
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lb/trustStore:TrustStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3Bucket".into(),
                    value: ca_certificates_bundle_s3_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3Key".into(),
                    value: ca_certificates_bundle_s3_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3ObjectVersion".into(),
                    value: ca_certificates_bundle_s3_object_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustStoreResult {
            arn: o.get_field("arn"),
            arn_suffix: o.get_field("arnSuffix"),
            ca_certificates_bundle_s3_bucket: o
                .get_field("caCertificatesBundleS3Bucket"),
            ca_certificates_bundle_s3_key: o.get_field("caCertificatesBundleS3Key"),
            ca_certificates_bundle_s3_object_version: o
                .get_field("caCertificatesBundleS3ObjectVersion"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
