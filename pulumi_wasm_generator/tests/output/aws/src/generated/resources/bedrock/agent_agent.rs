/// Resource for managing an AWS Agents for Amazon Bedrock Agent.
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
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let currentGetPartition = get_partition::invoke(
///         GetPartitionArgs::builder().build_struct(),
///     );
///     let currentGetRegion = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let exampleAgentPermissions = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["bedrock:InvokeModel",])
///                     .resources(vec!["arn:${currentGetPartition.partition}:bedrock:${currentGetRegion.name}::foundation-model/anthropic.claude-v2",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleAgentTrust = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["${current.accountId}",])
///                     .variable("aws:SourceAccount").build_struct(),
///                     GetPolicyDocumentStatementCondition::builder().test("ArnLike")
///                     .values(vec!["arn:${currentGetPartition.partition}:bedrock:${currentGetRegion.name}:${current.accountId}:agent/*",])
///                     .variable("AWS:SourceArn").build_struct(),])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["bedrock.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${exampleAgentTrust.json}")
///             .name_prefix("AmazonBedrockExecutionRoleForAgents_")
///             .build_struct(),
///     );
///     let exampleAgentAgent = agent_agent::create(
///         "exampleAgentAgent",
///         AgentAgentArgs::builder()
///             .agent_name("my-agent-name")
///             .agent_resource_role_arn("${example.arn}")
///             .foundation_model("anthropic.claude-v2")
///             .idle_session_ttl_in_seconds(500)
///             .build_struct(),
///     );
///     let exampleRolePolicy = role_policy::create(
///         "exampleRolePolicy",
///         RolePolicyArgs::builder()
///             .policy("${exampleAgentPermissions.json}")
///             .role("${example.id}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentArgs {
        /// Name of the agent.
        #[builder(into)]
        pub agent_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the agent.
        #[builder(into)]
        pub agent_resource_role_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the AWS KMS key that encrypts the agent.
        #[builder(into, default)]
        pub customer_encryption_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the agent.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Foundation model used for orchestration by the agent.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub foundation_model: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub guardrail_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::bedrock::AgentAgentGuardrailConfiguration>>,
        >,
        /// Number of seconds for which Amazon Bedrock keeps information about a user's conversation with the agent. A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Bedrock deletes any data provided before the timeout.
        #[builder(into, default)]
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Instructions that tell the agent what it should do and how it should interact with users.
        #[builder(into, default)]
        pub instruction: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to prepare the agent after creation or modification. Defaults to `true`.
        #[builder(into, default)]
        pub prepare_agent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configurations to override prompt templates in different parts of an agent sequence. For more information, see [Advanced prompts](https://docs.aws.amazon.com/bedrock/latest/userguide/advanced-prompts.html). See `prompt_override_configuration` Block for details.
        #[builder(into, default)]
        pub prompt_override_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfiguration>,
            >,
        >,
        /// Whether the in-use check is skipped when deleting the agent.
        #[builder(into, default)]
        pub skip_resource_in_use_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentResult {
        /// ARN of the agent.
        pub agent_arn: pulumi_wasm_rust::Output<String>,
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
        pub guardrail_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::bedrock::AgentAgentGuardrailConfiguration>>,
        >,
        /// Number of seconds for which Amazon Bedrock keeps information about a user's conversation with the agent. A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Bedrock deletes any data provided before the timeout.
        pub idle_session_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Instructions that tell the agent what it should do and how it should interact with users.
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
    pub fn create(name: &str, args: AgentAgentArgs) -> AgentAgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_name_binding = args.agent_name.get_inner();
        let agent_resource_role_arn_binding = args.agent_resource_role_arn.get_inner();
        let customer_encryption_key_arn_binding = args
            .customer_encryption_key_arn
            .get_inner();
        let description_binding = args.description.get_inner();
        let foundation_model_binding = args.foundation_model.get_inner();
        let guardrail_configurations_binding = args.guardrail_configurations.get_inner();
        let idle_session_ttl_in_seconds_binding = args
            .idle_session_ttl_in_seconds
            .get_inner();
        let instruction_binding = args.instruction.get_inner();
        let prepare_agent_binding = args.prepare_agent.get_inner();
        let prompt_override_configurations_binding = args
            .prompt_override_configurations
            .get_inner();
        let skip_resource_in_use_check_binding = args
            .skip_resource_in_use_check
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgent:AgentAgent".into(),
            name: name.to_string(),
            object: Vec::from([
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentArn".into(),
                },
                register_interface::ResultField {
                    name: "agentId".into(),
                },
                register_interface::ResultField {
                    name: "agentName".into(),
                },
                register_interface::ResultField {
                    name: "agentResourceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "customerEncryptionKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "foundationModel".into(),
                },
                register_interface::ResultField {
                    name: "guardrailConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "idleSessionTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "instruction".into(),
                },
                register_interface::ResultField {
                    name: "prepareAgent".into(),
                },
                register_interface::ResultField {
                    name: "promptOverrideConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "skipResourceInUseCheck".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
        AgentAgentResult {
            agent_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentArn").unwrap(),
            ),
            agent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentId").unwrap(),
            ),
            agent_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentName").unwrap(),
            ),
            agent_resource_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentResourceRoleArn").unwrap(),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            customer_encryption_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerEncryptionKeyArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            foundation_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("foundationModel").unwrap(),
            ),
            guardrail_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guardrailConfigurations").unwrap(),
            ),
            idle_session_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleSessionTtlInSeconds").unwrap(),
            ),
            instruction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instruction").unwrap(),
            ),
            prepare_agent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prepareAgent").unwrap(),
            ),
            prompt_override_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("promptOverrideConfigurations").unwrap(),
            ),
            skip_resource_in_use_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipResourceInUseCheck").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}