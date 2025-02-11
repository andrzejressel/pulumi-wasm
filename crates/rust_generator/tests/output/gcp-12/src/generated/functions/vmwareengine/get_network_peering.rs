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
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkPeeringArgs,
    ) -> GetNetworkPeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getNetworkPeering:getNetworkPeering".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkPeeringResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            export_custom_routes: o.get_field("exportCustomRoutes"),
            export_custom_routes_with_public_ip: o
                .get_field("exportCustomRoutesWithPublicIp"),
            id: o.get_field("id"),
            import_custom_routes: o.get_field("importCustomRoutes"),
            import_custom_routes_with_public_ip: o
                .get_field("importCustomRoutesWithPublicIp"),
            name: o.get_field("name"),
            peer_network: o.get_field("peerNetwork"),
            peer_network_type: o.get_field("peerNetworkType"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            state_details: o.get_field("stateDetails"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            vmware_engine_network: o.get_field("vmwareEngineNetwork"),
            vmware_engine_network_canonical: o.get_field("vmwareEngineNetworkCanonical"),
        }
    }
}
