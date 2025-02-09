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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// The name of the DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `soa_record` block as defined below.
        #[builder(into, default)]
        pub soa_record: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::ZoneSoaRecord>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// (Optional) Maximum number of Records in the zone. Defaults to `1000`.
        pub max_number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        /// The name of the DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// (Optional) A list of values that make up the NS record for the zone.
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// (Optional) The number of records already in the zone.
        pub number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// An `soa_record` block as defined below.
        pub soa_record: pulumi_gestalt_rust::Output<
            super::super::types::dns::ZoneSoaRecord,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZoneArgs,
    ) -> ZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let soa_record_binding_1 = args.soa_record.get_output(context);
        let soa_record_binding = soa_record_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneResult {
            max_number_of_record_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxNumberOfRecordSets"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameServers"),
            ),
            number_of_record_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numberOfRecordSets"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            soa_record: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("soaRecord"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
