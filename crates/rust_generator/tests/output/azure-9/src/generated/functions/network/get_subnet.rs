#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subnet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Specifies the name of the Subnet.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Virtual Network is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Virtual Network this Subnet is located within.
        #[builder(into)]
        pub virtual_network_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        pub address_prefix: pulumi_gestalt_rust::Output<String>,
        /// The address prefixes for the subnet.
        pub address_prefixes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Is the default outbound access enabled for the subnet.
        pub default_outbound_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Network Security Group associated with the subnet.
        pub network_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// Enable or Disable network policies for the private endpoint on the subnet.
        pub private_endpoint_network_policies: pulumi_gestalt_rust::Output<String>,
        /// Enable or Disable network policies for the private link service on the subnet.
        pub private_link_service_network_policies_enabled: pulumi_gestalt_rust::Output<
            bool,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Route Table associated with this subnet.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// A list of Service Endpoints within this subnet.
        pub service_endpoints: pulumi_gestalt_rust::Output<Vec<String>>,
        pub virtual_network_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubnetArgs,
    ) -> GetSubnetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let virtual_network_name_binding = args.virtual_network_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getSubnet:getSubnet".into(),
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
                    name: "virtualNetworkName".into(),
                    value: virtual_network_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubnetResult {
            address_prefix: o.get_field("addressPrefix"),
            address_prefixes: o.get_field("addressPrefixes"),
            default_outbound_access_enabled: o.get_field("defaultOutboundAccessEnabled"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            network_security_group_id: o.get_field("networkSecurityGroupId"),
            private_endpoint_network_policies: o
                .get_field("privateEndpointNetworkPolicies"),
            private_link_service_network_policies_enabled: o
                .get_field("privateLinkServiceNetworkPoliciesEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            route_table_id: o.get_field("routeTableId"),
            service_endpoints: o.get_field("serviceEndpoints"),
            virtual_network_name: o.get_field("virtualNetworkName"),
        }
    }
}
