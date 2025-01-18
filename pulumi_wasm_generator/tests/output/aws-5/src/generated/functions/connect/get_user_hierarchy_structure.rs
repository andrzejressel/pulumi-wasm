pub mod get_user_hierarchy_structure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserHierarchyStructureArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserHierarchyStructureResult {
        /// Block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
        pub hierarchy_structures: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::connect::GetUserHierarchyStructureHierarchyStructure,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetUserHierarchyStructureArgs,
    ) -> GetUserHierarchyStructureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getUserHierarchyStructure:getUserHierarchyStructure"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hierarchyStructures".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserHierarchyStructureResult {
            hierarchy_structures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hierarchyStructures").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
        }
    }
}
