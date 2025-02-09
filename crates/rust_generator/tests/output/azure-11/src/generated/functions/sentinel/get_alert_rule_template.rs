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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAlertRuleTemplateArgs,
    ) -> GetAlertRuleTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let log_analytics_workspace_id_binding_1 = args
            .log_analytics_workspace_id
            .get_output(context);
        let log_analytics_workspace_id_binding = log_analytics_workspace_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:sentinel/getAlertRuleTemplate:getAlertRuleTemplate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
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
        GetAlertRuleTemplateResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nrt_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nrtTemplates"),
            ),
            scheduled_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scheduledTemplates"),
            ),
            security_incident_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityIncidentTemplates"),
            ),
        }
    }
}
