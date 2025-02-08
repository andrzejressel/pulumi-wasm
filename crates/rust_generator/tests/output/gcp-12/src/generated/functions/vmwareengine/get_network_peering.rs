#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPeeringArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPeeringResult {
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub export_custom_routes: pulumi_gestalt_rust::Output<bool>,
        pub export_custom_routes_with_public_ip: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub import_custom_routes: pulumi_gestalt_rust::Output<bool>,
        pub import_custom_routes_with_public_ip: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub peer_network: pulumi_gestalt_rust::Output<String>,
        pub peer_network_type: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub state_details: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub vmware_engine_network: pulumi_gestalt_rust::Output<String>,
        pub vmware_engine_network_canonical: pulumi_gestalt_rust::Output<String>,
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
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getNetworkPeering:getNetworkPeering".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkPeeringResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            export_custom_routes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportCustomRoutes"),
            ),
            export_custom_routes_with_public_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exportCustomRoutesWithPublicIp"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            import_custom_routes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importCustomRoutes"),
            ),
            import_custom_routes_with_public_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importCustomRoutesWithPublicIp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            peer_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerNetwork"),
            ),
            peer_network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peerNetworkType"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vmware_engine_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareEngineNetwork"),
            ),
            vmware_engine_network_canonical: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareEngineNetworkCanonical"),
            ),
        }
    }
}
