/// Resource for managing an AWS EventBridge Pipes Pipe.
///
/// You can find out more about EventBridge Pipes in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).
///
/// EventBridge Pipes are very configurable, and may require IAM permissions to work correctly. More information on the configuration options and IAM permissions can be found in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html).
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             Effect: Allow
///             Action: sts:AssumeRole
///             Principal:
///               Service: pipes.amazonaws.com
///             Condition:
///               StringEquals:
///                 aws:SourceAccount: ${main.accountId}
///   source:
///     type: aws:iam:RolePolicy
///     properties:
///       role: ${example.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Effect: Allow
///               Action:
///                 - sqs:DeleteMessage
///                 - sqs:GetQueueAttributes
///                 - sqs:ReceiveMessage
///               Resource:
///                 - ${sourceQueue.arn}
///   sourceQueue:
///     type: aws:sqs:Queue
///     name: source
///   target:
///     type: aws:iam:RolePolicy
///     properties:
///       role: ${example.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Effect: Allow
///               Action:
///                 - sqs:SendMessage
///               Resource:
///                 - ${targetQueue.arn}
///   targetQueue:
///     type: aws:sqs:Queue
///     name: target
///   examplePipe:
///     type: aws:pipes:Pipe
///     name: example
///     properties:
///       name: example-pipe
///       roleArn: ${example.arn}
///       source: ${sourceQueue.arn}
///       target: ${targetQueue.arn}
///     options:
///       dependsOn:
///         - ${source}
///         - ${target}
/// variables:
///   main:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Enrichment Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:pipes:Pipe
///     properties:
///       name: example-pipe
///       roleArn: ${exampleAwsIamRole.arn}
///       source: ${source.arn}
///       target: ${target.arn}
///       enrichment: ${exampleAwsCloudwatchEventApiDestination.arn}
///       enrichmentParameters:
///         httpParameters:
///           pathParameterValues: example-path-param
///           headerParameters:
///             example-header: example-value
///             second-example-header: second-example-value
///           queryStringParameters:
///             example-query-string: example-value
///             second-example-query-string: second-example-value
/// ```
///
/// ### Filter Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:pipes:Pipe
///     properties:
///       name: example-pipe
///       roleArn: ${exampleAwsIamRole.arn}
///       source: ${source.arn}
///       target: ${target.arn}
///       sourceParameters:
///         filterCriteria:
///           filters:
///             - pattern:
///                 fn::toJSON:
///                   source:
///                     - event-source
/// ```
///
/// ### CloudWatch Logs Logging Configuration Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder().name("example-pipe-target").build_struct(),
///     );
///     let examplePipe = pipe::create(
///         "examplePipe",
///         PipeArgs::builder()
///             .log_configuration(
///                 PipeLogConfiguration::builder()
///                     .cloudwatchLogsLogDestination(
///                         PipeLogConfigurationCloudwatchLogsLogDestination::builder()
///                             .logGroupArn("${targetAwsCloudwatchLogGroup.arn}")
///                             .build_struct(),
///                     )
///                     .includeExecutionDatas(vec!["ALL",])
///                     .level("INFO")
///                     .build_struct(),
///             )
///             .name("example-pipe")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .source("${sourceAwsSqsQueue.arn}")
///             .target("${targetAwsSqsQueue.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SQS Source and Target Configuration Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = pipe::create(
///         "example",
///         PipeArgs::builder()
///             .name("example-pipe")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .source("${source.arn}")
///             .source_parameters(
///                 PipeSourceParameters::builder()
///                     .sqsQueueParameters(
///                         PipeSourceParametersSqsQueueParameters::builder()
///                             .batchSize(1)
///                             .maximumBatchingWindowInSeconds(2)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .target("${target.arn}")
///             .target_parameters(
///                 PipeTargetParameters::builder()
///                     .sqsQueueParameters(
///                         PipeTargetParametersSqsQueueParameters::builder()
///                             .messageDeduplicationId("example-dedupe")
///                             .messageGroupId("example-group")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import pipes using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:pipes/pipe:Pipe example my-pipe
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipe {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipeArgs {
        /// A description of the pipe. At most 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state the pipe should be in. One of: `RUNNING`, `STOPPED`.
        #[builder(into, default)]
        pub desired_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enrichment resource of the pipe (typically an ARN). Read more about enrichment in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html#pipes-enrichment).
        #[builder(into, default)]
        pub enrichment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters to configure enrichment for your pipe. Detailed below.
        #[builder(into, default)]
        pub enrichment_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pipes::PipeEnrichmentParameters>,
        >,
        /// Logging configuration settings for the pipe. Detailed below.
        #[builder(into, default)]
        pub log_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pipes::PipeLogConfiguration>,
        >,
        /// Name of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the role that allows the pipe to send data to the target.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Parameters to configure a source for the pipe. Detailed below.
        #[builder(into, default)]
        pub source_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pipes::PipeSourceParameters>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target resource of the pipe (typically an ARN).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Parameters to configure a target for your pipe. Detailed below.
        #[builder(into, default)]
        pub target_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pipes::PipeTargetParameters>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipeResult {
        /// ARN of this pipe.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the pipe. At most 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state the pipe should be in. One of: `RUNNING`, `STOPPED`.
        pub desired_state: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enrichment resource of the pipe (typically an ARN). Read more about enrichment in the [User Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html#pipes-enrichment).
        pub enrichment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Parameters to configure enrichment for your pipe. Detailed below.
        pub enrichment_parameters: pulumi_gestalt_rust::Output<
            Option<super::super::types::pipes::PipeEnrichmentParameters>,
        >,
        /// Logging configuration settings for the pipe. Detailed below.
        pub log_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::pipes::PipeLogConfiguration>,
        >,
        /// Name of the pipe. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// ARN of the role that allows the pipe to send data to the target.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// Parameters to configure a source for the pipe. Detailed below.
        pub source_parameters: pulumi_gestalt_rust::Output<
            super::super::types::pipes::PipeSourceParameters,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Target resource of the pipe (typically an ARN).
        ///
        /// The following arguments are optional:
        pub target: pulumi_gestalt_rust::Output<String>,
        /// Parameters to configure a target for your pipe. Detailed below.
        pub target_parameters: pulumi_gestalt_rust::Output<
            Option<super::super::types::pipes::PipeTargetParameters>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PipeArgs,
    ) -> PipeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let desired_state_binding = args.desired_state.get_output(context);
        let enrichment_binding = args.enrichment.get_output(context);
        let enrichment_parameters_binding = args
            .enrichment_parameters
            .get_output(context);
        let log_configuration_binding = args.log_configuration.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let source_binding = args.source.get_output(context);
        let source_parameters_binding = args.source_parameters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_binding = args.target.get_output(context);
        let target_parameters_binding = args.target_parameters.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pipes/pipe:Pipe".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desiredState".into(),
                    value: desired_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enrichment".into(),
                    value: enrichment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enrichmentParameters".into(),
                    value: enrichment_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfiguration".into(),
                    value: log_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceParameters".into(),
                    value: source_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetParameters".into(),
                    value: target_parameters_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PipeResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            desired_state: o.get_field("desiredState"),
            enrichment: o.get_field("enrichment"),
            enrichment_parameters: o.get_field("enrichmentParameters"),
            log_configuration: o.get_field("logConfiguration"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            role_arn: o.get_field("roleArn"),
            source: o.get_field("source"),
            source_parameters: o.get_field("sourceParameters"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target: o.get_field("target"),
            target_parameters: o.get_field("targetParameters"),
        }
    }
}
