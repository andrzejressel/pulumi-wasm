/// Resource for managing an AWS Agents for Amazon Bedrock Agent.
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
///       assumeRolePolicy: ${exampleAgentTrust.json}
///       namePrefix: AmazonBedrockExecutionRoleForAgents_
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       policy: ${exampleAgentPermissions.json}
///       role: ${example.id}
///   exampleAgentAgent:
///     type: aws:bedrock:AgentAgent
///     name: example
///     properties:
///       agentName: my-agent-name
///       agentResourceRoleArn: ${example.arn}
///       idleSessionTtlInSeconds: 500
///       foundationModel: anthropic.claude-v2
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   exampleAgentTrust:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - identifiers:
///                   - bedrock.amazonaws.com
///                 type: Service
///             conditions:
///               - test: StringEquals
///                 values:
///                   - ${current.accountId}
///                 variable: aws:SourceAccount
///               - test: ArnLike
///                 values:
///                   - arn:${currentGetPartition.partition}:bedrock:${currentGetRegion.name}:${current.accountId}:agent/*
///                 variable: AWS:SourceArn
///   exampleAgentPermissions:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - bedrock:InvokeModel
///             resources:
///               - arn:${currentGetPartition.partition}:bedrock:${currentGetRegion.name}::foundation-model/anthropic.claude-v2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Agent using the agent ID. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentAgent:AgentAgent example GGRRAED6JP
/// ```
pub mod agent_agent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentArgs {
        /// Agents collaboration role. Valid values: `SUPERVISOR`, `SUPERVISOR_ROUTER`, `DISABLED`.
        #[builder(into, default)]
        pub agent_collaboration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the agent.
        #[builder(into)]
        pub agent_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the agent.
        #[builder(into)]
        pub agent_resource_role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the AWS KMS key that encrypts the agent.
        #[builder(into, default)]
        pub customer_encryption_key_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the agent.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Foundation model used for orchestration by the agent.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub foundation_model: pulumi_wasm_rust::InputOrOutput<String>,
        /// Details about the guardrail associated with the agent. See `guardrail_configuration` Block for details.
        #[builder(into, default)]
        pub guardrail_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::bedrock::AgentAgentGuardrailConfiguration>>,
        >,
        /// Number of seconds for which Amazon Bedrock keeps information about a user's conversation with the agent. A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Bedrock deletes any data provided before the timeout.
        #[builder(into, default)]
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Instructions that tell the agent what it should do and how it should interact with users. The valid range is 40 - 8000 characters.
        #[builder(into, default)]
        pub instruction: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to prepare the agent after creation or modification. Defaults to `true`.
        #[builder(into, default)]
        pub prepare_agent: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configurations to override prompt templates in different parts of an agent sequence. For more information, see [Advanced prompts](https://docs.aws.amazon.com/bedrock/latest/userguide/advanced-prompts.html). See `prompt_override_configuration` Block for details.
        #[builder(into, default)]
        pub prompt_override_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfiguration>,
            >,
        >,
        /// Whether the in-use check is skipped when deleting the agent.
        #[builder(into, default)]
        pub skip_resource_in_use_check: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentResult {
        /// ARN of the agent.
        pub agent_arn: pulumi_wasm_rust::Output<String>,
        /// Agents collaboration role. Valid values: `SUPERVISOR`, `SUPERVISOR_ROUTER`, `DISABLED`.
        pub agent_collaboration: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the agent.
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Name of the agent.
        pub agent_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the agent.
        pub agent_resource_role_arn: pulumi_wasm_rust::Output<String>,
        /// Version of the agent.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// ARN of the AWS KMS key that encrypts the agent.
        pub customer_encryption_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the agent.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Foundation model used for orchestration by the agent.
        ///
        /// The following arguments are optional:
        pub foundation_model: pulumi_wasm_rust::Output<String>,
        /// Details about the guardrail associated with the agent. See `guardrail_configuration` Block for details.
        pub guardrail_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::bedrock::AgentAgentGuardrailConfiguration>>,
        >,
        /// Number of seconds for which Amazon Bedrock keeps information about a user's conversation with the agent. A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Bedrock deletes any data provided before the timeout.
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Instructions that tell the agent what it should do and how it should interact with users. The valid range is 40 - 8000 characters.
        pub instruction: pulumi_wasm_rust::Output<String>,
        /// Whether to prepare the agent after creation or modification. Defaults to `true`.
        pub prepare_agent: pulumi_wasm_rust::Output<bool>,
        /// Configurations to override prompt templates in different parts of an agent sequence. For more information, see [Advanced prompts](https://docs.aws.amazon.com/bedrock/latest/userguide/advanced-prompts.html). See `prompt_override_configuration` Block for details.
        pub prompt_override_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfiguration>,
        >,
        /// Whether the in-use check is skipped when deleting the agent.
        pub skip_resource_in_use_check: pulumi_wasm_rust::Output<bool>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AgentAgentArgs,
    ) -> AgentAgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_collaboration_binding = args
            .agent_collaboration
            .get_output(context)
            .get_inner();
        let agent_name_binding = args.agent_name.get_output(context).get_inner();
        let agent_resource_role_arn_binding = args
            .agent_resource_role_arn
            .get_output(context)
            .get_inner();
        let customer_encryption_key_arn_binding = args
            .customer_encryption_key_arn
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let foundation_model_binding = args
            .foundation_model
            .get_output(context)
            .get_inner();
        let guardrail_configurations_binding = args
            .guardrail_configurations
            .get_output(context)
            .get_inner();
        let idle_session_ttl_in_seconds_binding = args
            .idle_session_ttl_in_seconds
            .get_output(context)
            .get_inner();
        let instruction_binding = args.instruction.get_output(context).get_inner();
        let prepare_agent_binding = args.prepare_agent.get_output(context).get_inner();
        let prompt_override_configurations_binding = args
            .prompt_override_configurations
            .get_output(context)
            .get_inner();
        let skip_resource_in_use_check_binding = args
            .skip_resource_in_use_check
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgent:AgentAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentCollaboration".into(),
                    value: &agent_collaboration_binding,
                },
                register_interface::ObjectField {
                    name: "agentName".into(),
                    value: &agent_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentResourceRoleArn".into(),
                    value: &agent_resource_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "customerEncryptionKeyArn".into(),
                    value: &customer_encryption_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "foundationModel".into(),
                    value: &foundation_model_binding,
                },
                register_interface::ObjectField {
                    name: "guardrailConfigurations".into(),
                    value: &guardrail_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "idleSessionTtlInSeconds".into(),
                    value: &idle_session_ttl_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "instruction".into(),
                    value: &instruction_binding,
                },
                register_interface::ObjectField {
                    name: "prepareAgent".into(),
                    value: &prepare_agent_binding,
                },
                register_interface::ObjectField {
                    name: "promptOverrideConfigurations".into(),
                    value: &prompt_override_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "skipResourceInUseCheck".into(),
                    value: &skip_resource_in_use_check_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AgentAgentResult {
            agent_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentArn"),
            ),
            agent_collaboration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentCollaboration"),
            ),
            agent_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentId"),
            ),
            agent_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentName"),
            ),
            agent_resource_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentResourceRoleArn"),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentVersion"),
            ),
            customer_encryption_key_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerEncryptionKeyArn"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            foundation_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("foundationModel"),
            ),
            guardrail_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guardrailConfigurations"),
            ),
            idle_session_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("idleSessionTtlInSeconds"),
            ),
            instruction: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instruction"),
            ),
            prepare_agent: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("prepareAgent"),
            ),
            prompt_override_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("promptOverrideConfigurations"),
            ),
            skip_resource_in_use_check: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipResourceInUseCheck"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
