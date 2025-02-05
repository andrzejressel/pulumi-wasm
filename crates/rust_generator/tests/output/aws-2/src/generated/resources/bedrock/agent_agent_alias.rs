/// Resource for managing an AWS Agents for Amazon Bedrock Agent Alias.
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
///       idleTtl: 500
///       foundationModel: anthropic.claude-v2
///   exampleAgentAgentAlias:
///     type: aws:bedrock:AgentAgentAlias
///     name: example
///     properties:
///       agentAliasName: my-agent-alias
///       agentId: ${exampleAgentAgent.agentId}
///       description: Test Alias
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
/// Using `pulumi import`, import Agents for Amazon Bedrock Agent Alias using the alias ID and the agent ID separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentAgentAlias:AgentAgentAlias example 66IVY0GUTF,GGRRAED6JP
/// ```
pub mod agent_agent_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentAliasArgs {
        /// Name of the alias.
        #[builder(into)]
        pub agent_alias_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the agent to create an alias for.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub agent_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Details about the routing configuration of the alias. See `routing_configuration` Block for details.
        #[builder(into, default)]
        pub routing_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::bedrock::AgentAgentAliasRoutingConfiguration>,
            >,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentAliasTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentAliasResult {
        /// ARN of the alias.
        pub agent_alias_arn: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the alias.
        pub agent_alias_id: pulumi_wasm_rust::Output<String>,
        /// Name of the alias.
        pub agent_alias_name: pulumi_wasm_rust::Output<String>,
        /// Identifier of the agent to create an alias for.
        ///
        /// The following arguments are optional:
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the routing configuration of the alias. See `routing_configuration` Block for details.
        pub routing_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::bedrock::AgentAgentAliasRoutingConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::AgentAgentAliasTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AgentAgentAliasArgs,
    ) -> AgentAgentAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_alias_name_binding = args
            .agent_alias_name
            .get_output(context)
            .get_inner();
        let agent_id_binding = args.agent_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let routing_configurations_binding = args
            .routing_configurations
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentAlias:AgentAgentAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentAliasName".into(),
                    value: &agent_alias_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentId".into(),
                    value: &agent_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "routingConfigurations".into(),
                    value: &routing_configurations_binding,
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
        AgentAgentAliasResult {
            agent_alias_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentAliasArn"),
            ),
            agent_alias_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentAliasId"),
            ),
            agent_alias_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentAliasName"),
            ),
            agent_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            routing_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routingConfigurations"),
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
