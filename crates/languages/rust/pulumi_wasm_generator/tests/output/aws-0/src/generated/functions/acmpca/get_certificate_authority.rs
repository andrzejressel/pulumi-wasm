pub mod get_certificate_authority {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateAuthorityArgs {
        /// ARN of the certificate authority.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCertificateAuthorityResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Base64-encoded certificate authority (CA) certificate. Only available after the certificate authority certificate has been imported.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// Base64-encoded certificate chain that includes any intermediate certificates and chains up to root on-premises certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. Only available after the certificate authority certificate has been imported.
        pub certificate_chain: pulumi_wasm_rust::Output<String>,
        /// The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.
        pub certificate_signing_request: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_storage_security_standard: pulumi_wasm_rust::Output<String>,
        /// Date and time after which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_after: pulumi_wasm_rust::Output<String>,
        /// Date and time before which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_before: pulumi_wasm_rust::Output<String>,
        /// Nested attribute containing revocation configuration.
        pub revocation_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::acmpca::GetCertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Serial number of the certificate authority. Only available after the certificate authority certificate has been imported.
        pub serial: pulumi_wasm_rust::Output<String>,
        /// Status of the certificate authority.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the certificate authority.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly.
        pub usage_mode: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCertificateAuthorityArgs,
    ) -> GetCertificateAuthorityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:acmpca/getCertificateAuthority:getCertificateAuthority".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateAuthorityResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateChain"),
            ),
            certificate_signing_request: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateSigningRequest"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_storage_security_standard: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyStorageSecurityStandard"),
            ),
            not_after: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notAfter"),
            ),
            not_before: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notBefore"),
            ),
            revocation_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revocationConfigurations"),
            ),
            serial: pulumi_wasm_rust::__private::into_domain(o.extract_field("serial")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            usage_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usageMode"),
            ),
        }
    }
}
