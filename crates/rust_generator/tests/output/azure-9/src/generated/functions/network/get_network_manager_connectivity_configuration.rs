#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_manager_connectivity_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkManagerConnectivityConfigurationArgs {
        /// The name of this Network Manager Connectivity Configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Network Manager.
        #[builder(into)]
        pub network_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkManagerConnectivityConfigurationResult {
        /// An `applies_to_group` block as defined below.
        pub applies_to_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetNetworkManagerConnectivityConfigurationAppliesToGroup,
            >,
        >,
        /// The connectivity topology type.
        pub connectivity_topology: pulumi_gestalt_rust::Output<String>,
        /// Whether to current existing Virtual Network Peering in the Connectivity Configuration affected scope.
        pub delete_existing_peering_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The description of the Connectivity Configuration.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether global mesh is supported.
        pub global_mesh_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A `hub` block as defined below.
        pub hubs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetNetworkManagerConnectivityConfigurationHub,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_manager_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkManagerConnectivityConfigurationArgs,
    ) -> GetNetworkManagerConnectivityConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let network_manager_id_binding = args.network_manager_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getNetworkManagerConnectivityConfiguration:getNetworkManagerConnectivityConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkManagerId".into(),
                    value: network_manager_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkManagerConnectivityConfigurationResult {
            applies_to_groups: o.get_field("appliesToGroups"),
            connectivity_topology: o.get_field("connectivityTopology"),
            delete_existing_peering_enabled: o.get_field("deleteExistingPeeringEnabled"),
            description: o.get_field("description"),
            global_mesh_enabled: o.get_field("globalMeshEnabled"),
            hubs: o.get_field("hubs"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            network_manager_id: o.get_field("networkManagerId"),
        }
    }
}
