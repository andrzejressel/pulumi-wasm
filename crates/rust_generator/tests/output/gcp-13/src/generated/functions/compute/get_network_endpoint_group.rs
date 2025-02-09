#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkEndpointGroupArgs {
        /// The Network Endpoint Group name.
        /// Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project to list versions in.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group self_link.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group availability zone.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkEndpointGroupResult {
        /// The NEG default port.
        pub default_port: pulumi_gestalt_rust::Output<i32>,
        /// The NEG description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The network to which all network endpoints in the NEG belong.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        pub network_endpoint_type: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of network endpoints in the network endpoint group.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// subnetwork to which all network endpoints in the NEG belong.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkEndpointGroupArgs,
    ) -> GetNetworkEndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getNetworkEndpointGroup:getNetworkEndpointGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: self_link_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkEndpointGroupResult {
            default_port: o.get_field("defaultPort"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_endpoint_type: o.get_field("networkEndpointType"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            size: o.get_field("size"),
            subnetwork: o.get_field("subnetwork"),
            zone: o.get_field("zone"),
        }
    }
}
