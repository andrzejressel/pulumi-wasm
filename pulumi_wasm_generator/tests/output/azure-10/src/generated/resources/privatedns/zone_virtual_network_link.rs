/// Enables you to manage Private DNS zone Virtual Network Links. These Links enable DNS resolution and registration inside Azure Virtual Networks using Azure Private DNS.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("test-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("mydomain.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZoneVirtualNetworkLink = zone_virtual_network_link::create(
///         "exampleZoneVirtualNetworkLink",
///         ZoneVirtualNetworkLinkArgs::builder()
///             .name("test")
///             .private_dns_zone_name("${exampleZone.name}")
///             .resource_group_name("${example.name}")
///             .virtual_network_id("${exampleVirtualNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Private DNS Zone Virtual Network Links can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/zoneVirtualNetworkLink:ZoneVirtualNetworkLink link1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/privateDnsZones/zone1.com/virtualNetworkLinks/myVnetLink1
/// ```
///
pub mod zone_virtual_network_link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneVirtualNetworkLinkArgs {
        /// The name of the Private DNS Zone Virtual Network Link. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Private DNS zone (without a terminating dot). Changing this forces a new resource to be created.
        #[builder(into)]
        pub private_dns_zone_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Is auto-registration of virtual machine records in the virtual network in the Private DNS zone enabled? Defaults to `false`.
        #[builder(into, default)]
        pub registration_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the resource group where the Private DNS Zone exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneVirtualNetworkLinkResult {
        /// The name of the Private DNS Zone Virtual Network Link. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Private DNS zone (without a terminating dot). Changing this forces a new resource to be created.
        pub private_dns_zone_name: pulumi_wasm_rust::Output<String>,
        /// Is auto-registration of virtual machine records in the virtual network in the Private DNS zone enabled? Defaults to `false`.
        pub registration_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the resource group where the Private DNS Zone exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZoneVirtualNetworkLinkArgs,
    ) -> ZoneVirtualNetworkLinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let private_dns_zone_name_binding = args
            .private_dns_zone_name
            .get_output(context)
            .get_inner();
        let registration_enabled_binding = args
            .registration_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_network_id_binding = args
            .virtual_network_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/zoneVirtualNetworkLink:ZoneVirtualNetworkLink"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "registrationEnabled".into(),
                    value: &registration_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
            results: Vec::from([
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZoneVirtualNetworkLinkResult {
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
