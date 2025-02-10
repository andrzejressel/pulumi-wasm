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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agent_agent_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentAliasArgs {
        /// Name of the alias.
        #[builder(into)]
        pub agent_alias_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the agent to create an alias for.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the routing configuration of the alias. See `routing_configuration` Block for details.
        #[builder(into, default)]
        pub routing_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::bedrock::AgentAgentAliasRoutingConfiguration>,
            >,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentAgentAliasTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentAliasResult {
        /// ARN of the alias.
        pub agent_alias_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the alias.
        pub agent_alias_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the alias.
        pub agent_alias_name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the agent to create an alias for.
        ///
        /// The following arguments are optional:
        pub agent_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Details about the routing configuration of the alias. See `routing_configuration` Block for details.
        pub routing_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bedrock::AgentAgentAliasRoutingConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentAgentAliasTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AgentAgentAliasArgs,
    ) -> AgentAgentAliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_alias_name_binding = args.agent_alias_name.get_output(context);
        let agent_id_binding = args.agent_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let routing_configurations_binding = args
            .routing_configurations
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentAlias:AgentAgentAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentAliasName".into(),
                    value: agent_alias_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentId".into(),
                    value: agent_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingConfigurations".into(),
                    value: routing_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgentAgentAliasResult {
            agent_alias_arn: o.get_field("agentAliasArn"),
            agent_alias_id: o.get_field("agentAliasId"),
            agent_alias_name: o.get_field("agentAliasName"),
            agent_id: o.get_field("agentId"),
            description: o.get_field("description"),
            routing_configurations: o.get_field("routingConfigurations"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
