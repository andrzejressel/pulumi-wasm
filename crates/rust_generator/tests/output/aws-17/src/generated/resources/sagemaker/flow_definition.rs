/// Provides a SageMaker Flow Definition resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = flow_definition::create(
///         "example",
///         FlowDefinitionArgs::builder()
///             .flow_definition_name("example")
///             .human_loop_config(
///                 FlowDefinitionHumanLoopConfig::builder()
///                     .humanTaskUiArn("${exampleAwsSagemakerHumanTaskUi.arn}")
///                     .taskAvailabilityLifetimeInSeconds(1)
///                     .taskCount(1)
///                     .taskDescription("example")
///                     .taskTitle("example")
///                     .workteamArn("${exampleAwsSagemakerWorkteam.arn}")
///                     .build_struct(),
///             )
///             .output_config(
///                 FlowDefinitionOutputConfig::builder()
///                     .s3OutputPath("s3://${exampleAwsS3Bucket.bucket}/")
///                     .build_struct(),
///             )
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Public Workteam Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = flow_definition::create(
///         "example",
///         FlowDefinitionArgs::builder()
///             .flow_definition_name("example")
///             .human_loop_config(
///                 FlowDefinitionHumanLoopConfig::builder()
///                     .humanTaskUiArn("${exampleAwsSagemakerHumanTaskUi.arn}")
///                     .publicWorkforceTaskPrice(
///                         FlowDefinitionHumanLoopConfigPublicWorkforceTaskPrice::builder()
///                             .amountInUsd(
///                                 FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd::builder()
///                                     .cents(1)
///                                     .tenthFractionsOfACent(2)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .taskAvailabilityLifetimeInSeconds(1)
///                     .taskCount(1)
///                     .taskDescription("example")
///                     .taskTitle("example")
///                     .workteamArn(
///                         "arn:aws:sagemaker:${current.name}:394669845002:workteam/public-crowd/default",
///                     )
///                     .build_struct(),
///             )
///             .output_config(
///                 FlowDefinitionOutputConfig::builder()
///                     .s3OutputPath("s3://${exampleAwsS3Bucket.bucket}/")
///                     .build_struct(),
///             )
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Human Loop Activation Config Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = flow_definition::create(
///         "example",
///         FlowDefinitionArgs::builder()
///             .flow_definition_name("example")
///             .human_loop_activation_config(
///                 FlowDefinitionHumanLoopActivationConfig::builder()
///                     .humanLoopActivationConditionsConfig(
///                         FlowDefinitionHumanLoopActivationConfigHumanLoopActivationConditionsConfig::builder()
///                             .humanLoopActivationConditions(
///                                 "        {\n\t\t\t\"Conditions\": [\n\t\t\t  {\n\t\t\t\t\"ConditionType\": \"Sampling\",\n\t\t\t\t\"ConditionParameters\": {\n\t\t\t\t  \"RandomSamplingPercentage\": 5\n\t\t\t\t}\n\t\t\t  }\n\t\t\t]\n\t\t}\n",
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .human_loop_config(
///                 FlowDefinitionHumanLoopConfig::builder()
///                     .humanTaskUiArn("${exampleAwsSagemakerHumanTaskUi.arn}")
///                     .taskAvailabilityLifetimeInSeconds(1)
///                     .taskCount(1)
///                     .taskDescription("example")
///                     .taskTitle("example")
///                     .workteamArn("${exampleAwsSagemakerWorkteam.arn}")
///                     .build_struct(),
///             )
///             .human_loop_request_source(
///                 FlowDefinitionHumanLoopRequestSource::builder()
///                     .awsManagedHumanLoopRequestSource(
///                         "AWS/Textract/AnalyzeDocument/Forms/V1",
///                     )
///                     .build_struct(),
///             )
///             .output_config(
///                 FlowDefinitionOutputConfig::builder()
///                     .s3OutputPath("s3://${exampleAwsS3Bucket.bucket}/")
///                     .build_struct(),
///             )
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Flow Definitions using the `flow_definition_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/flowDefinition:FlowDefinition example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flow_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowDefinitionArgs {
        /// The name of your flow definition.
        #[builder(into)]
        pub flow_definition_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An object containing information about the events that trigger a human workflow. See Human Loop Activation Config details below.
        #[builder(into, default)]
        pub human_loop_activation_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfig,
            >,
        >,
        /// An object containing information about the tasks the human reviewers will perform. See Human Loop Config details below.
        #[builder(into)]
        pub human_loop_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::FlowDefinitionHumanLoopConfig,
        >,
        /// Container for configuring the source of human task requests. Use to specify if Amazon Rekognition or Amazon Textract is used as an integration source. See Human Loop Request Source details below.
        #[builder(into, default)]
        pub human_loop_request_source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::FlowDefinitionHumanLoopRequestSource>,
        >,
        /// An object containing information about where the human review results will be uploaded. See Output Config details below.
        #[builder(into)]
        pub output_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::FlowDefinitionOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) of the role needed to call other services on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlowDefinitionResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Flow Definition.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of your flow definition.
        pub flow_definition_name: pulumi_gestalt_rust::Output<String>,
        /// An object containing information about the events that trigger a human workflow. See Human Loop Activation Config details below.
        pub human_loop_activation_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfig,
            >,
        >,
        /// An object containing information about the tasks the human reviewers will perform. See Human Loop Config details below.
        pub human_loop_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::FlowDefinitionHumanLoopConfig,
        >,
        /// Container for configuring the source of human task requests. Use to specify if Amazon Rekognition or Amazon Textract is used as an integration source. See Human Loop Request Source details below.
        pub human_loop_request_source: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::FlowDefinitionHumanLoopRequestSource>,
        >,
        /// An object containing information about where the human review results will be uploaded. See Output Config details below.
        pub output_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::FlowDefinitionOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) of the role needed to call other services on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
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
        args: FlowDefinitionArgs,
    ) -> FlowDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let flow_definition_name_binding = args.flow_definition_name.get_output(context);
        let human_loop_activation_config_binding = args
            .human_loop_activation_config
            .get_output(context);
        let human_loop_config_binding = args.human_loop_config.get_output(context);
        let human_loop_request_source_binding = args
            .human_loop_request_source
            .get_output(context);
        let output_config_binding = args.output_config.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/flowDefinition:FlowDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flowDefinitionName".into(),
                    value: flow_definition_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "humanLoopActivationConfig".into(),
                    value: human_loop_activation_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "humanLoopConfig".into(),
                    value: human_loop_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "humanLoopRequestSource".into(),
                    value: human_loop_request_source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputConfig".into(),
                    value: output_config_binding.get_id(),
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
        FlowDefinitionResult {
            arn: o.get_field("arn"),
            flow_definition_name: o.get_field("flowDefinitionName"),
            human_loop_activation_config: o.get_field("humanLoopActivationConfig"),
            human_loop_config: o.get_field("humanLoopConfig"),
            human_loop_request_source: o.get_field("humanLoopRequestSource"),
            output_config: o.get_field("outputConfig"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
