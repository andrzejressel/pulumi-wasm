#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_circuit_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCircuitPeeringArgs {
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        #[builder(into)]
        pub peering_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCircuitPeeringResult {
        /// The ASN used by Azure for the peering.
        pub azure_asn: pulumi_gestalt_rust::Output<i32>,
        pub express_route_circuit_name: pulumi_gestalt_rust::Output<String>,
        pub gateway_manager_etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Indicates if IPv4 is enabled.
        pub ipv4_enabled: pulumi_gestalt_rust::Output<bool>,
        pub peer_asn: pulumi_gestalt_rust::Output<i32>,
        /// The type of the ExpressRoute Circuit Peering.
        pub peering_type: pulumi_gestalt_rust::Output<String>,
        /// The primary port used by Azure for this peering.
        pub primary_azure_port: pulumi_gestalt_rust::Output<String>,
        /// The primary peer address prefix.
        pub primary_peer_address_prefix: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub route_filter_id: pulumi_gestalt_rust::Output<String>,
        /// The secondary port used by Azure for this peering.
        pub secondary_azure_port: pulumi_gestalt_rust::Output<String>,
        /// The secondary peer address prefix.
        pub secondary_peer_address_prefix: pulumi_gestalt_rust::Output<String>,
        pub shared_key: pulumi_gestalt_rust::Output<String>,
        /// The VLAN ID used for this peering.
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCircuitPeeringArgs,
    ) -> GetCircuitPeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_output(context);
        let peering_type_binding = args.peering_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:expressroute/getCircuitPeering:getCircuitPeering".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: &express_route_circuit_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringType".into(),
                    value: &peering_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCircuitPeeringResult {
            azure_asn: o.get_field("azureAsn"),
            express_route_circuit_name: o.get_field("expressRouteCircuitName"),
            gateway_manager_etag: o.get_field("gatewayManagerEtag"),
            id: o.get_field("id"),
            ipv4_enabled: o.get_field("ipv4Enabled"),
            peer_asn: o.get_field("peerAsn"),
            peering_type: o.get_field("peeringType"),
            primary_azure_port: o.get_field("primaryAzurePort"),
            primary_peer_address_prefix: o.get_field("primaryPeerAddressPrefix"),
            resource_group_name: o.get_field("resourceGroupName"),
            route_filter_id: o.get_field("routeFilterId"),
            secondary_azure_port: o.get_field("secondaryAzurePort"),
            secondary_peer_address_prefix: o.get_field("secondaryPeerAddressPrefix"),
            shared_key: o.get_field("sharedKey"),
            vlan_id: o.get_field("vlanId"),
        }
    }
}
