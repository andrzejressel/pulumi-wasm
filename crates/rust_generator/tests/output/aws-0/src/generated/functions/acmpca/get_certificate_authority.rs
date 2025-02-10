#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate_authority {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateAuthorityArgs {
        /// ARN of the certificate authority.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCertificateAuthorityResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Base64-encoded certificate authority (CA) certificate. Only available after the certificate authority certificate has been imported.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// Base64-encoded certificate chain that includes any intermediate certificates and chains up to root on-premises certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. Only available after the certificate authority certificate has been imported.
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        /// The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.
        pub certificate_signing_request: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_storage_security_standard: pulumi_gestalt_rust::Output<String>,
        /// Date and time after which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_after: pulumi_gestalt_rust::Output<String>,
        /// Date and time before which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_before: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing revocation configuration.
        pub revocation_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::acmpca::GetCertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Serial number of the certificate authority. Only available after the certificate authority certificate has been imported.
        pub serial: pulumi_gestalt_rust::Output<String>,
        /// Status of the certificate authority.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the certificate authority.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly.
        pub usage_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateAuthorityArgs,
    ) -> GetCertificateAuthorityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:acmpca/getCertificateAuthority:getCertificateAuthority".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateAuthorityResult {
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_chain: o.get_field("certificateChain"),
            certificate_signing_request: o.get_field("certificateSigningRequest"),
            id: o.get_field("id"),
            key_storage_security_standard: o.get_field("keyStorageSecurityStandard"),
            not_after: o.get_field("notAfter"),
            not_before: o.get_field("notBefore"),
            revocation_configurations: o.get_field("revocationConfigurations"),
            serial: o.get_field("serial"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            usage_mode: o.get_field("usageMode"),
        }
    }
}
