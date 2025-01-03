pub mod get_network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPeeringArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPeeringResult {
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub export_custom_routes: pulumi_wasm_rust::Output<bool>,
        pub export_custom_routes_with_public_ip: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub import_custom_routes: pulumi_wasm_rust::Output<bool>,
        pub import_custom_routes_with_public_ip: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub peer_network: pulumi_wasm_rust::Output<String>,
        pub peer_network_type: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub state_details: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
        pub vmware_engine_network_canonical: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkPeeringArgs) -> GetNetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getNetworkPeering:getNetworkPeering".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "exportCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "exportCustomRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "importCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "importCustomRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerNetwork".into(),
                },
                register_interface::ResultField {
                    name: "peerNetworkType".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateDetails".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetwork".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetworkCanonical".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkPeeringResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            export_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportCustomRoutes").unwrap(),
            ),
            export_custom_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportCustomRoutesWithPublicIp").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            import_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importCustomRoutes").unwrap(),
            ),
            import_custom_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importCustomRoutesWithPublicIp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peer_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerNetwork").unwrap(),
            ),
            peer_network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerNetworkType").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateDetails").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vmware_engine_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetwork").unwrap(),
            ),
            vmware_engine_network_canonical: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetworkCanonical").unwrap(),
            ),
        }
    }
}
