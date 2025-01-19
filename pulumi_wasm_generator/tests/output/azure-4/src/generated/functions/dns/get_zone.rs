pub mod get_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneArgs {
        /// The name of the DNS Zone.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the DNS Zone exists.
        /// If the Name of the Resource Group is not provided, the first DNS Zone from the list of DNS Zones
        /// in your subscription that matches `name` will be returned.
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZoneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Maximum number of Records in the zone.
        pub max_number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of values that make up the NS record for the zone.
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of records already in the zone.
        pub number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the DNS Zone.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetZoneArgs) -> GetZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:dns/getZone:getZone".into(),
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "maxNumberOfRecordSets".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameServers".into(),
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
        GetZoneResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            max_number_of_record_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxNumberOfRecordSets").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameServers").unwrap(),
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
