/// Provides a Step Function State Machine resource
///
/// ## Example Usage
///
/// ### Basic (Standard Workflow)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod state_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StateMachineArgs {
        /// The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
        #[builder(into)]
        pub definition: pulumi_wasm_rust::Output<String>,
        /// Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::sfn::StateMachineEncryptionConfiguration>,
        >,
        /// Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
        #[builder(into, default)]
        pub logging_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::sfn::StateMachineLoggingConfiguration>,
        >,
        /// The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Set to true to publish a version of the state machine during creation. Default: false.
        #[builder(into, default)]
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the IAM role to use for this state machine.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Selects whether AWS X-Ray tracing is enabled.
        #[builder(into, default)]
        pub tracing_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::sfn::StateMachineTracingConfiguration>,
        >,
        /// Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StateMachineResult {
        /// The ARN of the state machine.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date the state machine was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) definition of the state machine.
        pub definition: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        /// Defines what encryption configuration is used to encrypt data in the State Machine. For more information see [TBD] in the AWS Step Functions User Guide.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            super::super::types::sfn::StateMachineEncryptionConfiguration,
        >,
        /// Defines what execution history events are logged and where they are logged. The `logging_configuration` parameter is valid when `type` is set to `STANDARD` or `EXPRESS`. Defaults to `OFF`. For more information see [Logging Express Workflows](https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html), [Log Levels](https://docs.aws.amazon.com/step-functions/latest/dg/cloudwatch-log-level.html) and [Logging Configuration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) in the AWS Step Functions User Guide.
        pub logging_configuration: pulumi_wasm_rust::Output<
            super::super::types::sfn::StateMachineLoggingConfiguration,
        >,
        /// The name of the state machine. The name should only contain `0`-`9`, `A`-`Z`, `a`-`z`, `-` and `_`. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Set to true to publish a version of the state machine during creation. Default: false.
        pub publish: pulumi_wasm_rust::Output<Option<bool>>,
        pub revision_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to use for this state machine.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the state machine version.
        pub state_machine_version_arn: pulumi_wasm_rust::Output<String>,
        /// The current status of the state machine. Either `ACTIVE` or `DELETING`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Selects whether AWS X-Ray tracing is enabled.
        pub tracing_configuration: pulumi_wasm_rust::Output<
            super::super::types::sfn::StateMachineTracingConfiguration,
        >,
        /// Determines whether a Standard or Express state machine is created. The default is `STANDARD`. You cannot update the type of a state machine once it has been created. Valid values: `STANDARD`, `EXPRESS`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        pub version_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StateMachineArgs) -> StateMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let logging_configuration_binding = args.logging_configuration.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let publish_binding = args.publish.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let tracing_configuration_binding = args.tracing_configuration.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sfn/stateMachine:StateMachine".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfiguration".into(),
                    value: &logging_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "publish".into(),
                    value: &publish_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tracingConfiguration".into(),
                    value: &tracing_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "publish".into(),
                },
                register_interface::ResultField {
                    name: "revisionId".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "stateMachineVersionArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tracingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "versionDescription".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StateMachineResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            logging_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            publish: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publish").unwrap(),
            ),
            revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionId").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            state_machine_version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateMachineVersionArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tracing_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tracingConfiguration").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionDescription").unwrap(),
            ),
        }
    }
}
