/// Resource for managing an AWS Agents for Amazon Bedrock Agent Knowledge Base Association.
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
pub mod agent_agent_knowledge_base_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentAgentKnowledgeBaseAssociationArgs {
        /// Unique identifier of the agent with which you want to associate the knowledge base.
        #[builder(into)]
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Version of the agent with which you want to associate the knowledge base. Valid values: `DRAFT`.
        #[builder(into, default)]
        pub agent_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of what the agent should use the knowledge base for.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the knowledge base to associate with the agent.
        #[builder(into)]
        pub knowledge_base_id: pulumi_wasm_rust::Output<String>,
        /// Whether to use the knowledge base when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub knowledge_base_state: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::AgentAgentKnowledgeBaseAssociationTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentAgentKnowledgeBaseAssociationResult {
        /// Unique identifier of the agent with which you want to associate the knowledge base.
        pub agent_id: pulumi_wasm_rust::Output<String>,
        /// Version of the agent with which you want to associate the knowledge base. Valid values: `DRAFT`.
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// Description of what the agent should use the knowledge base for.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the knowledge base to associate with the agent.
        pub knowledge_base_id: pulumi_wasm_rust::Output<String>,
        /// Whether to use the knowledge base when sending an [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html) request. Valid values: `ENABLED`, `DISABLED`.
        ///
        /// The following arguments are optional:
        pub knowledge_base_state: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
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
        name: &str,
        args: AgentAgentKnowledgeBaseAssociationArgs,
    ) -> AgentAgentKnowledgeBaseAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_id_binding = args.agent_id.get_inner();
        let agent_version_binding = args.agent_version.get_inner();
        let description_binding = args.description.get_inner();
        let knowledge_base_id_binding = args.knowledge_base_id.get_inner();
        let knowledge_base_state_binding = args.knowledge_base_state.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/agentAgentKnowledgeBaseAssociation:AgentAgentKnowledgeBaseAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentId".into(),
                    value: &agent_id_binding,
                },
                register_interface::ObjectField {
                    name: "agentVersion".into(),
                    value: &agent_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "knowledgeBaseId".into(),
                    value: &knowledge_base_id_binding,
                },
                register_interface::ObjectField {
                    name: "knowledgeBaseState".into(),
                    value: &knowledge_base_state_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "agentId".into(),
                },
                register_interface::ResultField {
                    name: "agentVersion".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "knowledgeBaseId".into(),
                },
                register_interface::ResultField {
                    name: "knowledgeBaseState".into(),
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
        AgentAgentKnowledgeBaseAssociationResult {
            agent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentId").unwrap(),
            ),
            agent_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentVersion").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            knowledge_base_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("knowledgeBaseId").unwrap(),
            ),
            knowledge_base_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("knowledgeBaseState").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
