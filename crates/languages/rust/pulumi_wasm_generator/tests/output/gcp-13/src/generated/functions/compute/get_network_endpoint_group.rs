pub mod get_network_endpoint_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkEndpointGroupArgs {
        /// The Network Endpoint Group name.
        /// Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project to list versions in.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group self_link.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group availability zone.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkEndpointGroupResult {
        /// The NEG default port.
        pub default_port: pulumi_wasm_rust::Output<i32>,
        /// The NEG description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network to which all network endpoints in the NEG belong.
        pub network: pulumi_wasm_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        pub network_endpoint_type: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of network endpoints in the network endpoint group.
        pub size: pulumi_wasm_rust::Output<i32>,
        /// subnetwork to which all network endpoints in the NEG belong.
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkEndpointGroupArgs,
    ) -> GetNetworkEndpointGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let self_link_binding = args.self_link.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetworkEndpointGroup:getNetworkEndpointGroup".into(),
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
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkEndpointGroupResult {
            default_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPort"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkEndpointType"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            size: pulumi_wasm_rust::__private::into_domain(o.extract_field("size")),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
