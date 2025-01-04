/// Manages a Sentinel NRT Alert Rule.
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
pub mod alert_rule_nrt {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleNrtArgs {
        /// An `alert_details_override` block as defined below.
        #[builder(into, default)]
        pub alert_details_overrides: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtAlertDetailsOverride>>,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub alert_rule_template_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel NRT Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        #[builder(into, default)]
        pub custom_details: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel NRT Alert Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel NRT Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Sentinel NRT Alert Rule be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        #[builder(into, default)]
        pub entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        #[builder(into)]
        pub event_grouping: pulumi_wasm_rust::Output<
            super::super::types::sentinel::AlertRuleNrtEventGrouping,
        >,
        /// A `incident` block as defined below.
        #[builder(into, default)]
        pub incident: pulumi_wasm_rust::Output<
            Option<super::super::types::sentinel::AlertRuleNrtIncident>,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel NRT Alert Rule belongs to. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The query of this Sentinel NRT Alert Rule.
        #[builder(into)]
        pub query: pulumi_wasm_rust::Output<String>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        #[builder(into, default)]
        pub sentinel_entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtSentinelEntityMapping>>,
        >,
        /// The alert severity of this Sentinel NRT Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        #[builder(into)]
        pub severity: pulumi_wasm_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        #[builder(into, default)]
        pub suppression_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Sentinel NRT Alert Rulea stop running query after alert is generated? Defaults to `false`.
        #[builder(into, default)]
        pub suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `Impact`, `ImpairProcessControl`, `InhibitResponseFunction`, `InitialAccess`, `LateralMovement`, `Persistence`, `PreAttack`, `PrivilegeEscalation`, `Reconnaissance` and `ResourceDevelopment`.
        #[builder(into, default)]
        pub tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        #[builder(into, default)]
        pub techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleNrtResult {
        /// An `alert_details_override` block as defined below.
        pub alert_details_overrides: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtAlertDetailsOverride>>,
        >,
        /// The GUID of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub alert_rule_template_guid: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the alert rule template which is used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub alert_rule_template_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of string key-value pairs of columns to be attached to this Sentinel NRT Alert Rule. The key will appear as the field name in alerts and the value is the event parameter you wish to surface in the alerts.
        pub custom_details: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The description of this Sentinel NRT Alert Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of this Sentinel NRT Alert Rule.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Sentinel NRT Alert Rule be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of `entity_mapping` blocks as defined below.
        pub entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtEntityMapping>>,
        >,
        /// A `event_grouping` block as defined below.
        pub event_grouping: pulumi_wasm_rust::Output<
            super::super::types::sentinel::AlertRuleNrtEventGrouping,
        >,
        /// A `incident` block as defined below.
        pub incident: pulumi_wasm_rust::Output<
            super::super::types::sentinel::AlertRuleNrtIncident,
        >,
        /// The ID of the Log Analytics Workspace this Sentinel NRT Alert Rule belongs to. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel NRT Alert Rule. Changing this forces a new Sentinel NRT Alert Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The query of this Sentinel NRT Alert Rule.
        pub query: pulumi_wasm_rust::Output<String>,
        /// A list of `sentinel_entity_mapping` blocks as defined below.
        ///
        /// > **NOTE:** `entity_mapping` and `sentinel_entity_mapping` together can't exceed 5.
        pub sentinel_entity_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sentinel::AlertRuleNrtSentinelEntityMapping>>,
        >,
        /// The alert severity of this Sentinel NRT Alert Rule. Possible values are `High`, `Medium`, `Low` and `Informational`.
        pub severity: pulumi_wasm_rust::Output<String>,
        /// If `suppression_enabled` is `true`, this is ISO 8601 timespan duration, which specifies the amount of time the query should stop running after alert is generated. Defaults to `PT5H`.
        pub suppression_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Sentinel NRT Alert Rulea stop running query after alert is generated? Defaults to `false`.
        pub suppression_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of categories of attacks by which to classify the rule. Possible values are `Collection`, `CommandAndControl`, `CredentialAccess`, `DefenseEvasion`, `Discovery`, `Execution`, `Exfiltration`, `Impact`, `ImpairProcessControl`, `InhibitResponseFunction`, `InitialAccess`, `LateralMovement`, `Persistence`, `PreAttack`, `PrivilegeEscalation`, `Reconnaissance` and `ResourceDevelopment`.
        pub tactics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AlertRuleNrtArgs) -> AlertRuleNrtResult {
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
        let sentinel_entity_mappings_binding = args.sentinel_entity_mappings.get_inner();
        let severity_binding = args.severity.get_inner();
        let suppression_duration_binding = args.suppression_duration.get_inner();
        let suppression_enabled_binding = args.suppression_enabled.get_inner();
        let tactics_binding = args.tactics.get_inner();
        let techniques_binding = args.techniques.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleNrt:AlertRuleNrt".into(),
            name: name.to_string(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleNrtResult {
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
        }
    }
}
