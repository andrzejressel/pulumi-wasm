/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleZone:
///     type: azure:dns:Zone
///     name: example
///     properties:
///       name: mydomain.com
///       resourceGroupName: ${example.name}
///   exampleTxtRecord:
///     type: azure:dns:TxtRecord
///     name: example
///     properties:
///       name: test
///       zoneName: ${exampleZone.name}
///       resourceGroupName: ${example.name}
///       ttl: 300
///       records:
///         - value: google-site-authenticator
///         - value: more site information here
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// TXT records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dns/txtRecord:TxtRecord example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/dnsZones/zone1/TXT/myrecord1
/// ```
///
pub mod txt_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TxtRecordArgs {
        /// The name of the DNS TXT Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of values that make up the txt record. Each `record` block supports fields documented below.
        #[builder(into)]
        pub records: pulumi_wasm_rust::Output<
            Vec<super::super::types::dns::TxtRecordRecord>,
        >,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds.
        #[builder(into)]
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TxtRecordResult {
        /// The FQDN of the DNS TXT Record.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The name of the DNS TXT Record. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of values that make up the txt record. Each `record` block supports fields documented below.
        pub records: pulumi_wasm_rust::Output<
            Vec<super::super::types::dns::TxtRecordRecord>,
        >,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TxtRecordArgs) -> TxtRecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let records_binding = args.records.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let ttl_binding = args.ttl.get_inner();
        let zone_name_binding = args.zone_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dns/txtRecord:TxtRecord".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "records".into(),
                    value: &records_binding,
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
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "zoneName".into(),
                    value: &zone_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "records".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "zoneName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TxtRecordResult {
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            records: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("records").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneName").unwrap(),
            ),
        }
    }
}
