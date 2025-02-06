pub mod get_origin_ca_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginCaCertificateArgs {
        /// The Origin CA Certificate unique identifier.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginCaCertificateResult {
        /// The Origin CA certificate.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the certificate will expire.
        pub expires_on: pulumi_wasm_rust::Output<String>,
        /// A list of hostnames or wildcard names bound to the certificate.
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Origin CA Certificate unique identifier.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`
        pub request_type: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the certificate was revoked.
        pub revoked_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOriginCaCertificateArgs,
    ) -> GetOriginCaCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaCertificate:getOriginCaCertificate"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOriginCaCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiresOn"),
            ),
            hostnames: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostnames"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            request_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requestType"),
            ),
            revoked_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revokedAt"),
            ),
        }
    }
}
