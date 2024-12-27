pub mod get_origin_ca_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginCaCertificateArgs {
        /// The Origin CA Certificate unique identifier.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetOriginCaCertificateArgs) -> GetOriginCaCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaCertificate:getOriginCaCertificate"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "hostnames".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "requestType".into(),
                },
                register_interface::ResultField {
                    name: "revokedAt".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginCaCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            hostnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnames").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            request_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestType").unwrap(),
            ),
            revoked_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revokedAt").unwrap(),
            ),
        }
    }
}
