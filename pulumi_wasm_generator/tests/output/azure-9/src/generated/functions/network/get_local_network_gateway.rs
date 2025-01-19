pub mod get_local_network_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalNetworkGatewayArgs {
        /// The name of the Local Network Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Local Network Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocalNetworkGatewayResult {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        pub address_spaces: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        pub bgp_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetLocalNetworkGatewayBgpSetting>,
        >,
        /// The gateway IP address the Local Network Gateway uses.
        pub gateway_address: pulumi_wasm_rust::Output<String>,
        /// The gateway FQDN the Local Network Gateway uses.
        pub gateway_fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Local Network Gateway exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Local Network Gateway.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLocalNetworkGatewayArgs) -> GetLocalNetworkGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getLocalNetworkGateway:getLocalNetworkGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressSpaces".into(),
                },
                register_interface::ResultField {
                    name: "bgpSettings".into(),
                },
                register_interface::ResultField {
                    name: "gatewayAddress".into(),
                },
                register_interface::ResultField {
                    name: "gatewayFqdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLocalNetworkGatewayResult {
            address_spaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressSpaces").unwrap(),
            ),
            bgp_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpSettings").unwrap(),
            ),
            gateway_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayAddress").unwrap(),
            ),
            gateway_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayFqdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
