pub mod get_slack_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSlackWorkspaceArgs {
        /// Slack workspace name configured with AWS Chatbot.
        #[builder(into)]
        pub slack_team_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSlackWorkspaceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the Slack Workspace assigned by AWS Chatbot.
        pub slack_team_id: pulumi_gestalt_rust::Output<String>,
        pub slack_team_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSlackWorkspaceArgs,
    ) -> GetSlackWorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let slack_team_name_binding = args
            .slack_team_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:chatbot/getSlackWorkspace:getSlackWorkspace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "slackTeamName".into(),
                    value: &slack_team_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSlackWorkspaceResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            slack_team_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackTeamId"),
            ),
            slack_team_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slackTeamName"),
            ),
        }
    }
}
