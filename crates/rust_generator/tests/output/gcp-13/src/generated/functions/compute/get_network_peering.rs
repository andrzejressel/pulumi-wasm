#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPeeringArgs {
        /// Name of the peering.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The primary network of the peering.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPeeringResult {
        pub export_custom_routes: pulumi_gestalt_rust::Output<bool>,
        pub export_subnet_routes_with_public_ip: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub import_custom_routes: pulumi_gestalt_rust::Output<bool>,
        pub import_subnet_routes_with_public_ip: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub peer_network: pulumi_gestalt_rust::Output<String>,
        pub stack_type: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub state_details: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkPeeringArgs,
    ) -> GetNetworkPeeringResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getNetworkPeering:getNetworkPeering".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkPeeringResult {
            export_custom_routes: o.get_field("exportCustomRoutes"),
            export_subnet_routes_with_public_ip: o
                .get_field("exportSubnetRoutesWithPublicIp"),
            id: o.get_field("id"),
            import_custom_routes: o.get_field("importCustomRoutes"),
            import_subnet_routes_with_public_ip: o
                .get_field("importSubnetRoutesWithPublicIp"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            peer_network: o.get_field("peerNetwork"),
            stack_type: o.get_field("stackType"),
            state: o.get_field("state"),
            state_details: o.get_field("stateDetails"),
        }
    }
}
