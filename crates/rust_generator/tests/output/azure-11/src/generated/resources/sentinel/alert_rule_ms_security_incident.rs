/// Manages a Sentinel MS Security Incident Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAlertRuleMsSecurityIncident = alert_rule_ms_security_incident::create(
///         "exampleAlertRuleMsSecurityIncident",
///         AlertRuleMsSecurityIncidentArgs::builder()
///             .display_name("example rule")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example-ms-security-incident-alert-rule")
///             .product_filter("Microsoft Cloud App Security")
///             .severity_filters(vec!["High",])
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel MS Security Incident Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleMsSecurityIncident:AlertRuleMsSecurityIncident example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_rule_ms_security_incident {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleMsSecurityIncidentArgs {
        /// The GUID of the alert rule template which is used to create this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of this Sentinel MS Security Incident Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The friendly name of this Sentinel MS Security Incident Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only create incidents when the alert display name doesn't contain text from this list.
        #[builder(into, default)]
        pub display_name_exclude_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Only create incidents when the alert display name contain text from this list, leave empty to apply no filter.
        #[builder(into, default)]
        pub display_name_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Should this Sentinel MS Security Incident Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel MS Security Incident Alert Rule belongs to. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel MS Security Incident Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Microsoft Security Service from where the alert will be generated. Possible values are `Azure Active Directory Identity Protection`, `Azure Advanced Threat Protection`, `Azure Security Center`, `Azure Security Center for IoT`, `Microsoft Cloud App Security`, `Microsoft Defender Advanced Threat Protection` and `Office 365 Advanced Threat Protection`.
        #[builder(into)]
        pub product_filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Only create incidents from alerts when alert severity level is contained in this list. Possible values are `High`, `Medium`, `Low` and `Informational`.
        ///
        /// > **NOTE** At least one of the severity filters need to be set.
        #[builder(into)]
        pub severity_filters: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleMsSecurityIncidentResult {
        /// The GUID of the alert rule template which is used to create this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of this Sentinel MS Security Incident Alert Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel MS Security Incident Alert Rule.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Only create incidents when the alert display name doesn't contain text from this list.
        pub display_name_exclude_filters: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Only create incidents when the alert display name contain text from this list, leave empty to apply no filter.
        pub display_name_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Should this Sentinel MS Security Incident Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Log Analytics Workspace this Sentinel MS Security Incident Alert Rule belongs to. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Sentinel MS Security Incident Alert Rule. Changing this forces a new Sentinel MS Security Incident Alert Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Microsoft Security Service from where the alert will be generated. Possible values are `Azure Active Directory Identity Protection`, `Azure Advanced Threat Protection`, `Azure Security Center`, `Azure Security Center for IoT`, `Microsoft Cloud App Security`, `Microsoft Defender Advanced Threat Protection` and `Office 365 Advanced Threat Protection`.
        pub product_filter: pulumi_gestalt_rust::Output<String>,
        /// Only create incidents from alerts when alert severity level is contained in this list. Possible values are `High`, `Medium`, `Low` and `Informational`.
        ///
        /// > **NOTE** At least one of the severity filters need to be set.
        pub severity_filters: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertRuleMsSecurityIncidentArgs,
    ) -> AlertRuleMsSecurityIncidentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alert_rule_template_guid_binding = args
            .alert_rule_template_guid
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let display_name_exclude_filters_binding = args
            .display_name_exclude_filters
            .get_output(context);
        let display_name_filters_binding = args.display_name_filters.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let product_filter_binding = args.product_filter.get_output(context);
        let severity_filters_binding = args.severity_filters.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleMsSecurityIncident:AlertRuleMsSecurityIncident"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertRuleTemplateGuid".into(),
                    value: &alert_rule_template_guid_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayNameExcludeFilters".into(),
                    value: &display_name_exclude_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayNameFilters".into(),
                    value: &display_name_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productFilter".into(),
                    value: &product_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "severityFilters".into(),
                    value: &severity_filters_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertRuleMsSecurityIncidentResult {
            alert_rule_template_guid: o.get_field("alertRuleTemplateGuid"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            display_name_exclude_filters: o.get_field("displayNameExcludeFilters"),
            display_name_filters: o.get_field("displayNameFilters"),
            enabled: o.get_field("enabled"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            product_filter: o.get_field("productFilter"),
            severity_filters: o.get_field("severityFilters"),
        }
    }
}
