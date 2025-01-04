/// Manages the association between a Network Interface and a Application Gateway's Backend Address Pool.
///
/// ## Import
///
/// Associations between Network Interfaces and Application Gateway Backend Address Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkInterfaceApplicationGatewayBackendAddressPoolAssociation:NetworkInterfaceApplicationGatewayBackendAddressPoolAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkInterfaces/nic1/ipConfigurations/example|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/applicationGateways/gateway1/backendAddressPools/pool1
/// ```
///
pub mod network_interface_application_gateway_backend_address_pool_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationArgs {
        /// The ID of the Application Gateway's Backend Address Pool which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ip_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
        /// The ID of the Application Gateway's Backend Address Pool which this Network Interface which should be connected to. Changing this forces a new resource to be created.
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the IP Configuration within the Network Interface which should be connected to the Backend Address Pool. Changing this forces a new resource to be created.
        pub ip_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Interface. Changing this forces a new resource to be created.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationArgs,
    ) -> NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_address_pool_id_binding = args.backend_address_pool_id.get_inner();
        let ip_configuration_name_binding = args.ip_configuration_name.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkInterfaceApplicationGatewayBackendAddressPoolAssociation:NetworkInterfaceApplicationGatewayBackendAddressPoolAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendAddressPoolId".into(),
                    value: &backend_address_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurationName".into(),
                    value: &ip_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendAddressPoolId".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInterfaceApplicationGatewayBackendAddressPoolAssociationResult {
            backend_address_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPoolId").unwrap(),
            ),
            ip_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurationName").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
        }
    }
}
