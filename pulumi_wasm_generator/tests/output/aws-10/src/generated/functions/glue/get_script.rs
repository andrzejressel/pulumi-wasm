pub mod get_script {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScriptArgs {
        /// List of the edges in the DAG. Defined below.
        #[builder(into)]
        pub dag_edges: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::super::types::glue::GetScriptDagEdge>,
        >,
        /// List of the nodes in the DAG. Defined below.
        #[builder(into)]
        pub dag_nodes: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::super::types::glue::GetScriptDagNode>,
        >,
        /// Programming language of the resulting code from the DAG. Defaults to `PYTHON`. Valid values are `PYTHON` and `SCALA`.
        #[builder(into, default)]
        pub language: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetScriptArgs,
    ) -> GetScriptResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dag_edges_binding = args.dag_edges.get_output(context).get_inner();
        let dag_nodes_binding = args.dag_nodes.get_output(context).get_inner();
        let language_binding = args.language.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getScript:getScript".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetScriptResult {
            dag_edges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dagEdges"),
            ),
            dag_nodes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dagNodes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            language: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("language"),
            ),
            python_script: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pythonScript"),
            ),
            scala_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scalaCode"),
            ),
        }
    }
}
