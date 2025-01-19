pub mod get_origin_ca_root_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginCaRootCertificateArgs {
        /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
        #[builder(into)]
        pub algorithm: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginCaRootCertificateResult {
        /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
        pub algorithm: pulumi_wasm_rust::Output<String>,
        /// The Origin CA root certificate in PEM format.
        pub cert_pem: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetOriginCaRootCertificateArgs,
    ) -> GetOriginCaRootCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let algorithm_binding = args.algorithm.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaRootCertificate:getOriginCaRootCertificate"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "algorithm".into(),
                    value: &algorithm_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "algorithm".into(),
                },
                register_interface::ResultField {
                    name: "certPem".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginCaRootCertificateResult {
            algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("algorithm").unwrap(),
            ),
            cert_pem: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certPem").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
