/// Provides a SageMaker Flow Definition resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod flow_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowDefinitionArgs {
        /// The name of your flow definition.
        #[builder(into)]
        pub flow_definition_name: pulumi_wasm_rust::Output<String>,
        /// An object containing information about the events that trigger a human workflow. See Human Loop Activation Config details below.
        #[builder(into, default)]
        pub human_loop_activation_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfig,
            >,
        >,
        /// An object containing information about the tasks the human reviewers will perform. See Human Loop Config details below.
        #[builder(into)]
        pub human_loop_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::FlowDefinitionHumanLoopConfig,
        >,
        /// Container for configuring the source of human task requests. Use to specify if Amazon Rekognition or Amazon Textract is used as an integration source. See Human Loop Request Source details below.
        #[builder(into, default)]
        pub human_loop_request_source: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FlowDefinitionHumanLoopRequestSource>,
        >,
        /// An object containing information about where the human review results will be uploaded. See Output Config details below.
        #[builder(into)]
        pub output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::FlowDefinitionOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) of the role needed to call other services on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlowDefinitionResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Flow Definition.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of your flow definition.
        pub flow_definition_name: pulumi_wasm_rust::Output<String>,
        /// An object containing information about the events that trigger a human workflow. See Human Loop Activation Config details below.
        pub human_loop_activation_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfig,
            >,
        >,
        /// An object containing information about the tasks the human reviewers will perform. See Human Loop Config details below.
        pub human_loop_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::FlowDefinitionHumanLoopConfig,
        >,
        /// Container for configuring the source of human task requests. Use to specify if Amazon Rekognition or Amazon Textract is used as an integration source. See Human Loop Request Source details below.
        pub human_loop_request_source: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::FlowDefinitionHumanLoopRequestSource>,
        >,
        /// An object containing information about where the human review results will be uploaded. See Output Config details below.
        pub output_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::FlowDefinitionOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) of the role needed to call other services on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: FlowDefinitionArgs) -> FlowDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let flow_definition_name_binding = args.flow_definition_name.get_inner();
        let human_loop_activation_config_binding = args
            .human_loop_activation_config
            .get_inner();
        let human_loop_config_binding = args.human_loop_config.get_inner();
        let human_loop_request_source_binding = args
            .human_loop_request_source
            .get_inner();
        let output_config_binding = args.output_config.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/flowDefinition:FlowDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "flowDefinitionName".into(),
                    value: &flow_definition_name_binding,
                },
                register_interface::ObjectField {
                    name: "humanLoopActivationConfig".into(),
                    value: &human_loop_activation_config_binding,
                },
                register_interface::ObjectField {
                    name: "humanLoopConfig".into(),
                    value: &human_loop_config_binding,
                },
                register_interface::ObjectField {
                    name: "humanLoopRequestSource".into(),
                    value: &human_loop_request_source_binding,
                },
                register_interface::ObjectField {
                    name: "outputConfig".into(),
                    value: &output_config_binding,
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
                    name: "flowDefinitionName".into(),
                },
                register_interface::ResultField {
                    name: "humanLoopActivationConfig".into(),
                },
                register_interface::ResultField {
                    name: "humanLoopConfig".into(),
                },
                register_interface::ResultField {
                    name: "humanLoopRequestSource".into(),
                },
                register_interface::ResultField {
                    name: "outputConfig".into(),
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
        FlowDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            flow_definition_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flowDefinitionName").unwrap(),
            ),
            human_loop_activation_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("humanLoopActivationConfig").unwrap(),
            ),
            human_loop_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("humanLoopConfig").unwrap(),
            ),
            human_loop_request_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("humanLoopRequestSource").unwrap(),
            ),
            output_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputConfig").unwrap(),
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