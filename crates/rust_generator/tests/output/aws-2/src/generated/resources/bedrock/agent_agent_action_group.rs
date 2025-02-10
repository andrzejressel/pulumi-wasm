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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agent_agent_action_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentActionGroupArgs {
        /// ARN of the Lambda function containing the business logic that is carried out upon invoking the action or custom control method for handling the information elicited from the user. See `action_group_executor` Block for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub action_group_executor: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentAgentActionGroupActionGroupExecutor,
            >,
        >,
        /// Name of the action group.
        #[builder(into)]
        pub action_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the action group is available for the agent to invoke or not when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub action_group_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier of the agent for which to create the action group.
        #[builder(into)]
        pub agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the agent for which to create the action group. Valid values: `DRAFT`.
        #[builder(into)]
        pub agent_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see [Action group OpenAPI schemas](https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html). See `api_schema` Block for details.
        #[builder(into, default)]
        pub api_schema: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentActionGroupApiSchema>,
        >,
        /// Description of the action group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes the function schema for the action group.
        /// Each function represents an action in an action group.
        /// See `function_schema` Block for details.
        #[builder(into, default)]
        pub function_schema: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentActionGroupFunctionSchema>,
        >,
        /// To allow your agent to request the user for additional information when trying to complete a task, set this argument to `AMAZON.UserInput`. You must leave the `description`, `api_schema`, and `action_group_executor` arguments blank for this action group. Valid values: `AMAZON.UserInput`.
        #[builder(into, default)]
        pub parent_action_group_signature: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether or not to prepare the agent after creation or modification. Defaults to `true`.
        #[builder(into, default)]
        pub prepare_agent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the in-use check is skipped when deleting the action group.
        #[builder(into, default)]
        pub skip_resource_in_use_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentActionGroupTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentActionGroupResult {
        /// ARN of the Lambda function containing the business logic that is carried out upon invoking the action or custom control method for handling the information elicited from the user. See `action_group_executor` Block for details.
        ///
        /// The following arguments are optional:
        pub action_group_executor: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentAgentActionGroupActionGroupExecutor,
            >,
        >,
        /// Unique identifier of the action group.
        pub action_group_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the action group.
        pub action_group_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the action group is available for the agent to invoke or not when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        pub action_group_state: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the agent for which to create the action group.
        pub agent_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the agent for which to create the action group. Valid values: `DRAFT`.
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        /// Either details about the S3 object containing the OpenAPI schema for the action group or the JSON or YAML-formatted payload defining the schema. For more information, see [Action group OpenAPI schemas](https://docs.aws.amazon.com/bedrock/latest/userguide/agents-api-schema.html). See `api_schema` Block for details.
        pub api_schema: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupApiSchema>,
        >,
        /// Description of the action group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Describes the function schema for the action group.
        /// Each function represents an action in an action group.
        /// See `function_schema` Block for details.
        pub function_schema: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupFunctionSchema>,
        >,
        /// To allow your agent to request the user for additional information when trying to complete a task, set this argument to `AMAZON.UserInput`. You must leave the `description`, `api_schema`, and `action_group_executor` arguments blank for this action group. Valid values: `AMAZON.UserInput`.
        pub parent_action_group_signature: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether or not to prepare the agent after creation or modification. Defaults to `true`.
        pub prepare_agent: pulumi_gestalt_rust::Output<bool>,
        /// Whether the in-use check is skipped when deleting the action group.
        pub skip_resource_in_use_check: pulumi_gestalt_rust::Output<bool>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentAgentActionGroupTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AgentAgentActionGroupArgs,
    ) -> AgentAgentActionGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_group_executor_binding = args
            .action_group_executor
            .get_output(context);
        let action_group_name_binding = args.action_group_name.get_output(context);
        let action_group_state_binding = args.action_group_state.get_output(context);
        let agent_id_binding = args.agent_id.get_output(context);
        let agent_version_binding = args.agent_version.get_output(context);
        let api_schema_binding = args.api_schema.get_output(context);
        let description_binding = args.description.get_output(context);
        let function_schema_binding = args.function_schema.get_output(context);
        let parent_action_group_signature_binding = args
            .parent_action_group_signature
            .get_output(context);
        let prepare_agent_binding = args.prepare_agent.get_output(context);
        let skip_resource_in_use_check_binding = args
            .skip_resource_in_use_check
            .get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentActionGroup:AgentAgentActionGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionGroupExecutor".into(),
                    value: action_group_executor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionGroupName".into(),
                    value: action_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionGroupState".into(),
                    value: action_group_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentId".into(),
                    value: agent_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentVersion".into(),
                    value: agent_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiSchema".into(),
                    value: api_schema_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionSchema".into(),
                    value: function_schema_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentActionGroupSignature".into(),
                    value: parent_action_group_signature_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prepareAgent".into(),
                    value: prepare_agent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipResourceInUseCheck".into(),
                    value: skip_resource_in_use_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgentAgentActionGroupResult {
            action_group_executor: o.get_field("actionGroupExecutor"),
            action_group_id: o.get_field("actionGroupId"),
            action_group_name: o.get_field("actionGroupName"),
            action_group_state: o.get_field("actionGroupState"),
            agent_id: o.get_field("agentId"),
            agent_version: o.get_field("agentVersion"),
            api_schema: o.get_field("apiSchema"),
            description: o.get_field("description"),
            function_schema: o.get_field("functionSchema"),
            parent_action_group_signature: o.get_field("parentActionGroupSignature"),
            prepare_agent: o.get_field("prepareAgent"),
            skip_resource_in_use_check: o.get_field("skipResourceInUseCheck"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
