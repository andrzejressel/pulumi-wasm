/// Enables you to manage DNS zones within Azure DNS. These zones are hosted on Azure's name servers to which you can delegate the zone from the parent domain.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   example-public:
///     type: azure:dns:Zone
///     properties:
///       name: mydomain.com
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// DNS Zones can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dns/zone:Zone zone1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/dnsZones/zone1
/// ```
///
pub mod zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// The name of the DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// An `soa_record` block as defined below.
        #[builder(into, default)]
        pub soa_record: pulumi_wasm_rust::Output<
            Option<super::super::types::dns::ZoneSoaRecord>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// (Optional) Maximum number of Records in the zone. Defaults to `1000`.
        pub max_number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        /// The name of the DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// (Optional) A list of values that make up the NS record for the zone.
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// (Optional) The number of records already in the zone.
        pub number_of_record_sets: pulumi_wasm_rust::Output<i32>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// An `soa_record` block as defined below.
        pub soa_record: pulumi_wasm_rust::Output<
            super::super::types::dns::ZoneSoaRecord,
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
            type_: "azure:dns/zone:Zone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
            soa_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("soaRecord").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
