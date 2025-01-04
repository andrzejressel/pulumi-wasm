pub mod get_zone_virtual_network_link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneVirtualNetworkLinkArgs {
        /// The name of the Private DNS Zone Virtual Network Link.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Private DNS zone (without a terminating dot).
        #[builder(into)]
        pub private_dns_zone_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource group where the Private DNS Zone exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetZoneVirtualNetworkLinkResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_dns_zone_name: pulumi_wasm_rust::Output<String>,
        /// Whether the auto-registration of virtual machine records in the virtual network in the Private DNS zone is enabled or not.
        pub registration_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the Virtual Network that is linked to the DNS Zone.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetZoneVirtualNetworkLinkArgs,
    ) -> GetZoneVirtualNetworkLinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let private_dns_zone_name_binding = args.private_dns_zone_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getZoneVirtualNetworkLink:getZoneVirtualNetworkLink"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsZoneName".into(),
                    value: &private_dns_zone_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsZoneName".into(),
                },
                register_interface::ResultField {
                    name: "registrationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetZoneVirtualNetworkLinkResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_dns_zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsZoneName").unwrap(),
            ),
            registration_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrationEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
        }
    }
}
