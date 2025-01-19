/// Manages a Sentinel Scheduled Alert Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod alert_rule_scheduled {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleScheduledArgs {
        /// An `alert_details_override` block as defined below.
        #[builder(into, default)]
        pub alert_details_overrides: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverride,
                >,
            >,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel Scheduled Alert Rule.
        #[builder(into, default)]
        pub alert_rule_template_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel Scheduled Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        #[builder(into, default)]
        pub custom_details: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel Scheduled Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel Scheduled Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Sentinel Scheduled Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        #[builder(into, default)]
        pub entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleScheduledEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        #[builder(into, default)]
        pub event_grouping: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::AlertRuleScheduledEventGrouping>,
        >,
        /// A `incident` block as defined below.
        #[builder(into, default)]
        pub incident: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::AlertRuleScheduledIncident>,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel Scheduled Alert Rule belongs to. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The query of this Sentinel Scheduled Alert Rule.
        #[builder(into)]
        pub query: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration between two consecutive queries. Defaults to `PT5H`.
        #[builder(into, default)]
        pub query_frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query. For example, it can query the past 10 minutes of data, or the past 6 hours of data. Defaults to `PT5H`.
        ///
        /// > **NOTE** `query_period` must larger than or equal to `query_frequency`, which ensures there is no gaps in the overall query coverage.
        #[builder(into, default)]
        pub query_period: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        #[builder(into, default)]
        pub sentinel_entity_mappings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledSentinelEntityMapping,
                >,
            >,
        >,
        /// The alert severity of this Sentinel Scheduled Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        #[builder(into)]
        pub severity: pulumi_wasm_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        ///
        /// > **NOTE** `suppression_duration` must larger than or equal to `query_frequency`, otherwise the suppression has no actual effect since no query will happen during the suppression duration.
        #[builder(into, default)]
        pub suppression_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Sentinel Scheduled Alert Rulea stop running query after alert is generated? Defaults to `false`.
        #[builder(into, default)]
        pub suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `ImpairProcessControl`, `InhibitResponseFunction`, `Impact`, `InitialAccess`, `LateralMovement`, `Persistence`, `PrivilegeEscalation`, `PreAttack`, `Reconnaissance` and `ResourceDevelopment`.
        #[builder(into, default)]
        pub tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        #[builder(into, default)]
        pub techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule. Possible values are `Equal`, `GreaterThan`, `LessThan`, `NotEqual`. Defaults to `GreaterThan`.
        #[builder(into, default)]
        pub trigger_operator: pulumi_wasm_rust::Output<Option<String>>,
        /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule. Defaults to `0`.
        #[builder(into, default)]
        pub trigger_threshold: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleScheduledResult {
        /// An `alert_details_override` block as defined below.
        pub alert_details_overrides: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverride,
                >,
            >,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel Scheduled Alert Rule.
        pub alert_rule_template_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel Scheduled Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        pub custom_details: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel Scheduled Alert Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel Scheduled Alert Rule.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Sentinel Scheduled Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        pub entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleScheduledEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        pub event_grouping: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::AlertRuleScheduledEventGrouping>,
        >,
        /// A `incident` block as defined below.
        pub incident: pulumi_wasm_rust::Output<
            super::super::types::sentinel::AlertRuleScheduledIncident,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel Scheduled Alert Rule belongs to. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Scheduled Alert Rule. Changing this forces a new Sentinel Scheduled Alert Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The query of this Sentinel Scheduled Alert Rule.
        pub query: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration between two consecutive queries. Defaults to `PT5H`.
        pub query_frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query. For example, it can query the past 10 minutes of data, or the past 6 hours of data. Defaults to `PT5H`.
        ///
        /// > **NOTE** `query_period` must larger than or equal to `query_frequency`, which ensures there is no gaps in the overall query coverage.
        pub query_period: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        pub sentinel_entity_mappings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleScheduledSentinelEntityMapping,
                >,
            >,
        >,
        /// The alert severity of this Sentinel Scheduled Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        pub severity: pulumi_wasm_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        ///
        /// > **NOTE** `suppression_duration` must larger than or equal to `query_frequency`, otherwise the suppression has no actual effect since no query will happen during the suppression duration.
        pub suppression_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Sentinel Scheduled Alert Rulea stop running query after alert is generated? Defaults to `false`.
        pub suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `ImpairProcessControl`, `InhibitResponseFunction`, `Impact`, `InitialAccess`, `LateralMovement`, `Persistence`, `PrivilegeEscalation`, `PreAttack`, `Reconnaissance` and `ResourceDevelopment`.
        pub tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule. Possible values are `Equal`, `GreaterThan`, `LessThan`, `NotEqual`. Defaults to `GreaterThan`.
        pub trigger_operator: pulumi_wasm_rust::Output<Option<String>>,
        /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule. Defaults to `0`.
        pub trigger_threshold: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AlertRuleScheduledArgs) -> AlertRuleScheduledResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alert_details_overrides_binding = args.alert_details_overrides.get_inner();
        let alert_rule_template_guid_binding = args.alert_rule_template_guid.get_inner();
        let alert_rule_template_version_binding = args
            .alert_rule_template_version
            .get_inner();
        let custom_details_binding = args.custom_details.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let entity_mappings_binding = args.entity_mappings.get_inner();
        let event_grouping_binding = args.event_grouping.get_inner();
        let incident_binding = args.incident.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let query_binding = args.query.get_inner();
        let query_frequency_binding = args.query_frequency.get_inner();
        let query_period_binding = args.query_period.get_inner();
        let sentinel_entity_mappings_binding = args.sentinel_entity_mappings.get_inner();
        let severity_binding = args.severity.get_inner();
        let suppression_duration_binding = args.suppression_duration.get_inner();
        let suppression_enabled_binding = args.suppression_enabled.get_inner();
        let tactics_binding = args.tactics.get_inner();
        let techniques_binding = args.techniques.get_inner();
        let trigger_operator_binding = args.trigger_operator.get_inner();
        let trigger_threshold_binding = args.trigger_threshold.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alertDetailsOverrides".into(),
                },
                register_interface::ResultField {
                    name: "alertRuleTemplateGuid".into(),
                },
                register_interface::ResultField {
                    name: "alertRuleTemplateVersion".into(),
                },
                register_interface::ResultField {
                    name: "customDetails".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "entityMappings".into(),
                },
                register_interface::ResultField {
                    name: "eventGrouping".into(),
                },
                register_interface::ResultField {
                    name: "incident".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "query".into(),
                },
                register_interface::ResultField {
                    name: "queryFrequency".into(),
                },
                register_interface::ResultField {
                    name: "queryPeriod".into(),
                },
                register_interface::ResultField {
                    name: "sentinelEntityMappings".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "suppressionDuration".into(),
                },
                register_interface::ResultField {
                    name: "suppressionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "tactics".into(),
                },
                register_interface::ResultField {
                    name: "techniques".into(),
                },
                register_interface::ResultField {
                    name: "triggerOperator".into(),
                },
                register_interface::ResultField {
                    name: "triggerThreshold".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleScheduledResult {
            alert_details_overrides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertDetailsOverrides").unwrap(),
            ),
            alert_rule_template_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertRuleTemplateGuid").unwrap(),
            ),
            alert_rule_template_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alertRuleTemplateVersion").unwrap(),
            ),
            custom_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDetails").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            entity_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entityMappings").unwrap(),
            ),
            event_grouping: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventGrouping").unwrap(),
            ),
            incident: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("incident").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("query").unwrap(),
            ),
            query_frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryFrequency").unwrap(),
            ),
            query_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryPeriod").unwrap(),
            ),
            sentinel_entity_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sentinelEntityMappings").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            suppression_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressionDuration").unwrap(),
            ),
            suppression_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressionEnabled").unwrap(),
            ),
            tactics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tactics").unwrap(),
            ),
            techniques: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("techniques").unwrap(),
            ),
            trigger_operator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerOperator").unwrap(),
            ),
            trigger_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerThreshold").unwrap(),
            ),
        }
    }
}
