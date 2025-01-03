pub mod get_network_endpoint_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkEndpointGroupArgs {
        /// The Network Endpoint Group name.
        /// Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project to list versions in.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Network Endpoint Group self_link.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        /// The Network Endpoint Group availability zone.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetNetworkEndpointGroupArgs) -> GetNetworkEndpointGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let self_link_binding = args.self_link.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetworkEndpointGroup:getNetworkEndpointGroup".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultPort".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkEndpointGroupResult {
            default_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPort").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkEndpointType").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
