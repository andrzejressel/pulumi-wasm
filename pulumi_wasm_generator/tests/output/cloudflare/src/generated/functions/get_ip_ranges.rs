pub mod get_ip_ranges {
    #[allow(dead_code)]
    pub struct GetIpRangesResult {
        /// The lexically ordered list of only the IPv4 China CIDR blocks.
        pub china_ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The lexically ordered list of only the IPv6 China CIDR blocks.
        pub china_ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The lexically ordered list of all non-China CIDR blocks.
        pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The lexically ordered list of only the IPv4 CIDR blocks.
        pub ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The lexically ordered list of only the IPv6 CIDR blocks.
        pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetIpRangesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getIpRanges:getIpRanges".into(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "chinaIpv4CidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "chinaIpv6CidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv4CidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlocks".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIpRangesResult {
            china_ipv4_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chinaIpv4CidrBlocks").unwrap(),
            ),
            china_ipv6_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chinaIpv6CidrBlocks").unwrap(),
            ),
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocks").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv4_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4CidrBlocks").unwrap(),
            ),
            ipv6_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlocks").unwrap(),
            ),
        }
    }
}
