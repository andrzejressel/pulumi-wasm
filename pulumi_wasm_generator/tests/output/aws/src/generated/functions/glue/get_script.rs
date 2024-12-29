pub mod get_script {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScriptArgs {
        /// List of the edges in the DAG. Defined below.
        #[builder(into)]
        pub dag_edges: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagEdge>,
        >,
        /// List of the nodes in the DAG. Defined below.
        #[builder(into)]
        pub dag_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagNode>,
        >,
        /// Programming language of the resulting code from the DAG. Defaults to `PYTHON`. Valid values are `PYTHON` and `SCALA`.
        #[builder(into, default)]
        pub language: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetScriptResult {
        pub dag_edges: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagEdge>,
        >,
        pub dag_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagNode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub language: pulumi_wasm_rust::Output<Option<String>>,
        /// Python script generated from the DAG when the `language` argument is set to `PYTHON`.
        pub python_script: pulumi_wasm_rust::Output<String>,
        /// Scala code generated from the DAG when the `language` argument is set to `SCALA`.
        pub scala_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetScriptArgs) -> GetScriptResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dag_edges_binding = args.dag_edges.get_inner();
        let dag_nodes_binding = args.dag_nodes.get_inner();
        let language_binding = args.language.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getScript:getScript".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dagEdges".into(),
                    value: &dag_edges_binding,
                },
                register_interface::ObjectField {
                    name: "dagNodes".into(),
                    value: &dag_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "language".into(),
                    value: &language_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dagEdges".into(),
                },
                register_interface::ResultField {
                    name: "dagNodes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "language".into(),
                },
                register_interface::ResultField {
                    name: "pythonScript".into(),
                },
                register_interface::ResultField {
                    name: "scalaCode".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetScriptResult {
            dag_edges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dagEdges").unwrap(),
            ),
            dag_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dagNodes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("language").unwrap(),
            ),
            python_script: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pythonScript").unwrap(),
            ),
            scala_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalaCode").unwrap(),
            ),
        }
    }
}
