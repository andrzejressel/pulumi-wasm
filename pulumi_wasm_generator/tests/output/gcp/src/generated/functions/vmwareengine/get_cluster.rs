pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the private cloud that this cluster belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub autoscaling_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetClusterAutoscalingSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub management: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub node_type_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetClusterNodeTypeConfig>,
        >,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterArgs) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getCluster:getCluster".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingSettings".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "management".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeTypeConfigs".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            autoscaling_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("management").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_type_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeTypeConfigs").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uid").unwrap()),
        }
    }
}
