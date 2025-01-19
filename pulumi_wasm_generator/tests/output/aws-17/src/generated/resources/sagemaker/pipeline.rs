/// Provides a SageMaker Pipeline resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:Pipeline
///     properties:
///       pipelineName: example
///       pipelineDisplayName: example
///       roleArn: ${exampleAwsIamRole.arn}
///       pipelineDefinition:
///         fn::toJSON:
///           Version: 2020-12-01
///           Steps:
///             - Name: Test
///               Type: Fail
///               Arguments:
///                 ErrorMessage: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import pipelines using the `pipeline_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/pipeline:Pipeline test_pipeline pipeline
/// ```
pub mod pipeline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default. see Parallelism Configuration details below.
        #[builder(into, default)]
        pub parallelism_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::PipelineParallelismConfiguration>,
        >,
        /// The [JSON pipeline definition](https://aws-sagemaker-mlops.github.io/sagemaker-model-building-pipeline-definition-JSON-schema/) of the pipeline.
        #[builder(into, default)]
        pub pipeline_definition: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location. see Pipeline Definition S3 Location details below.
        #[builder(into, default)]
        pub pipeline_definition_s3_location: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::PipelinePipelineDefinitionS3Location>,
        >,
        /// A description of the pipeline.
        #[builder(into, default)]
        pub pipeline_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the pipeline.
        #[builder(into)]
        pub pipeline_display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the pipeline.
        #[builder(into)]
        pub pipeline_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role the pipeline will execute as.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Pipeline.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default. see Parallelism Configuration details below.
        pub parallelism_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::PipelineParallelismConfiguration>,
        >,
        /// The [JSON pipeline definition](https://aws-sagemaker-mlops.github.io/sagemaker-model-building-pipeline-definition-JSON-schema/) of the pipeline.
        pub pipeline_definition: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location. see Pipeline Definition S3 Location details below.
        pub pipeline_definition_s3_location: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::PipelinePipelineDefinitionS3Location>,
        >,
        /// A description of the pipeline.
        pub pipeline_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the pipeline.
        pub pipeline_display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the pipeline.
        pub pipeline_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role the pipeline will execute as.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PipelineArgs) -> PipelineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parallelism_configuration_binding = args
            .parallelism_configuration
            .get_inner();
        let pipeline_definition_binding = args.pipeline_definition.get_inner();
        let pipeline_definition_s3_location_binding = args
            .pipeline_definition_s3_location
            .get_inner();
        let pipeline_description_binding = args.pipeline_description.get_inner();
        let pipeline_display_name_binding = args.pipeline_display_name.get_inner();
        let pipeline_name_binding = args.pipeline_name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parallelismConfiguration".into(),
                    value: &parallelism_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineDefinition".into(),
                    value: &pipeline_definition_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineDefinitionS3Location".into(),
                    value: &pipeline_definition_s3_location_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineDescription".into(),
                    value: &pipeline_description_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineDisplayName".into(),
                    value: &pipeline_display_name_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineName".into(),
                    value: &pipeline_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "parallelismConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "pipelineDefinition".into(),
                },
                register_interface::ResultField {
                    name: "pipelineDefinitionS3Location".into(),
                },
                register_interface::ResultField {
                    name: "pipelineDescription".into(),
                },
                register_interface::ResultField {
                    name: "pipelineDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "pipelineName".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PipelineResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            parallelism_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parallelismConfiguration").unwrap(),
            ),
            pipeline_definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineDefinition").unwrap(),
            ),
            pipeline_definition_s3_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineDefinitionS3Location").unwrap(),
            ),
            pipeline_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineDescription").unwrap(),
            ),
            pipeline_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineDisplayName").unwrap(),
            ),
            pipeline_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineName").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
