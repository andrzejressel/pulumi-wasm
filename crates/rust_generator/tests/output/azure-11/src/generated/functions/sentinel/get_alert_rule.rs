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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAlertRuleArgs,
    ) -> GetAlertRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:sentinel/getAlertRule:getAlertRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAlertRuleResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
