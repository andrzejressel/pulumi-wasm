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
///     let exampleARecord = a_record::create(
///         "exampleARecord",
///         ARecordArgs::builder()
///             .name("test")
///             .records(vec!["10.0.180.17",])
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("${exampleZone.name}")
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
///
/// ### Alias Record)
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
///     let exampleARecord = a_record::create(
///         "exampleARecord",
///         ARecordArgs::builder()
///             .name("test")
///             .resource_group_name("${example.name}")
///             .target_resource_id("${examplePublicIp.id}")
///             .ttl(300)
///             .zone_name("${exampleZone.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Dynamic")
///             .ip_version("IPv4")
///             .location("${example.location}")
///             .name("mypublicip")
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
/// }
/// ```
///
/// ## Import
///
/// A records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dns/aRecord:ARecord example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/dnsZones/zone1/A/myrecord1
/// ```
///
pub mod a_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ARecordArgs {
        /// The name of the DNS A Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of IPv4 Addresses. Conflicts with `target_resource_id`.
        #[builder(into, default)]
        pub records: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** either `records` OR `target_resource_id` must be specified, but not both.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure resource id of the target object. Conflicts with `records`.
        #[builder(into, default)]
        pub target_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Time To Live (TTL) of the DNS record in seconds.
        #[builder(into)]
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `zone_name` should be the name of resource `azure.dns.Zone` instead of `azure.privatedns.Zone`.
        #[builder(into)]
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ARecordResult {
        /// The FQDN of the DNS A Record.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The name of the DNS A Record. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of IPv4 Addresses. Conflicts with `target_resource_id`.
        pub records: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** either `records` OR `target_resource_id` must be specified, but not both.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure resource id of the target object. Conflicts with `records`.
        pub target_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `zone_name` should be the name of resource `azure.dns.Zone` instead of `azure.privatedns.Zone`.
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ARecordArgs) -> ARecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let records_binding = args.records.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let ttl_binding = args.ttl.get_inner();
        let zone_name_binding = args.zone_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dns/aRecord:ARecord".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
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
                    name: "targetResourceId".into(),
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
        ARecordResult {
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
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
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
