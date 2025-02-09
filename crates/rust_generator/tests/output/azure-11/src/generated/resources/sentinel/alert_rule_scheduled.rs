/// Manages a Sentinel Scheduled Alert Rule.
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
///     let exampleAlertRuleScheduled = alert_rule_scheduled::create(
///         "exampleAlertRuleScheduled",
///         AlertRuleScheduledArgs::builder()
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
/// Sentinel Scheduled Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleScheduled:AlertRuleScheduled example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/alertRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_rule_scheduled {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleScheduledArgs {
        /// An `alert_details_override` block as defined below.
        #[builder(into, default)]
        pub alert_details_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverride,
                >,
            >,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel Scheduled Alert Rule.
        #[builder(into, default)]
        pub alert_rule_template_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of string key-value pairs of columns to be attached to this Sentinel Scheduled Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        #[builder(into, default)]
        pub custom_details: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel Scheduled Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The friendly name of this Sentinel Scheduled Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Sentinel Scheduled Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        #[builder(into, default)]
        pub entity_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sentinel::AlertRuleScheduledEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        #[builder(into, default)]
        pub event_grouping: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::AlertRuleScheduledEventGrouping>,
        >,
        /// A `incident` block as defined below.
        #[builder(into, default)]
        pub incident: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sentinel::AlertRuleScheduledIncident>,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel Scheduled Alert Rule belongs to. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query of this Sentinel Scheduled Alert Rule.
        #[builder(into)]
        pub query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ISO 8601 timespan duration between two consecutive queries. Defaults to `PT5H`.
        #[builder(into, default)]
        pub query_frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query. For example, it can query the past 10 minutes of data, or the past 6 hours of data. Defaults to `PT5H`.
        ///
        /// > **NOTE** `query_period` must larger than or equal to `query_frequency`, which ensures there is no gaps in the overall query coverage.
        #[builder(into, default)]
        pub query_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        #[builder(into, default)]
        pub sentinel_entity_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledSentinelEntityMapping,
                >,
            >,
        >,
        /// The alert severity of this Sentinel Scheduled Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        #[builder(into)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        ///
        /// > **NOTE** `suppression_duration` must larger than or equal to `query_frequency`, otherwise the suppression has no actual effect since no query will happen during the suppression duration.
        #[builder(into, default)]
        pub suppression_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Sentinel Scheduled Alert Rulea stop running query after alert is generated? Defaults to `false`.
        #[builder(into, default)]
        pub suppression_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `ImpairProcessControl`, `InhibitResponseFunction`, `Impact`, `InitialAccess`, `LateralMovement`, `Persistence`, `PrivilegeEscalation`, `PreAttack`, `Reconnaissance` and `ResourceDevelopment`.
        #[builder(into, default)]
        pub tactics: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        #[builder(into, default)]
        pub techniques: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule. Possible values are `Equal`, `GreaterThan`, `LessThan`, `NotEqual`. Defaults to `GreaterThan`.
        #[builder(into, default)]
        pub trigger_operator: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule. Defaults to `0`.
        #[builder(into, default)]
        pub trigger_threshold: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleScheduledResult {
        /// An `alert_details_override` block as defined below.
        pub alert_details_overrides: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverride,
                >,
            >,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel Scheduled Alert Rule.
        pub alert_rule_template_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel Scheduled Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        pub custom_details: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel Scheduled Alert Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel Scheduled Alert Rule.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Should the Sentinel Scheduled Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        pub entity_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleScheduledEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        pub event_grouping: pulumi_gestalt_rust::Output<
            Option<super::super::types::sentinel::AlertRuleScheduledEventGrouping>,
        >,
        /// A `incident` block as defined below.
        pub incident: pulumi_gestalt_rust::Output<
            super::super::types::sentinel::AlertRuleScheduledIncident,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel Scheduled Alert Rule belongs to. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The query of this Sentinel Scheduled Alert Rule.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// The ISO 8601 timespan duration between two consecutive queries. Defaults to `PT5H`.
        pub query_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query. For example, it can query the past 10 minutes of data, or the past 6 hours of data. Defaults to `PT5H`.
        ///
        /// > **NOTE** `query_period` must larger than or equal to `query_frequency`, which ensures there is no gaps in the overall query coverage.
        pub query_period: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        pub sentinel_entity_mappings: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledSentinelEntityMapping,
                >,
            >,
        >,
        /// The alert severity of this Sentinel Scheduled Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        pub severity: pulumi_gestalt_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        ///
        /// > **NOTE** `suppression_duration` must larger than or equal to `query_frequency`, otherwise the suppression has no actual effect since no query will happen during the suppression duration.
        pub suppression_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Sentinel Scheduled Alert Rulea stop running query after alert is generated? Defaults to `false`.
        pub suppression_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `ImpairProcessControl`, `InhibitResponseFunction`, `Impact`, `InitialAccess`, `LateralMovement`, `Persistence`, `PrivilegeEscalation`, `PreAttack`, `Reconnaissance` and `ResourceDevelopment`.
        pub tactics: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule. Possible values are `Equal`, `GreaterThan`, `LessThan`, `NotEqual`. Defaults to `GreaterThan`.
        pub trigger_operator: pulumi_gestalt_rust::Output<Option<String>>,
        /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule. Defaults to `0`.
        pub trigger_threshold: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AlertRuleScheduledArgs,
    ) -> AlertRuleScheduledResult {
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
        let query_frequency_binding_1 = args.query_frequency.get_output(context);
        let query_frequency_binding = query_frequency_binding_1.get_inner();
        let query_period_binding_1 = args.query_period.get_output(context);
        let query_period_binding = query_period_binding_1.get_inner();
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
        let trigger_operator_binding_1 = args.trigger_operator.get_output(context);
        let trigger_operator_binding = trigger_operator_binding_1.get_inner();
        let trigger_threshold_binding_1 = args.trigger_threshold.get_output(context);
        let trigger_threshold_binding = trigger_threshold_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleScheduled:AlertRuleScheduled".into(),
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
                    name: "queryFrequency".into(),
                    value: &query_frequency_binding,
                },
                register_interface::ObjectField {
                    name: "queryPeriod".into(),
                    value: &query_period_binding,
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
                register_interface::ObjectField {
                    name: "triggerOperator".into(),
                    value: &trigger_operator_binding,
                },
                register_interface::ObjectField {
                    name: "triggerThreshold".into(),
                    value: &trigger_threshold_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertRuleScheduledResult {
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
            query_frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryFrequency"),
            ),
            query_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryPeriod"),
            ),
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
            trigger_operator: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerOperator"),
            ),
            trigger_threshold: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerThreshold"),
            ),
        }
    }
}
