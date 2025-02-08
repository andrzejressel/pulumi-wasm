#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkPeeringArgs,
    ) -> GetNetworkPeeringResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetworkPeering:getNetworkPeering".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkPeeringResult {
            export_custom_routes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportCustomRoutes"),
            ),
            export_subnet_routes_with_public_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportSubnetRoutesWithPublicIp"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            import_custom_routes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importCustomRoutes"),
            ),
            import_subnet_routes_with_public_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importSubnetRoutesWithPublicIp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            peer_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerNetwork"),
            ),
            stack_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackType"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
        }
    }
}
