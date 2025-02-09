/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod a_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ARecordArgs {
        /// The name of the DNS A Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of IPv4 Addresses. Conflicts with `target_resource_id`.
        #[builder(into, default)]
        pub records: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** either `records` OR `target_resource_id` must be specified, but not both.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure resource id of the target object. Conflicts with `records`.
        #[builder(into, default)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Time To Live (TTL) of the DNS record in seconds.
        #[builder(into)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `zone_name` should be the name of resource `azure.dns.Zone` instead of `azure.privatedns.Zone`.
        #[builder(into)]
        pub zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ARecordResult {
        /// The FQDN of the DNS A Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the DNS A Record. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of IPv4 Addresses. Conflicts with `target_resource_id`.
        pub records: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **Note:** either `records` OR `target_resource_id` must be specified, but not both.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Azure resource id of the target object. Conflicts with `records`.
        pub target_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `zone_name` should be the name of resource `azure.dns.Zone` instead of `azure.privatedns.Zone`.
        pub zone_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ARecordArgs,
    ) -> ARecordResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let records_binding_1 = args.records.get_output(context);
        let records_binding = records_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let target_resource_id_binding_1 = args.target_resource_id.get_output(context);
        let target_resource_id_binding = target_resource_id_binding_1.get_inner();
        let ttl_binding_1 = args.ttl.get_output(context);
        let ttl_binding = ttl_binding_1.get_inner();
        let zone_name_binding_1 = args.zone_name.get_output(context);
        let zone_name_binding = zone_name_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ARecordResult {
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            records: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("records"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            zone_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneName"),
            ),
        }
    }
}
