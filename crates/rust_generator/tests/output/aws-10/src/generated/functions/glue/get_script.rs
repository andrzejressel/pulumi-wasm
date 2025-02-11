#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_script {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScriptArgs {
        /// List of the edges in the DAG. Defined below.
        #[builder(into)]
        pub dag_edges: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::glue::GetScriptDagEdge>,
        >,
        /// List of the nodes in the DAG. Defined below.
        #[builder(into)]
        pub dag_nodes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::glue::GetScriptDagNode>,
        >,
        /// Programming language of the resulting code from the DAG. Defaults to `PYTHON`. Valid values are `PYTHON` and `SCALA`.
        #[builder(into, default)]
        pub language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetScriptResult {
        pub dag_edges: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagEdge>,
        >,
        pub dag_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::glue::GetScriptDagNode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Python script generated from the DAG when the `language` argument is set to `PYTHON`.
        pub python_script: pulumi_gestalt_rust::Output<String>,
        /// Scala code generated from the DAG when the `language` argument is set to `SCALA`.
        pub scala_code: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetScriptArgs,
    ) -> GetScriptResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dag_edges_binding = args.dag_edges.get_output(context);
        let dag_nodes_binding = args.dag_nodes.get_output(context);
        let language_binding = args.language.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:glue/getScript:getScript".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dagEdges".into(),
                    value: &dag_edges_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dagNodes".into(),
                    value: &dag_nodes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "language".into(),
                    value: &language_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetScriptResult {
            dag_edges: o.get_field("dagEdges"),
            dag_nodes: o.get_field("dagNodes"),
            id: o.get_field("id"),
            language: o.get_field("language"),
            python_script: o.get_field("pythonScript"),
            scala_code: o.get_field("scalaCode"),
        }
    }
}
