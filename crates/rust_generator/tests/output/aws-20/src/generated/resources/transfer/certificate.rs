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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_binding_1 = args.certificate.get_output(context);
        let certificate_binding = certificate_binding_1.get_inner();
        let certificate_chain_binding_1 = args.certificate_chain.get_output(context);
        let certificate_chain_binding = certificate_chain_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let private_key_binding_1 = args.private_key.get_output(context);
        let private_key_binding = private_key_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let usage_binding_1 = args.usage.get_output(context);
        let usage_binding = usage_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "certificateChain".into(),
                    value: &certificate_chain_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "usage".into(),
                    value: &usage_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            active_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activeDate"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            certificate_chain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateChain"),
            ),
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            inactive_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inactiveDate"),
            ),
            private_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateKey"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            usage: pulumi_gestalt_rust::__private::into_domain(o.extract_field("usage")),
        }
    }
}
