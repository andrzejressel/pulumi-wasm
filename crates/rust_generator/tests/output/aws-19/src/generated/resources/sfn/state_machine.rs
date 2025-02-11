/// Provides a Step Function State Machine resource
///
/// ## Example Usage
///
/// ### Basic (Standard Workflow)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sfnStateMachine = state_machine::create(
///         "sfnStateMachine",
///         StateMachineArgs::builder()
///             .definition(
///                 "{\n  \"Comment\": \"A Hello World example of the Amazon States Language using an AWS Lambda Function\",\n  \"StartAt\": \"HelloWorld\",\n  \"States\": {\n    \"HelloWorld\": {\n      \"Type\": \"Task\",\n      \"Resource\": \"${lambda.arn}\",\n      \"End\": true\n    }\n  }\n}",
///             )
///             .name("my-state-machine")
///             .role_arn("${iamForSfn.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic (Express Workflow)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sfnStateMachine = state_machine::create(
///         "sfnStateMachine",
///         StateMachineArgs::builder()
///             .definition(
///                 "{\n  \"Comment\": \"A Hello World example of the Amazon States Language using an AWS Lambda Function\",\n  \"StartAt\": \"HelloWorld\",\n  \"States\": {\n    \"HelloWorld\": {\n      \"Type\": \"Task\",\n      \"Resource\": \"${lambda.arn}\",\n      \"End\": true\n    }\n  }\n}",
///             )
///             .name("my-state-machine")
///             .role_arn("${iamForSfn.arn}")
///             .type_("EXPRESS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Publish (Publish SFN version)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sfnStateMachine = state_machine::create(
///         "sfnStateMachine",
///         StateMachineArgs::builder()
///             .definition(
///                 "{\n  \"Comment\": \"A Hello World example of the Amazon States Language using an AWS Lambda Function\",\n  \"StartAt\": \"HelloWorld\",\n  \"States\": {\n    \"HelloWorld\": {\n      \"Type\": \"Task\",\n      \"Resource\": \"${lambda.arn}\",\n      \"End\": true\n    }\n  }\n}",
///             )
///             .name("my-state-machine")
///             .publish(true)
///             .role_arn("${iamForSfn.arn}")
///             .type_("EXPRESS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Logging
///
/// > *NOTE:* See the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling Step Function logging.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sfnStateMachine = state_machine::create(
///         "sfnStateMachine",
///         StateMachineArgs::builder()
///             .definition(
///                 "{\n  \"Comment\": \"A Hello World example of the Amazon States Language using an AWS Lambda Function\",\n  \"StartAt\": \"HelloWorld\",\n  \"States\": {\n    \"HelloWorld\": {\n      \"Type\": \"Task\",\n      \"Resource\": \"${lambda.arn}\",\n      \"End\": true\n    }\n  }\n}\n",
///             )
///             .logging_configuration(
///                 StateMachineLoggingConfiguration::builder()
///                     .includeExecutionData(true)
///                     .level("ERROR")
///                     .logDestination("${logGroupForSfn.arn}:*")
///                     .build_struct(),
///             )
///             .name("my-state-machine")
///             .role_arn("${iamForSfn.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Encryption
///
/// > *NOTE:* See the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling encryption of data using a customer-managed key for Step Functions State Machines data.
///
/// ```yaml
/// resources:
///   # ...
///   sfnStateMachine:
///     type: aws:sfn:StateMachine
///     name: sfn_state_machine
///     properties:
///       name: my-state-machine
///       roleArn: ${iamForSfn.arn}
///       definition: |
///         {
///           "Comment": "A Hello World example of the Amazon States Language using an AWS Lambda Function",
///           "StartAt": "HelloWorld",
///           "States": {
///             "HelloWorld": {
///               "Type": "Task",
///               "Resource": "${lambda.arn}",
///               "End": true
///             }
///           }
///         }
///       encryptionConfiguration:
///         kmsKeyId: ${kmsKeyForSfn.arn}
///         type: CUSTOMER_MANAGED_KMS_KEY
///         kmsDataKeyReusePeriodSeconds: 900
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import State Machines using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sfn/stateMachine:StateMachine foo arn:aws:states:eu-west-1:123456789098:stateMachine:bar
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod state_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StateMachineArgs {
        /// The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sfn::StateMachineEncryptionConfiguration>,
        >,
        /// Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
        #[builder(into, default)]
        pub logging_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sfn::StateMachineLoggingConfiguration>,
        >,
        /// The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to true to publish a version of the state machine during creation. Default: false.
        #[builder(into, default)]
        pub publish: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the IAM role to use for this state machine.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Selects whether AWS X-Ray tracing is enabled.
        #[builder(into, default)]
        pub tracing_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sfn::StateMachineTracingConfiguration>,
        >,
        /// Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StateMachineResult {
        /// The ARN of the state machine.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date the state machine was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
        pub definition: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::sfn::StateMachineEncryptionConfiguration,
        >,
        /// Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
        pub logging_configuration: pulumi_gestalt_rust::Output<
            super::super::types::sfn::StateMachineLoggingConfiguration,
        >,
        /// The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Set to true to publish a version of the state machine during creation. Default: false.
        pub publish: pulumi_gestalt_rust::Output<Option<bool>>,
        pub revision_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to use for this state machine.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the state machine version.
        pub state_machine_version_arn: pulumi_gestalt_rust::Output<String>,
        /// The current status of the state machine. Either `ACTIVE` or `DELETING`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Selects whether AWS X-Ray tracing is enabled.
        pub tracing_configuration: pulumi_gestalt_rust::Output<
            super::super::types::sfn::StateMachineTracingConfiguration,
        >,
        /// Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        pub version_description: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StateMachineArgs,
    ) -> StateMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let definition_binding = args.definition.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let logging_configuration_binding = args
            .logging_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let publish_binding = args.publish.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tracing_configuration_binding = args
            .tracing_configuration
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sfn/stateMachine:StateMachine".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfiguration".into(),
                    value: &logging_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publish".into(),
                    value: &publish_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tracingConfiguration".into(),
                    value: &tracing_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StateMachineResult {
            arn: o.get_field("arn"),
            creation_date: o.get_field("creationDate"),
            definition: o.get_field("definition"),
            description: o.get_field("description"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            logging_configuration: o.get_field("loggingConfiguration"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            publish: o.get_field("publish"),
            revision_id: o.get_field("revisionId"),
            role_arn: o.get_field("roleArn"),
            state_machine_version_arn: o.get_field("stateMachineVersionArn"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tracing_configuration: o.get_field("tracingConfiguration"),
            type_: o.get_field("type"),
            version_description: o.get_field("versionDescription"),
        }
    }
}
