pub mod get_pipeline_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPipelineDefinitionArgs {
        /// Parameter values used in the pipeline definition. See below
        #[builder(into, default)]
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::datapipeline::GetPipelineDefinitionParameterValue,
                >,
            >,
        >,
        /// ID of the pipeline.
        #[builder(into)]
        pub pipeline_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPipelineDefinitionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Parameter objects used in the pipeline definition. See below
        pub parameter_objects: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::datapipeline::GetPipelineDefinitionParameterObject,
            >,
        >,
        /// Parameter values used in the pipeline definition. See below
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::datapipeline::GetPipelineDefinitionParameterValue,
                >,
            >,
        >,
        pub pipeline_id: pulumi_wasm_rust::Output<String>,
        /// Objects defined in the pipeline. See below
        pub pipeline_objects: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::datapipeline::GetPipelineDefinitionPipelineObject,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPipelineDefinitionArgs) -> GetPipelineDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parameter_values_binding = args.parameter_values.get_inner();
        let pipeline_id_binding = args.pipeline_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:datapipeline/getPipelineDefinition:getPipelineDefinition".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineId".into(),
                    value: &pipeline_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parameterObjects".into(),
                },
                register_interface::ResultField {
                    name: "parameterValues".into(),
                },
                register_interface::ResultField {
                    name: "pipelineId".into(),
                },
                register_interface::ResultField {
                    name: "pipelineObjects".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPipelineDefinitionResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parameter_objects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterObjects").unwrap(),
            ),
            parameter_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterValues").unwrap(),
            ),
            pipeline_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineId").unwrap(),
            ),
            pipeline_objects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineObjects").unwrap(),
            ),
        }
    }
}