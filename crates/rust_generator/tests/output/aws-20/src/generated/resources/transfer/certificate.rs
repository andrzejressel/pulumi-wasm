/// Provides a AWS Transfer AS2 Certificate resource.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Transfer AS2 Certificate using the `certificate_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/certificate:Certificate example c-4221a88afd5f4362a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The valid certificate file required for the transfer.
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The optional list of certificate that make up the chain for the certificate that is being imported.
        #[builder(into, default)]
        pub certificate_chain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short description that helps identify the certificate.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private key associated with the certificate being imported.
        #[builder(into, default)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if a certificate is being used for signing or encryption. The valid values are SIGNING and ENCRYPTION.
        #[builder(into)]
        pub usage: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// An date when the certificate becomes active
        pub active_date: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the certificate
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The valid certificate file required for the transfer.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The optional list of certificate that make up the chain for the certificate that is being imported.
        pub certificate_chain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the AS2 certificate
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// A short description that helps identify the certificate.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An date when the certificate becomes inactive
        pub inactive_date: pulumi_gestalt_rust::Output<String>,
        /// The private key associated with the certificate being imported.
        pub private_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies if a certificate is being used for signing or encryption. The valid values are SIGNING and ENCRYPTION.
        pub usage: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let certificate_chain_binding = args.certificate_chain.get_output(context);
        let description_binding = args.description.get_output(context);
        let private_key_binding = args.private_key.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let usage_binding = args.usage.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateChain".into(),
                    value: certificate_chain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKey".into(),
                    value: private_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usage".into(),
                    value: usage_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            active_date: o.get_field("activeDate"),
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_chain: o.get_field("certificateChain"),
            certificate_id: o.get_field("certificateId"),
            description: o.get_field("description"),
            inactive_date: o.get_field("inactiveDate"),
            private_key: o.get_field("privateKey"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            usage: o.get_field("usage"),
        }
    }
}
