pub mod get_slack_workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSlackWorkspaceArgs {
        /// Slack workspace name configured with AWS Chatbot.
        #[builder(into)]
        pub slack_team_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSlackWorkspaceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the Slack Workspace assigned by AWS Chatbot.
        pub slack_team_id: pulumi_wasm_rust::Output<String>,
        pub slack_team_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSlackWorkspaceArgs) -> GetSlackWorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let slack_team_name_binding = args.slack_team_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:chatbot/getSlackWorkspace:getSlackWorkspace".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "slackTeamName".into(),
                    value: &slack_team_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "slackTeamId".into(),
                },
                register_interface::ResultField {
                    name: "slackTeamName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSlackWorkspaceResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            slack_team_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackTeamId").unwrap(),
            ),
            slack_team_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slackTeamName").unwrap(),
            ),
        }
    }
}