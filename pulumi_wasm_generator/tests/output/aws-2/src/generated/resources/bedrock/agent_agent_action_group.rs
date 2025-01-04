/// Resource for managing an AWS Agents for Amazon Bedrock Agent Action Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:AgentAgentActionGroup
///     properties:
///       actionGroupName: example
///       agentId: GGRRAED6JP
///       agentVersion: DRAFT
///       skipResourceInUseCheck: true
///       actionGroupExecutor:
///         lambda: arn:aws:lambda:us-west-2:123456789012:function:example-function
///       apiSchema:
///         payload:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: path/to/schema.yaml
///             return: result
/// ```
///
/// ### API Schema in S3 Bucket
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = agent_agent_action_group::create(
///         "example",
///         AgentAgentActionGroupArgs::builder()
///             .action_group_executor(
///                 AgentAgentActionGroupActionGroupExecutor::builder()
///                     .lambda(
///                         "arn:aws:lambda:us-west-2:123456789012:function:example-function",
///                     )
///                     .build_struct(),
///             )
///             .action_group_name("example")
///             .agent_id("GGRRAED6JP")
///             .agent_version("DRAFT")
///             .api_schema(
///                 AgentAgentActionGroupApiSchema::builder()
///                     .s3(
///                         AgentAgentActionGroupApiSchemaS3::builder()
///                             .s3BucketName("example-bucket")
///                             .s3ObjectKey("path/to/schema.json")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .skip_resource_in_use_check(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Function Schema (Simplified Schema)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = agent_agent_action_group::create(
///         "example",
///         AgentAgentActionGroupArgs::builder()
///             .action_group_executor(
///                 AgentAgentActionGroupActionGroupExecutor::builder()
///                     .lambda(
///                         "arn:aws:lambda:us-west-2:123456789012:function:example-function",
///                     )
///                     .build_struct(),
///             )
///             .action_group_name("example")
///             .agent_id("GGRRAED6JP")
///             .agent_version("DRAFT")
///             .function_schema(
///                 AgentAgentActionGroupFunctionSchema::builder()
///                     .memberFunctions(
///                         AgentAgentActionGroupFunctionSchemaMemberFunctions::builder()
///                             .functions(
///                                 vec![
///                                     AgentAgentActionGroupFunctionSchemaMemberFunctionsFunction::builder()
///                                     .description("Example function").name("example-function")
///                                     .parameters(vec![AgentAgentActionGroupFunctionSchemaMemberFunctionsFunctionParameter::builder()
///                                     .description("The first parameter").mapBlockKey("param1")
///                                     .required(true). type ("string").build_struct(),
///                                     AgentAgentActionGroupFunctionSchemaMemberFunctionsFunctionParameter::builder()
///                                     .description("The second parameter").mapBlockKey("param2")
///                                     .required(false). type ("integer").build_struct(),])
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .skip_resource_in_use_check(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Return of Control
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:AgentAgentActionGroup
///     properties:
///       actionGroupName: example
///       agentId: GGRRAED6JP
///       agentVersion: DRAFT
///       skipResourceInUseCheck: true
///       actionGroupExecutor:
///         customControl: RETURN_CONTROL
///       apiSchema:
///         payload:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: path/to/schema.yaml
///             return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Agent Action Group the action group ID, the agent ID, and the agent version separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentAgentActionGroup:AgentAgentActionGroup example MMAUDBZTH4,GGRRAED6JP,DRAFT
/// ```
pub mod agent_agent_action_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentActionGroupArgs {
        /// ARN of the Lambda function containing the business logic that is carried out upon invoking the action or custom control method for handling the information elicited from the user. See `action_group_executor` Block for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub action_group_executor: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::AgentAgentActionGroupActionGroupExecutor,
            >,
        >,
        /// Name of the action group.
        #[builder(into)]
        pub action_group_name: pulumi_wasm_rust::Output<String>,
        /// Whether the action group is available for the agent to invoke or not when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub action_group_state: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier of the agent for which to create the action group.
        #[builder(into)]
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Version of the agent for which to create the action group. Valid values: `DRAFT`.
        #[builder(into)]
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// Either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see [Action group OpenAPI schemas](https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html). See `api_schema` Block for details.
        #[builder(into, default)]
        pub api_schema: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupApiSchema>,
        >,
        /// Description of the action group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the function schema for the action group.
        /// Each function represents an action in an action group.
        /// See `function_schema` Block for details.
        #[builder(into, default)]
        pub function_schema: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupFunctionSchema>,
        >,
        /// To allow your agent to request the user for additional information when trying to complete a task, set this argument to `AMAZON.UserInput`. You must leave the `description`, `api_schema`, and `action_group_executor` arguments blank for this action group. Valid values: `AMAZON.UserInput`.
        #[builder(into, default)]
        pub parent_action_group_signature: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not to prepare the agent after creation or modification. Defaults to `true`.
        #[builder(into, default)]
        pub prepare_agent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the in-use check is skipped when deleting the action group.
        #[builder(into, default)]
        pub skip_resource_in_use_check: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentActionGroupResult {
        /// ARN of the Lambda function containing the business logic that is carried out upon invoking the action or custom control method for handling the information elicited from the user. See `action_group_executor` Block for details.
        ///
        /// The following arguments are optional:
        pub action_group_executor: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::AgentAgentActionGroupActionGroupExecutor,
            >,
        >,
        /// Unique identifier of the action group.
        pub action_group_id: pulumi_wasm_rust::Output<String>,
        /// Name of the action group.
        pub action_group_name: pulumi_wasm_rust::Output<String>,
        /// Whether the action group is available for the agent to invoke or not when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        pub action_group_state: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the agent for which to create the action group.
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Version of the agent for which to create the action group. Valid values: `DRAFT`.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// Either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see [Action group OpenAPI schemas](https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html). See `api_schema` Block for details.
        pub api_schema: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupApiSchema>,
        >,
        /// Description of the action group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the function schema for the action group.
        /// Each function represents an action in an action group.
        /// See `function_schema` Block for details.
        pub function_schema: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupFunctionSchema>,
        >,
        /// To allow your agent to request the user for additional information when trying to complete a task, set this argument to `AMAZON.UserInput`. You must leave the `description`, `api_schema`, and `action_group_executor` arguments blank for this action group. Valid values: `AMAZON.UserInput`.
        pub parent_action_group_signature: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not to prepare the agent after creation or modification. Defaults to `true`.
        pub prepare_agent: pulumi_wasm_rust::Output<bool>,
        /// Whether the in-use check is skipped when deleting the action group.
        pub skip_resource_in_use_check: pulumi_wasm_rust::Output<bool>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AgentAgentActionGroupArgs,
    ) -> AgentAgentActionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_group_executor_binding = args.action_group_executor.get_inner();
        let action_group_name_binding = args.action_group_name.get_inner();
        let action_group_state_binding = args.action_group_state.get_inner();
        let agent_id_binding = args.agent_id.get_inner();
        let agent_version_binding = args.agent_version.get_inner();
        let api_schema_binding = args.api_schema.get_inner();
        let description_binding = args.description.get_inner();
        let function_schema_binding = args.function_schema.get_inner();
        let parent_action_group_signature_binding = args
            .parent_action_group_signature
            .get_inner();
        let prepare_agent_binding = args.prepare_agent.get_inner();
        let skip_resource_in_use_check_binding = args
            .skip_resource_in_use_check
            .get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentActionGroup:AgentAgentActionGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionGroupExecutor".into(),
                    value: &action_group_executor_binding,
                },
                register_interface::ObjectField {
                    name: "actionGroupName".into(),
                    value: &action_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "actionGroupState".into(),
                    value: &action_group_state_binding,
                },
                register_interface::ObjectField {
                    name: "agentId".into(),
                    value: &agent_id_binding,
                },
                register_interface::ObjectField {
                    name: "agentVersion".into(),
                    value: &agent_version_binding,
                },
                register_interface::ObjectField {
                    name: "apiSchema".into(),
                    value: &api_schema_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "functionSchema".into(),
                    value: &function_schema_binding,
                },
                register_interface::ObjectField {
                    name: "parentActionGroupSignature".into(),
                    value: &parent_action_group_signature_binding,
                },
                register_interface::ObjectField {
                    name: "prepareAgent".into(),
                    value: &prepare_agent_binding,
                },
                register_interface::ObjectField {
                    name: "skipResourceInUseCheck".into(),
                    value: &skip_resource_in_use_check_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionGroupExecutor".into(),
                },
                register_interface::ResultField {
                    name: "actionGroupId".into(),
                },
                register_interface::ResultField {
                    name: "actionGroupName".into(),
                },
                register_interface::ResultField {
                    name: "actionGroupState".into(),
                },
                register_interface::ResultField {
                    name: "agentId".into(),
                },
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "apiSchema".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "functionSchema".into(),
                },
                register_interface::ResultField {
                    name: "parentActionGroupSignature".into(),
                },
                register_interface::ResultField {
                    name: "prepareAgent".into(),
                },
                register_interface::ResultField {
                    name: "skipResourceInUseCheck".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AgentAgentActionGroupResult {
            action_group_executor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionGroupExecutor").unwrap(),
            ),
            action_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionGroupId").unwrap(),
            ),
            action_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionGroupName").unwrap(),
            ),
            action_group_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionGroupState").unwrap(),
            ),
            agent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentId").unwrap(),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            api_schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiSchema").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            function_schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionSchema").unwrap(),
            ),
            parent_action_group_signature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentActionGroupSignature").unwrap(),
            ),
            prepare_agent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prepareAgent").unwrap(),
            ),
            skip_resource_in_use_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipResourceInUseCheck").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
