pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// ARN of the certificate issued by the private certificate authority.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the certificate authority.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// PEM-encoded certificate value.
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub certificate_authority_arn: pulumi_wasm_rust::Output<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA.
        pub certificate_chain: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:acmpca/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: &certificate_authority_arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            certificate_authority_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateAuthorityArn"),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateChain"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
