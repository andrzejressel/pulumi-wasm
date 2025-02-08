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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetZoneVirtualNetworkLinkArgs,
    ) -> GetZoneVirtualNetworkLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let private_dns_zone_name_binding = args
            .private_dns_zone_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getZoneVirtualNetworkLink:getZoneVirtualNetworkLink"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsZoneName".into(),
                    value: &private_dns_zone_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZoneVirtualNetworkLinkResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_dns_zone_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsZoneName"),
            ),
            registration_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registrationEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}
