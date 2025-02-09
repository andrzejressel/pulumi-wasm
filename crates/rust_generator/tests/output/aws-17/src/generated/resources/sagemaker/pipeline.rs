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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default. see Parallelism Configuration details below.
        #[builder(into, default)]
        pub parallelism_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::PipelineParallelismConfiguration>,
        >,
        /// The [JSON pipeline definition](https://aws-sagemaker-mlops.github.io/sagemaker-model-building-pipeline-definition-JSON-schema/) of the pipeline.
        #[builder(into, default)]
        pub pipeline_definition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location. see Pipeline Definition S3 Location details below.
        #[builder(into, default)]
        pub pipeline_definition_s3_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::PipelinePipelineDefinitionS3Location>,
        >,
        /// A description of the pipeline.
        #[builder(into, default)]
        pub pipeline_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the pipeline.
        #[builder(into)]
        pub pipeline_display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the pipeline.
        #[builder(into)]
        pub pipeline_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the IAM role the pipeline will execute as.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Pipeline.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default. see Parallelism Configuration details below.
        pub parallelism_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::PipelineParallelismConfiguration>,
        >,
        /// The [JSON pipeline definition](https://aws-sagemaker-mlops.github.io/sagemaker-model-building-pipeline-definition-JSON-schema/) of the pipeline.
        pub pipeline_definition: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location. see Pipeline Definition S3 Location details below.
        pub pipeline_definition_s3_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::PipelinePipelineDefinitionS3Location>,
        >,
        /// A description of the pipeline.
        pub pipeline_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the pipeline.
        pub pipeline_display_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the pipeline.
        pub pipeline_name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role the pipeline will execute as.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parallelism_configuration_binding = args
            .parallelism_configuration
            .get_output(context);
        let pipeline_definition_binding = args.pipeline_definition.get_output(context);
        let pipeline_definition_s3_location_binding = args
            .pipeline_definition_s3_location
            .get_output(context);
        let pipeline_description_binding = args.pipeline_description.get_output(context);
        let pipeline_display_name_binding = args
            .pipeline_display_name
            .get_output(context);
        let pipeline_name_binding = args.pipeline_name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parallelismConfiguration".into(),
                    value: parallelism_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineDefinition".into(),
                    value: pipeline_definition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineDefinitionS3Location".into(),
                    value: pipeline_definition_s3_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineDescription".into(),
                    value: pipeline_description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineDisplayName".into(),
                    value: pipeline_display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineName".into(),
                    value: pipeline_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PipelineResult {
            arn: o.get_field("arn"),
            parallelism_configuration: o.get_field("parallelismConfiguration"),
            pipeline_definition: o.get_field("pipelineDefinition"),
            pipeline_definition_s3_location: o.get_field("pipelineDefinitionS3Location"),
            pipeline_description: o.get_field("pipelineDescription"),
            pipeline_display_name: o.get_field("pipelineDisplayName"),
            pipeline_name: o.get_field("pipelineName"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
