/// Enables you to manage Private DNS zones within Azure DNS. These zones are hosted on Azure's name servers.
///
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// The name of the Private DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If you are going to be using the Private DNS Zone with a Private Endpoint the name of the Private DNS Zone must follow the **Private DNS Zone name** schema in the [product documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-dns#virtual-network-and-on-premises-workloads-using-a-dns-forwarder) in order for the two resources to be connected successfully.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `soa_record` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub soa_record: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::privatedns::ZoneSoaRecord>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// The maximum number of record sets that can be created in this Private DNS zone.
        pub max_number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of virtual networks that can be linked to this Private DNS zone.
        pub max_number_of_virtual_network_links: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of virtual networks that can be linked to this Private DNS zone with registration enabled.
        pub max_number_of_virtual_network_links_with_registration: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// The name of the Private DNS Zone. Must be a valid domain name. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** If you are going to be using the Private DNS Zone with a Private Endpoint the name of the Private DNS Zone must follow the **Private DNS Zone name** schema in the [product documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-dns#virtual-network-and-on-premises-workloads-using-a-dns-forwarder) in order for the two resources to be connected successfully.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The current number of record sets in this Private DNS zone.
        pub number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// An `soa_record` block as defined below. Changing this forces a new resource to be created.
        pub soa_record: pulumi_gestalt_rust::Output<
            super::super::types::privatedns::ZoneSoaRecord,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneArgs,
    ) -> ZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let soa_record_binding = args.soa_record.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:privatedns/zone:Zone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "soaRecord".into(),
                    value: soa_record_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneResult {
            max_number_of_record_sets: o.get_field("maxNumberOfRecordSets"),
            max_number_of_virtual_network_links: o
                .get_field("maxNumberOfVirtualNetworkLinks"),
            max_number_of_virtual_network_links_with_registration: o
                .get_field("maxNumberOfVirtualNetworkLinksWithRegistration"),
            name: o.get_field("name"),
            number_of_record_sets: o.get_field("numberOfRecordSets"),
            resource_group_name: o.get_field("resourceGroupName"),
            soa_record: o.get_field("soaRecord"),
            tags: o.get_field("tags"),
        }
    }
}
