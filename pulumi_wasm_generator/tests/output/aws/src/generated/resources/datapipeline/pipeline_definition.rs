/// Provides a DataPipeline Pipeline Definition resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = pipeline::create(
///         "default",
///         PipelineArgs::builder().name("tf-pipeline-default").build_struct(),
///     );
///     let example = pipeline_definition::create(
///         "example",
///         PipelineDefinitionArgs::builder()
///             .pipeline_id("${default.id}")
///             .pipeline_objects(
///                 vec![
///                     PipelineDefinitionPipelineObject::builder()
///                     .fields(vec![PipelineDefinitionPipelineObjectField::builder()
///                     .key("workerGroup").stringValue("workerGroup").build_struct(),])
///                     .id("Default").name("Default").build_struct(),
///                     PipelineDefinitionPipelineObject::builder()
///                     .fields(vec![PipelineDefinitionPipelineObjectField::builder()
///                     .key("startDateTime").stringValue("2012-12-12T00:00:00")
///                     .build_struct(), PipelineDefinitionPipelineObjectField::builder()
///                     .key("type").stringValue("Schedule").build_struct(),
///                     PipelineDefinitionPipelineObjectField::builder().key("period")
///                     .stringValue("1 hour").build_struct(),
///                     PipelineDefinitionPipelineObjectField::builder().key("endDateTime")
///                     .stringValue("2012-12-21T18:00:00").build_struct(),]).id("Schedule")
///                     .name("Schedule").build_struct(),
///                     PipelineDefinitionPipelineObject::builder()
///                     .fields(vec![PipelineDefinitionPipelineObjectField::builder()
///                     .key("type").stringValue("ShellCommandActivity").build_struct(),
///                     PipelineDefinitionPipelineObjectField::builder().key("command")
///                     .stringValue("echo hello").build_struct(),
///                     PipelineDefinitionPipelineObjectField::builder().key("parent")
///                     .stringValue("Default").build_struct(),
///                     PipelineDefinitionPipelineObjectField::builder().key("schedule")
///                     .stringValue("Schedule").build_struct(),]).id("SayHello")
///                     .name("SayHello").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datapipeline_pipeline_definition` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:datapipeline/pipelineDefinition:PipelineDefinition example df-1234567890
/// ```
pub mod pipeline_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineDefinitionArgs {
        /// Configuration block for the parameter objects used in the pipeline definition. See below
        #[builder(into, default)]
        pub parameter_objects: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datapipeline::PipelineDefinitionParameterObject>,
            >,
        >,
        /// Configuration block for the parameter values used in the pipeline definition. See below
        #[builder(into, default)]
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datapipeline::PipelineDefinitionParameterValue>,
            >,
        >,
        /// ID of the pipeline.
        #[builder(into)]
        pub pipeline_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the objects that define the pipeline. See below
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub pipeline_objects: pulumi_wasm_rust::Output<
            Vec<super::super::types::datapipeline::PipelineDefinitionPipelineObject>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineDefinitionResult {
        /// Configuration block for the parameter objects used in the pipeline definition. See below
        pub parameter_objects: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datapipeline::PipelineDefinitionParameterObject>,
            >,
        >,
        /// Configuration block for the parameter values used in the pipeline definition. See below
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datapipeline::PipelineDefinitionParameterValue>,
            >,
        >,
        /// ID of the pipeline.
        pub pipeline_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the objects that define the pipeline. See below
        ///
        /// The following arguments are optional:
        pub pipeline_objects: pulumi_wasm_rust::Output<
            Vec<super::super::types::datapipeline::PipelineDefinitionPipelineObject>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PipelineDefinitionArgs) -> PipelineDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parameter_objects_binding = args.parameter_objects.get_inner();
        let parameter_values_binding = args.parameter_values.get_inner();
        let pipeline_id_binding = args.pipeline_id.get_inner();
        let pipeline_objects_binding = args.pipeline_objects.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datapipeline/pipelineDefinition:PipelineDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parameterObjects".into(),
                    value: &parameter_objects_binding,
                },
                register_interface::ObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineId".into(),
                    value: &pipeline_id_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineObjects".into(),
                    value: &pipeline_objects_binding,
                },
            ]),
            results: Vec::from([
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PipelineDefinitionResult {
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