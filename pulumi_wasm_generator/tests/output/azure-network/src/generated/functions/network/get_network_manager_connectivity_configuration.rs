pub mod get_network_manager_connectivity_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkManagerConnectivityConfigurationArgs {
        /// The name of this Network Manager Connectivity Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Manager.
        #[builder(into)]
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkManagerConnectivityConfigurationResult {
        /// An `applies_to_group` block as defined below.
        pub applies_to_groups: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetNetworkManagerConnectivityConfigurationAppliesToGroup,
            >,
        >,
        /// The connectivity topology type.
        pub connectivity_topology: pulumi_wasm_rust::Output<String>,
        /// Whether to current existing Virtual Network Peering in the Connectivity Configuration affected scope.
        pub delete_existing_peering_enabled: pulumi_wasm_rust::Output<bool>,
        /// The description of the Connectivity Configuration.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether global mesh is supported.
        pub global_mesh_enabled: pulumi_wasm_rust::Output<bool>,
        /// A `hub` block as defined below.
        pub hubs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetNetworkManagerConnectivityConfigurationHub,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_manager_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetNetworkManagerConnectivityConfigurationArgs,
    ) -> GetNetworkManagerConnectivityConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let network_manager_id_binding = args.network_manager_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getNetworkManagerConnectivityConfiguration:getNetworkManagerConnectivityConfiguration"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkManagerId".into(),
                    value: &network_manager_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appliesToGroups".into(),
                },
                register_interface::ResultField {
                    name: "connectivityTopology".into(),
                },
                register_interface::ResultField {
                    name: "deleteExistingPeeringEnabled".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "globalMeshEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hubs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkManagerId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkManagerConnectivityConfigurationResult {
            applies_to_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appliesToGroups").unwrap(),
            ),
            connectivity_topology: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectivityTopology").unwrap(),
            ),
            delete_existing_peering_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteExistingPeeringEnabled").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            global_mesh_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalMeshEnabled").unwrap(),
            ),
            hubs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkManagerId").unwrap(),
            ),
        }
    }
}
