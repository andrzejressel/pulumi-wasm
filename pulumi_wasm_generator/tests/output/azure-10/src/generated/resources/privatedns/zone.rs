/// Enables you to manage Private DNS zones within Azure DNS. These zones are hosted on Azure's name servers.
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
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("mydomain.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Private DNS Zones can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/zone:Zone zone1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/privateDnsZones/zone1
/// ```
///
pub mod zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// The name of the Private DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If you are going to be using the Private DNS Zone with a Private Endpoint the name of the Private DNS Zone must follow the **Private DNS Zone name** schema in the [product documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-dns#virtual-network-and-on-premises-workloads-using-a-dns-forwarder) in order for the two resources to be connected successfully.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// An `soa_record` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub soa_record: pulumi_wasm_rust::Output<
            Option<super::super::types::privatedns::ZoneSoaRecord>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// The maximum number of record sets that can be created in this Private DNS zone.
        pub max_number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of virtual networks that can be linked to this Private DNS zone.
        pub max_number_of_virtual_network_links: pulumi_wasm_rust::Output<i32>,
        /// The maximum number of virtual networks that can be linked to this Private DNS zone with registration enabled.
        pub max_number_of_virtual_network_links_with_registration: pulumi_wasm_rust::Output<
            i32,
        >,
        /// The name of the Private DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If you are going to be using the Private DNS Zone with a Private Endpoint the name of the Private DNS Zone must follow the **Private DNS Zone name** schema in the [product documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-dns#virtual-network-and-on-premises-workloads-using-a-dns-forwarder) in order for the two resources to be connected successfully.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The current number of record sets in this Private DNS zone.
        pub number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// An `soa_record` block as defined below. Changing this forces a new resource to be created.
        pub soa_record: pulumi_wasm_rust::Output<
            super::super::types::privatedns::ZoneSoaRecord,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ZoneArgs) -> ZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let soa_record_binding = args.soa_record.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/zone:Zone".into(),
            name: name.to_string(),
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
                    name: "soaRecord".into(),
                    value: &soa_record_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "maxNumberOfRecordSets".into(),
                },
                register_interface::ResultField {
                    name: "maxNumberOfVirtualNetworkLinks".into(),
                },
                register_interface::ResultField {
                    name: "maxNumberOfVirtualNetworkLinksWithRegistration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "numberOfRecordSets".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "soaRecord".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZoneResult {
            max_number_of_record_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxNumberOfRecordSets").unwrap(),
            ),
            max_number_of_virtual_network_links: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxNumberOfVirtualNetworkLinks").unwrap(),
            ),
            max_number_of_virtual_network_links_with_registration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxNumberOfVirtualNetworkLinksWithRegistration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number_of_record_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfRecordSets").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            soa_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("soaRecord").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
