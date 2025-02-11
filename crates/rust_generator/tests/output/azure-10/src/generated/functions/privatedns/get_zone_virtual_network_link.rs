#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zone_virtual_network_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneVirtualNetworkLinkArgs {
        /// The name of the Private DNS Zone Virtual Network Link.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Private DNS zone (without a terminating dot).
        #[builder(into)]
        pub private_dns_zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the resource group where the Private DNS Zone exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetZoneVirtualNetworkLinkResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_dns_zone_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the auto-registration of virtual machine records in the virtual network in the Private DNS zone is enabled or not.
        pub registration_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the Virtual Network that is linked to the DNS Zone.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZoneVirtualNetworkLinkArgs,
    ) -> GetZoneVirtualNetworkLinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let private_dns_zone_name_binding = args
            .private_dns_zone_name
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatedns/getZoneVirtualNetworkLink:getZoneVirtualNetworkLink"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsZoneName".into(),
                    value: &private_dns_zone_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZoneVirtualNetworkLinkResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            private_dns_zone_name: o.get_field("privateDnsZoneName"),
            registration_enabled: o.get_field("registrationEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
