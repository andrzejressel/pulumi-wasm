/// Resource for managing an AWS Agents for Amazon Bedrock Agent Knowledge Base Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = agent_agent_knowledge_base_association::create(
///         "example",
///         AgentAgentKnowledgeBaseAssociationArgs::builder()
///             .agent_id("GGRRAED6JP")
///             .description("Example Knowledge base")
///             .knowledge_base_id("EMDPPAYPZI")
///             .knowledge_base_state("ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Agent Knowledge Base Association using the agent ID, the agent version, and the knowledge base ID separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentAgentKnowledgeBaseAssociation:AgentAgentKnowledgeBaseAssociation example GGRRAED6JP,DRAFT,EMDPPAYPZI
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agent_agent_knowledge_base_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentKnowledgeBaseAssociationArgs {
        /// Unique identifier of the agent with which you want to associate the knowledge base.
        #[builder(into)]
        pub agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the agent with which you want to associate the knowledge base. Valid values: `DRAFT`.
        #[builder(into, default)]
        pub agent_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of what the agent should use the knowledge base for.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique identifier of the knowledge base to associate with the agent.
        #[builder(into)]
        pub knowledge_base_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to use the knowledge base when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub knowledge_base_state: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentAgentKnowledgeBaseAssociationTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentKnowledgeBaseAssociationResult {
        /// Unique identifier of the agent with which you want to associate the knowledge base.
        pub agent_id: pulumi_gestalt_rust::Output<String>,
        /// Version of the agent with which you want to associate the knowledge base. Valid values: `DRAFT`.
        pub agent_version: pulumi_gestalt_rust::Output<String>,
        /// Description of what the agent should use the knowledge base for.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the knowledge base to associate with the agent.
        pub knowledge_base_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to use the knowledge base when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        pub knowledge_base_state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentAgentKnowledgeBaseAssociationTimeouts,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AgentAgentKnowledgeBaseAssociationArgs,
    ) -> AgentAgentKnowledgeBaseAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_id_binding = args.agent_id.get_output(context);
        let agent_version_binding = args.agent_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let knowledge_base_id_binding = args.knowledge_base_id.get_output(context);
        let knowledge_base_state_binding = args.knowledge_base_state.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentKnowledgeBaseAssociation:AgentAgentKnowledgeBaseAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentId".into(),
                    value: agent_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentVersion".into(),
                    value: agent_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "knowledgeBaseId".into(),
                    value: knowledge_base_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "knowledgeBaseState".into(),
                    value: knowledge_base_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgentAgentKnowledgeBaseAssociationResult {
            agent_id: o.get_field("agentId"),
            agent_version: o.get_field("agentVersion"),
            description: o.get_field("description"),
            knowledge_base_id: o.get_field("knowledgeBaseId"),
            knowledge_base_state: o.get_field("knowledgeBaseState"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
