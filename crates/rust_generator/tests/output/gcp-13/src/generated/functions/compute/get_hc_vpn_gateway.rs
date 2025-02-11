#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hc_vpn_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHcVpnGatewayArgs {
        /// The name of the forwarding rule.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the project region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHcVpnGatewayResult {
        pub description: pulumi_gestalt_rust::Output<String>,
        pub gateway_ip_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub stack_type: pulumi_gestalt_rust::Output<String>,
        pub vpn_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetHcVpnGatewayVpnInterface>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHcVpnGatewayArgs,
    ) -> GetHcVpnGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getHcVpnGateway:getHcVpnGateway".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetHcVpnGatewayResult {
            description: o.get_field("description"),
            gateway_ip_version: o.get_field("gatewayIpVersion"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            stack_type: o.get_field("stackType"),
            vpn_interfaces: o.get_field("vpnInterfaces"),
        }
    }
}
