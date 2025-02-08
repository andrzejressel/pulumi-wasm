#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_agent_agent_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAgentAgentVersionsArgs {
        /// Unique identifier of the agent.
        #[builder(into)]
        pub agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of objects, each of which contains information about a version of the agent. See Agent Version Summaries
        #[builder(into, default)]
        pub agent_version_summaries: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::bedrock::GetAgentAgentVersionsAgentVersionSummary,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAgentAgentVersionsResult {
        pub agent_id: pulumi_gestalt_rust::Output<String>,
        /// List of objects, each of which contains information about a version of the agent. See Agent Version Summaries
        pub agent_version_summaries: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::bedrock::GetAgentAgentVersionsAgentVersionSummary,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAgentAgentVersionsArgs,
    ) -> GetAgentAgentVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let agent_id_binding = args.agent_id.get_output(context).get_inner();
        let agent_version_summaries_binding = args
            .agent_version_summaries
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getAgentAgentVersions:getAgentAgentVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentId".into(),
                    value: &agent_id_binding,
                },
                register_interface::ObjectField {
                    name: "agentVersionSummaries".into(),
                    value: &agent_version_summaries_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAgentAgentVersionsResult {
            agent_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("agentId"),
            ),
            agent_version_summaries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("agentVersionSummaries"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
