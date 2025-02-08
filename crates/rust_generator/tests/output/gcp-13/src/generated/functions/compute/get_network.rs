#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkArgs,
    ) -> GetNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let network_profile_binding = args
            .network_profile
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetwork:getNetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            gateway_ipv4: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayIpv4"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            internal_ipv6_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalIpv6Range"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkProfile"),
            ),
            numeric_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numericId"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            subnetworks_self_links: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetworksSelfLinks"),
            ),
        }
    }
}
