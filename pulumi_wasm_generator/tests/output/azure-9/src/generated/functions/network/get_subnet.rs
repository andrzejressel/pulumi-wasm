pub mod get_subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Specifies the name of the Subnet.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Virtual Network this Subnet is located within.
        #[builder(into)]
        pub virtual_network_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        pub address_prefix: pulumi_wasm_rust::Output<String>,
        /// The address prefixes for the subnet.
        pub address_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Is the default outbound access enabled for the subnet.
        pub default_outbound_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Security Group associated with the subnet.
        pub network_security_group_id: pulumi_wasm_rust::Output<String>,
        /// Enable or Disable network policies for the private endpoint on the subnet.
        pub private_endpoint_network_policies: pulumi_wasm_rust::Output<String>,
        /// Enable or Disable network policies for the private link service on the subnet.
        pub private_link_service_network_policies_enabled: pulumi_wasm_rust::Output<
            bool,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Route Table associated with this subnet.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// A list of Service Endpoints within this subnet.
        pub service_endpoints: pulumi_wasm_rust::Output<Vec<String>>,
        pub virtual_network_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubnetArgs,
    ) -> GetSubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let virtual_network_name_binding = args
            .virtual_network_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getSubnet:getSubnet".into(),
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
                register_interface::ObjectField {
                    name: "virtualNetworkName".into(),
                    value: &virtual_network_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "addressPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "defaultOutboundAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpointNetworkPolicies".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkServiceNetworkPoliciesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "serviceEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSubnetResult {
            address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressPrefix").unwrap(),
            ),
            address_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressPrefixes").unwrap(),
            ),
            default_outbound_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultOutboundAccessEnabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSecurityGroupId").unwrap(),
            ),
            private_endpoint_network_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpointNetworkPolicies").unwrap(),
            ),
            private_link_service_network_policies_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkServiceNetworkPoliciesEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            service_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceEndpoints").unwrap(),
            ),
            virtual_network_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkName").unwrap(),
            ),
        }
    }
}
