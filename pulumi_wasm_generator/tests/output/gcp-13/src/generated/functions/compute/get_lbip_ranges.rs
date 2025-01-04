pub mod get_lbip_ranges {
    #[allow(dead_code)]
    pub struct GetLbipRangesResult {
        /// The IP ranges used for health checks when **HTTP(S), SSL proxy, TCP proxy, and Internal load balancing** is used
        pub http_ssl_tcp_internals: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The IP ranges used for health checks when **Network load balancing** is used
        pub networks: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetLbipRangesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getLBIPRanges:getLBIPRanges".into(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "httpSslTcpInternals".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLbipRangesResult {
            http_ssl_tcp_internals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpSslTcpInternals").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
        }
    }
}
