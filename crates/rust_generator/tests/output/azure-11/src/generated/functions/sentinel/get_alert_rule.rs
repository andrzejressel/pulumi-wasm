#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_alert_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleArgs {
        /// The ID of the Log Analytics Workspace this Sentinel Alert Rule belongs to.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel Alert Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAlertRuleArgs,
    ) -> GetAlertRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:sentinel/getAlertRule:getAlertRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAlertRuleResult {
            id: o.get_field("id"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
        }
    }
}
