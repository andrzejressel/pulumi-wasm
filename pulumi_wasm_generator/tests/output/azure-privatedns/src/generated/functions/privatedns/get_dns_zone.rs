pub mod get_dns_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDnsZoneArgs {
        /// The name of the Private DNS Zone.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the Private DNS Zone exists.
        /// If the Name of the Resource Group is not provided, the first Private DNS Zone from the list of Private
        /// DNS Zones in your subscription that matches `name` will be returned.
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags for the zone.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDnsZoneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Maximum number of recordsets that can be created in this Private Zone.
        pub max_number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        /// Maximum number of Virtual Networks that can be linked to this Private Zone.
        pub max_number_of_virtual_network_links: pulumi_wasm_rust::Output<i32>,
        /// Maximum number of Virtual Networks that can be linked to this Private Zone with registration enabled.
        pub max_number_of_virtual_network_links_with_registration: pulumi_wasm_rust::Output<
            i32,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of recordsets currently in the zone.
        pub number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags for the zone.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDnsZoneArgs) -> GetDnsZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getDnsZone:getDnsZone".into(),
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
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
        GetDnsZoneResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
