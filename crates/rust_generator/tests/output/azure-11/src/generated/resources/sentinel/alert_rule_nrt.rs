/// Manages a Sentinel NRT Alert Rule.
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
///     let exampleAlertRuleNrt = alert_rule_nrt::create(
///         "exampleAlertRuleNrt",
///         AlertRuleNrtArgs::builder()
///             .display_name("example")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example")
///             .query(
///                 "AzureActivity |\n  where OperationName == \"Create or Update Virtual Machine\" or OperationName ==\"Create Deployment\" |\n  where ActivityStatus == \"Succeeded\" |\n  make-series dcount(ResourceId) default=0 on EventSubmissionTimestamp in range(ago(7d), now(), 1d) by Caller",
///             )
///             .severity("High")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("pergb2018")
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
/// Sentinel NRT Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleNrt:AlertRuleNrt example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_rule_nrt {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleNrtArgs {
        /// An `alert_details_override` block as defined below.
        #[builder(into, default)]
        pub alert_details_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtAlertDetailsOverride>>,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of string key-value pairs of columns to be attached to this Sentinel NRT Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        #[builder(into, default)]
        pub custom_details: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel NRT Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The friendly name of this Sentinel NRT Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Sentinel NRT Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        #[builder(into, default)]
        pub entity_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        #[builder(into)]
        pub event_grouping: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sentinel::AlertRuleNrtEventGrouping,
        >,
        /// A `incident` block as defined below.
        #[builder(into, default)]
        pub incident: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::AlertRuleNrtIncident>,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel NRT Alert Rule belongs to. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query of this Sentinel NRT Alert Rule.
        #[builder(into)]
        pub query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        #[builder(into, default)]
        pub sentinel_entity_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtSentinelEntityMapping>>,
        >,
        /// The alert severity of this Sentinel NRT Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        #[builder(into)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        #[builder(into, default)]
        pub suppression_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Sentinel NRT Alert Rulea stop running query after alert is generated? Defaults to `false`.
        #[builder(into, default)]
        pub suppression_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `Impact`, `ImpairProcessControl`, `InhibitResponseFunction`, `InitialAccess`, `LateralMovement`, `Persistence`, `PreAttack`, `PrivilegeEscalation`, `Reconnaissance` and `ResourceDevelopment`.
        #[builder(into, default)]
        pub tactics: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        #[builder(into, default)]
        pub techniques: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleNrtResult {
        /// An `alert_details_override` block as defined below.
        pub alert_details_overrides: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtAlertDetailsOverride>>,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub alert_rule_template_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel NRT Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        pub custom_details: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel NRT Alert Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel NRT Alert Rule.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Should the Sentinel NRT Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        pub entity_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        pub event_grouping: pulumi_gestalt_rust::Output<
            super::super::types::sentinel::AlertRuleNrtEventGrouping,
        >,
        /// A `incident` block as defined below.
        pub incident: pulumi_gestalt_rust::Output<
            super::super::types::sentinel::AlertRuleNrtIncident,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel NRT Alert Rule belongs to. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The query of this Sentinel NRT Alert Rule.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        pub sentinel_entity_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtSentinelEntityMapping>>,
        >,
        /// The alert severity of this Sentinel NRT Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        pub severity: pulumi_gestalt_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        pub suppression_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Sentinel NRT Alert Rulea stop running query after alert is generated? Defaults to `false`.
        pub suppression_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `Impact`, `ImpairProcessControl`, `InhibitResponseFunction`, `InitialAccess`, `LateralMovement`, `Persistence`, `PreAttack`, `PrivilegeEscalation`, `Reconnaissance` and `ResourceDevelopment`.
        pub tactics: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AlertRuleNrtArgs,
    ) -> AlertRuleNrtResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alert_details_overrides_binding_1 = args
            .alert_details_overrides
            .get_output(context);
        let alert_details_overrides_binding = alert_details_overrides_binding_1
            .get_inner();
        let alert_rule_template_guid_binding_1 = args
            .alert_rule_template_guid
            .get_output(context);
        let alert_rule_template_guid_binding = alert_rule_template_guid_binding_1
            .get_inner();
        let alert_rule_template_version_binding_1 = args
            .alert_rule_template_version
            .get_output(context);
        let alert_rule_template_version_binding = alert_rule_template_version_binding_1
            .get_inner();
        let custom_details_binding_1 = args.custom_details.get_output(context);
        let custom_details_binding = custom_details_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let entity_mappings_binding_1 = args.entity_mappings.get_output(context);
        let entity_mappings_binding = entity_mappings_binding_1.get_inner();
        let event_grouping_binding_1 = args.event_grouping.get_output(context);
        let event_grouping_binding = event_grouping_binding_1.get_inner();
        let incident_binding_1 = args.incident.get_output(context);
        let incident_binding = incident_binding_1.get_inner();
        let log_analytics_workspace_id_binding_1 = args
            .log_analytics_workspace_id
            .get_output(context);
        let log_analytics_workspace_id_binding = log_analytics_workspace_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let query_binding_1 = args.query.get_output(context);
        let query_binding = query_binding_1.get_inner();
        let sentinel_entity_mappings_binding_1 = args
            .sentinel_entity_mappings
            .get_output(context);
        let sentinel_entity_mappings_binding = sentinel_entity_mappings_binding_1
            .get_inner();
        let severity_binding_1 = args.severity.get_output(context);
        let severity_binding = severity_binding_1.get_inner();
        let suppression_duration_binding_1 = args
            .suppression_duration
            .get_output(context);
        let suppression_duration_binding = suppression_duration_binding_1.get_inner();
        let suppression_enabled_binding_1 = args.suppression_enabled.get_output(context);
        let suppression_enabled_binding = suppression_enabled_binding_1.get_inner();
        let tactics_binding_1 = args.tactics.get_output(context);
        let tactics_binding = tactics_binding_1.get_inner();
        let techniques_binding_1 = args.techniques.get_output(context);
        let techniques_binding = techniques_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleNrt:AlertRuleNrt".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alertDetailsOverrides".into(),
                    value: &alert_details_overrides_binding,
                },
                register_interface::ObjectField {
                    name: "alertRuleTemplateGuid".into(),
                    value: &alert_rule_template_guid_binding,
                },
                register_interface::ObjectField {
                    name: "alertRuleTemplateVersion".into(),
                    value: &alert_rule_template_version_binding,
                },
                register_interface::ObjectField {
                    name: "customDetails".into(),
                    value: &custom_details_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "entityMappings".into(),
                    value: &entity_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "eventGrouping".into(),
                    value: &event_grouping_binding,
                },
                register_interface::ObjectField {
                    name: "incident".into(),
                    value: &incident_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
                register_interface::ObjectField {
                    name: "sentinelEntityMappings".into(),
                    value: &sentinel_entity_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "suppressionDuration".into(),
                    value: &suppression_duration_binding,
                },
                register_interface::ObjectField {
                    name: "suppressionEnabled".into(),
                    value: &suppression_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tactics".into(),
                    value: &tactics_binding,
                },
                register_interface::ObjectField {
                    name: "techniques".into(),
                    value: &techniques_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertRuleNrtResult {
            alert_details_overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alertDetailsOverrides"),
            ),
            alert_rule_template_guid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alertRuleTemplateGuid"),
            ),
            alert_rule_template_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alertRuleTemplateVersion"),
            ),
            custom_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDetails"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            entity_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entityMappings"),
            ),
            event_grouping: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventGrouping"),
            ),
            incident: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("incident"),
            ),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query: pulumi_gestalt_rust::__private::into_domain(o.extract_field("query")),
            sentinel_entity_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sentinelEntityMappings"),
            ),
            severity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            suppression_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("suppressionDuration"),
            ),
            suppression_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("suppressionEnabled"),
            ),
            tactics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tactics"),
            ),
            techniques: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("techniques"),
            ),
        }
    }
}
