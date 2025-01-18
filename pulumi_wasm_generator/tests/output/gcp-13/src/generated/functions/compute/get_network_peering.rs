pub mod get_network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPeeringArgs {
        /// Name of the peering.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary network of the peering.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPeeringResult {
        pub export_custom_routes: pulumi_wasm_rust::Output<bool>,
        pub export_subnet_routes_with_public_ip: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub import_custom_routes: pulumi_wasm_rust::Output<bool>,
        pub import_subnet_routes_with_public_ip: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub peer_network: pulumi_wasm_rust::Output<String>,
        pub stack_type: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub state_details: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkPeeringArgs) -> GetNetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetworkPeering:getNetworkPeering".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "exportCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "exportSubnetRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "importCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "importSubnetRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "peerNetwork".into(),
                },
                register_interface::ResultField {
                    name: "stackType".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateDetails".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkPeeringResult {
            export_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportCustomRoutes").unwrap(),
            ),
            export_subnet_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportSubnetRoutesWithPublicIp").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            import_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importCustomRoutes").unwrap(),
            ),
            import_subnet_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importSubnetRoutesWithPublicIp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            peer_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerNetwork").unwrap(),
            ),
            stack_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackType").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateDetails").unwrap(),
            ),
        }
    }
}
