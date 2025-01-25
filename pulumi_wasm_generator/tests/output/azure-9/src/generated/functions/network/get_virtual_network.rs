pub mod get_virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkArgs {
        /// Specifies the name of the Virtual Network.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkResult {
        /// The list of address spaces used by the virtual network.
        pub address_spaces: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of DNS servers used by the virtual network.
        pub dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The GUID of the virtual network.
        pub guid: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Location of the virtual network.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The list of name of the subnets that are attached to this virtual network.
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A mapping of name - virtual network id of the virtual network peerings.
        pub vnet_peerings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of virtual network peerings IP addresses.
        pub vnet_peerings_addresses: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVirtualNetworkArgs,
    ) -> GetVirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualNetwork:getVirtualNetwork".into(),
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
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "guid".into(),
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
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vnetPeerings".into(),
                },
                register_interface::ResultField {
                    name: "vnetPeeringsAddresses".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualNetworkResult {
            address_spaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressSpaces").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guid").unwrap(),
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
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vnet_peerings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetPeerings").unwrap(),
            ),
            vnet_peerings_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetPeeringsAddresses").unwrap(),
            ),
        }
    }
}
