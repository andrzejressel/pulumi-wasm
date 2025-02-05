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
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetLbipRangesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getLBIPRanges:getLBIPRanges".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLbipRangesResult {
            http_ssl_tcp_internals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpSslTcpInternals"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            networks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networks"),
            ),
        }
    }
}
