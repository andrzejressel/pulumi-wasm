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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertRuleScheduledArgs,
    ) -> AlertRuleScheduledResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alert_details_overrides_binding = args
            .alert_details_overrides
            .get_output(context);
        let alert_rule_template_guid_binding = args
            .alert_rule_template_guid
            .get_output(context);
        let alert_rule_template_version_binding = args
            .alert_rule_template_version
            .get_output(context);
        let custom_details_binding = args.custom_details.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let entity_mappings_binding = args.entity_mappings.get_output(context);
        let event_grouping_binding = args.event_grouping.get_output(context);
        let incident_binding = args.incident.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let query_binding = args.query.get_output(context);
        let query_frequency_binding = args.query_frequency.get_output(context);
        let query_period_binding = args.query_period.get_output(context);
        let sentinel_entity_mappings_binding = args
            .sentinel_entity_mappings
            .get_output(context);
        let severity_binding = args.severity.get_output(context);
        let suppression_duration_binding = args.suppression_duration.get_output(context);
        let suppression_enabled_binding = args.suppression_enabled.get_output(context);
        let tactics_binding = args.tactics.get_output(context);
        let techniques_binding = args.techniques.get_output(context);
        let trigger_operator_binding = args.trigger_operator.get_output(context);
        let trigger_threshold_binding = args.trigger_threshold.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleScheduled:AlertRuleScheduled".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertDetailsOverrides".into(),
                    value: alert_details_overrides_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertRuleTemplateGuid".into(),
                    value: alert_rule_template_guid_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alertRuleTemplateVersion".into(),
                    value: alert_rule_template_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDetails".into(),
                    value: custom_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entityMappings".into(),
                    value: entity_mappings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventGrouping".into(),
                    value: event_grouping_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "incident".into(),
                    value: incident_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "query".into(),
                    value: query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryFrequency".into(),
                    value: query_frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryPeriod".into(),
                    value: query_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sentinelEntityMappings".into(),
                    value: sentinel_entity_mappings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "severity".into(),
                    value: severity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suppressionDuration".into(),
                    value: suppression_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suppressionEnabled".into(),
                    value: suppression_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tactics".into(),
                    value: tactics_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "techniques".into(),
                    value: techniques_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerOperator".into(),
                    value: trigger_operator_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerThreshold".into(),
                    value: trigger_threshold_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertRuleScheduledResult {
            alert_details_overrides: o.get_field("alertDetailsOverrides"),
            alert_rule_template_guid: o.get_field("alertRuleTemplateGuid"),
            alert_rule_template_version: o.get_field("alertRuleTemplateVersion"),
            custom_details: o.get_field("customDetails"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            entity_mappings: o.get_field("entityMappings"),
            event_grouping: o.get_field("eventGrouping"),
            incident: o.get_field("incident"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
            query: o.get_field("query"),
            query_frequency: o.get_field("queryFrequency"),
            query_period: o.get_field("queryPeriod"),
            sentinel_entity_mappings: o.get_field("sentinelEntityMappings"),
            severity: o.get_field("severity"),
            suppression_duration: o.get_field("suppressionDuration"),
            suppression_enabled: o.get_field("suppressionEnabled"),
            tactics: o.get_field("tactics"),
            techniques: o.get_field("techniques"),
            trigger_operator: o.get_field("triggerOperator"),
            trigger_threshold: o.get_field("triggerThreshold"),
        }
    }
}
