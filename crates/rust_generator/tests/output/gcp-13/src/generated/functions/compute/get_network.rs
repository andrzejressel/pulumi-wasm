#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkArgs {
        /// The name of the network.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Beta A full or partial URL of the network profile to apply to this network.
        #[builder(into, default)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkResult {
        /// Description of this network.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the gateway.
        pub gateway_ipv4: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ula internal ipv6 range assigned to this network.
        pub internal_ipv6_range: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Beta A full or partial URL of the network profile to apply to this network.
        pub network_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The numeric unique identifier for the resource.
        pub numeric_id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// the list of subnetworks which belong to the network
        pub subnetworks_self_links: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkArgs,
    ) -> GetNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let network_profile_binding = args.network_profile.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getNetwork:getNetwork".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkProfile".into(),
                    value: network_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkResult {
            description: o.get_field("description"),
            gateway_ipv4: o.get_field("gatewayIpv4"),
            id: o.get_field("id"),
            internal_ipv6_range: o.get_field("internalIpv6Range"),
            name: o.get_field("name"),
            network_profile: o.get_field("networkProfile"),
            numeric_id: o.get_field("numericId"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            subnetworks_self_links: o.get_field("subnetworksSelfLinks"),
        }
    }
}
