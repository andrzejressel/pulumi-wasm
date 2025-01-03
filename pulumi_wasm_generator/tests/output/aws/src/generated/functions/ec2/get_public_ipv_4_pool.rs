pub mod get_public_ipv_4_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIpv4PoolArgs {
        /// AWS resource IDs of a public IPv4 pool (as a string) for which this data source will fetch detailed information.
        #[builder(into)]
        pub pool_id: pulumi_wasm_rust::Output<String>,
        /// Any tags for the address pool.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPublicIpv4PoolResult {
        /// Description of the pool, if any.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the location from which the address pool is advertised.
        /// * pool_address_ranges` - List of Address Ranges in the Pool; each address range record contains:
        pub network_border_group: pulumi_wasm_rust::Output<String>,
        pub pool_address_ranges: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetPublicIpv4PoolPoolAddressRange>,
        >,
        pub pool_id: pulumi_wasm_rust::Output<String>,
        /// Any tags for the address pool.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Total number of addresses in the pool.
        pub total_address_count: pulumi_wasm_rust::Output<i32>,
        /// Total number of available addresses in the pool.
        pub total_available_address_count: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPublicIpv4PoolArgs) -> GetPublicIpv4PoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pool_id_binding = args.pool_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getPublicIpv4Pool:getPublicIpv4Pool".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "poolId".into(),
                    value: &pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "networkBorderGroup".into(),
                },
                register_interface::ResultField {
                    name: "poolAddressRanges".into(),
                },
                register_interface::ResultField {
                    name: "poolId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "totalAddressCount".into(),
                },
                register_interface::ResultField {
                    name: "totalAvailableAddressCount".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPublicIpv4PoolResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            network_border_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkBorderGroup").unwrap(),
            ),
            pool_address_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolAddressRanges").unwrap(),
            ),
            pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            total_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalAddressCount").unwrap(),
            ),
            total_available_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalAvailableAddressCount").unwrap(),
            ),
        }
    }
}
