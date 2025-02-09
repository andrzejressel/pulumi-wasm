#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dns_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDnsZoneArgs {
        /// The name of the Private DNS Zone.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the Private DNS Zone exists.
        /// If the Name of the Resource Group is not provided, the first Private DNS Zone from the list of Private
        /// DNS Zones in your subscription that matches `name` will be returned.
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags for the zone.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDnsZoneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Maximum number of recordsets that can be created in this Private Zone.
        pub max_number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        /// Maximum number of Virtual Networks that can be linked to this Private Zone.
        pub max_number_of_virtual_network_links: pulumi_gestalt_rust::Output<i32>,
        /// Maximum number of Virtual Networks that can be linked to this Private Zone with registration enabled.
        pub max_number_of_virtual_network_links_with_registration: pulumi_gestalt_rust::Output<
            i32,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of recordsets currently in the zone.
        pub number_of_record_sets: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags for the zone.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDnsZoneArgs,
    ) -> GetDnsZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatedns/getDnsZone:getDnsZone".into(),
            version: super::super::super::get_version(),
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
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDnsZoneResult {
            id: o.get_field("id"),
            max_number_of_record_sets: o.get_field("maxNumberOfRecordSets"),
            max_number_of_virtual_network_links: o
                .get_field("maxNumberOfVirtualNetworkLinks"),
            max_number_of_virtual_network_links_with_registration: o
                .get_field("maxNumberOfVirtualNetworkLinksWithRegistration"),
            name: o.get_field("name"),
            number_of_record_sets: o.get_field("numberOfRecordSets"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
