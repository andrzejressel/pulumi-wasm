#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_alert_rule_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleTemplateArgs {
        /// The display name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        ///
        /// > **NOTE** As `display_name` is not unique, errors may occur when there are multiple Sentinel Alert Rule Template with same `display_name`.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleTemplateResult {
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `nrt_template` block as defined below. This only applies to Sentinel NRT Alert Rule Template.
        pub nrt_templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sentinel::GetAlertRuleTemplateNrtTemplate>,
        >,
        /// A `scheduled_template` block as defined below. This only applies to Sentinel Scheduled Alert Rule Template.
        pub scheduled_templates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleTemplateScheduledTemplate,
            >,
        >,
        /// A `security_incident_template` block as defined below. This only applies to Sentinel MS Security Incident Alert Rule Template.
        pub security_incident_templates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleTemplateSecurityIncidentTemplate,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAlertRuleTemplateArgs,
    ) -> GetAlertRuleTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:sentinel/getAlertRuleTemplate:getAlertRuleTemplate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
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
        GetAlertRuleTemplateResult {
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            nrt_templates: o.get_field("nrtTemplates"),
            scheduled_templates: o.get_field("scheduledTemplates"),
            security_incident_templates: o.get_field("securityIncidentTemplates"),
        }
    }
}
