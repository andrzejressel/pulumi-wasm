/// Manages an AlertingAction Scheduled Query Rules Version 2 resource within Azure Monitor
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: example-mag
///       resourceGroupName: ${example.name}
///       shortName: test mag
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-uai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleInsights.id}
///       roleDefinitionName: Reader
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   exampleScheduledQueryRulesAlertV2:
///     type: azure:monitoring:ScheduledQueryRulesAlertV2
///     name: example
///     properties:
///       name: example-msqrv2
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       evaluationFrequency: PT10M
///       windowDuration: PT10M
///       scopes: ${exampleInsights.id}
///       severity: 4
///       criterias:
///         - query: |
///             requests
///               | summarize CountByCountry=count() by client_CountryOrRegion
///           timeAggregationMethod: Maximum
///           threshold: 17.5
///           operator: LessThan
///           resourceIdColumn: client_CountryOrRegion
///           metricMeasureColumn: CountByCountry
///           dimensions:
///             - name: client_CountryOrRegion
///               operator: Exclude
///               values:
///                 - '123'
///           failingPeriods:
///             minimumFailingPeriodsToTriggerAlert: 1
///             numberOfEvaluationPeriods: 1
///       autoMitigationEnabled: true
///       workspaceAlertsStorageEnabled: false
///       description: example sqr
///       displayName: example-sqr
///       enabled: true
///       queryTimeRangeOverride: PT1H
///       skipQueryValidation: true
///       action:
///         actionGroups:
///           - ${exampleActionGroup.id}
///         customProperties:
///           key: value
///           key2: value2
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       tags:
///         key: value
///         key2: value2
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// ```
///
/// ## Import
///
/// Monitor Scheduled Query Rule Alert can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/scheduledQueryRulesAlertV2:ScheduledQueryRulesAlertV2 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Insights/scheduledQueryRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod scheduled_query_rules_alert_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertV2Args {
        /// An `action` block as defined below.
        #[builder(into, default)]
        pub action: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::ScheduledQueryRulesAlertV2Action>,
        >,
        /// Specifies the flag that indicates whether the alert should be automatically resolved or not. Value should be `true` or `false`. The default is `false`.
        #[builder(into, default)]
        pub auto_mitigation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `criteria` block as defined below.
        #[builder(into)]
        pub criterias: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::monitoring::ScheduledQueryRulesAlertV2Criteria>,
        >,
        /// Specifies the description of the scheduled query rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the display name of the alert rule.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the flag which indicates whether this scheduled query rule is enabled. Value should be `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// How often the scheduled query rule is evaluated, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D`.
        ///
        /// > **Note** `evaluation_frequency` cannot be greater than the query look back which is `window_duration`*`number_of_evaluation_periods`.
        ///
        /// > **Note** `evaluation_frequency` cannot be greater than the `mute_actions_after_alert_duration`.
        #[builder(into)]
        pub evaluation_frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::ScheduledQueryRulesAlertV2Identity>,
        >,
        /// Specifies the Azure Region where the Monitor Scheduled Query Rule should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mute actions for the chosen period of time in ISO 8601 duration format after the alert is fired. Possible values are `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D` and `P2D`.
        ///
        /// > **Note** `auto_mitigation_enabled` and `mute_actions_after_alert_duration` are mutually exclusive and cannot both be set.
        #[builder(into, default)]
        pub mute_actions_after_alert_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name which should be used for this Monitor Scheduled Query Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set this if the alert evaluation period is different from the query time range. If not specified, the value is `window_duration`*`number_of_evaluation_periods`. Possible values are `PT5M`, `PT10M`, `PT15M`, `PT20M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D` and `P2D`.
        ///
        /// > **Note** `query_time_range_override` cannot be less than the query look back which is `window_duration`*`number_of_evaluation_periods`.
        #[builder(into, default)]
        pub query_time_range_override: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Resource Group where the Monitor Scheduled Query Rule should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the list of resource IDs that this scheduled query rule is scoped to. Changing this forces a new resource to be created. Currently, the API supports exactly 1 resource ID in the scopes list.
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Severity of the alert. Should be an integer between 0 and 4. Value of 0 is severest.
        #[builder(into)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the flag which indicates whether the provided query should be validated or not. The default is false.
        #[builder(into, default)]
        pub skip_query_validation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags which should be assigned to the Monitor Scheduled Query Rule.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of resource type of the target resource(s) on which the alert is created/updated. For example if the scope is a resource group and targetResourceTypes is `Microsoft.Compute/virtualMachines`, then a different alert will be fired for each virtual machine in the resource group which meet the alert criteria.
        #[builder(into, default)]
        pub target_resource_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the period of time in ISO 8601 duration format on which the Scheduled Query Rule will be executed (bin size). If `evaluation_frequency` is `PT1M`, possible values are `PT1M`, `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, and `PT6H`. Otherwise, possible values are `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D`, and `P2D`.
        #[builder(into)]
        pub window_duration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the flag which indicates whether this scheduled query rule check if storage is configured. Value should be `true` or `false`. The default is `false`.
        #[builder(into, default)]
        pub workspace_alerts_storage_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertV2Result {
        /// An `action` block as defined below.
        pub action: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::ScheduledQueryRulesAlertV2Action>,
        >,
        /// Specifies the flag that indicates whether the alert should be automatically resolved or not. Value should be `true` or `false`. The default is `false`.
        pub auto_mitigation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The api-version used when creating this alert rule.
        pub created_with_api_version: pulumi_gestalt_rust::Output<String>,
        /// A `criteria` block as defined below.
        pub criterias: pulumi_gestalt_rust::Output<
            Vec<super::super::types::monitoring::ScheduledQueryRulesAlertV2Criteria>,
        >,
        /// Specifies the description of the scheduled query rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the display name of the alert rule.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the flag which indicates whether this scheduled query rule is enabled. Value should be `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// How often the scheduled query rule is evaluated, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D`.
        ///
        /// > **Note** `evaluation_frequency` cannot be greater than the query look back which is `window_duration`*`number_of_evaluation_periods`.
        ///
        /// > **Note** `evaluation_frequency` cannot be greater than the `mute_actions_after_alert_duration`.
        pub evaluation_frequency: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::ScheduledQueryRulesAlertV2Identity>,
        >,
        /// True if this alert rule is a legacy Log Analytic Rule.
        pub is_a_legacy_log_analytics_rule: pulumi_gestalt_rust::Output<bool>,
        /// The flag indicates whether this Scheduled Query Rule has been configured to be stored in the customer's storage.
        pub is_workspace_alerts_storage_configured: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the Azure Region where the Monitor Scheduled Query Rule should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Mute actions for the chosen period of time in ISO 8601 duration format after the alert is fired. Possible values are `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D` and `P2D`.
        ///
        /// > **Note** `auto_mitigation_enabled` and `mute_actions_after_alert_duration` are mutually exclusive and cannot both be set.
        pub mute_actions_after_alert_duration: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the name which should be used for this Monitor Scheduled Query Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set this if the alert evaluation period is different from the query time range. If not specified, the value is `window_duration`*`number_of_evaluation_periods`. Possible values are `PT5M`, `PT10M`, `PT15M`, `PT20M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D` and `P2D`.
        ///
        /// > **Note** `query_time_range_override` cannot be less than the query look back which is `window_duration`*`number_of_evaluation_periods`.
        pub query_time_range_override: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group where the Monitor Scheduled Query Rule should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the list of resource IDs that this scheduled query rule is scoped to. Changing this forces a new resource to be created. Currently, the API supports exactly 1 resource ID in the scopes list.
        pub scopes: pulumi_gestalt_rust::Output<String>,
        /// Severity of the alert. Should be an integer between 0 and 4. Value of 0 is severest.
        pub severity: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the flag which indicates whether the provided query should be validated or not. The default is false.
        pub skip_query_validation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to the Monitor Scheduled Query Rule.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of resource type of the target resource(s) on which the alert is created/updated. For example if the scope is a resource group and targetResourceTypes is `Microsoft.Compute/virtualMachines`, then a different alert will be fired for each virtual machine in the resource group which meet the alert criteria.
        pub target_resource_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the period of time in ISO 8601 duration format on which the Scheduled Query Rule will be executed (bin size). If `evaluation_frequency` is `PT1M`, possible values are `PT1M`, `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, and `PT6H`. Otherwise, possible values are `PT5M`, `PT10M`, `PT15M`, `PT30M`, `PT45M`, `PT1H`, `PT2H`, `PT3H`, `PT4H`, `PT5H`, `PT6H`, `P1D`, and `P2D`.
        pub window_duration: pulumi_gestalt_rust::Output<String>,
        /// Specifies the flag which indicates whether this scheduled query rule check if storage is configured. Value should be `true` or `false`. The default is `false`.
        pub workspace_alerts_storage_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScheduledQueryRulesAlertV2Args,
    ) -> ScheduledQueryRulesAlertV2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let auto_mitigation_enabled_binding = args
            .auto_mitigation_enabled
            .get_output(context)
            .get_inner();
        let criterias_binding = args.criterias.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let evaluation_frequency_binding = args
            .evaluation_frequency
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mute_actions_after_alert_duration_binding = args
            .mute_actions_after_alert_duration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_time_range_override_binding = args
            .query_time_range_override
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let severity_binding = args.severity.get_output(context).get_inner();
        let skip_query_validation_binding = args
            .skip_query_validation
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_resource_types_binding = args
            .target_resource_types
            .get_output(context)
            .get_inner();
        let window_duration_binding = args
            .window_duration
            .get_output(context)
            .get_inner();
        let workspace_alerts_storage_enabled_binding = args
            .workspace_alerts_storage_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/scheduledQueryRulesAlertV2:ScheduledQueryRulesAlertV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "autoMitigationEnabled".into(),
                    value: &auto_mitigation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "criterias".into(),
                    value: &criterias_binding,
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
                    name: "evaluationFrequency".into(),
                    value: &evaluation_frequency_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "muteActionsAfterAlertDuration".into(),
                    value: &mute_actions_after_alert_duration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryTimeRangeOverride".into(),
                    value: &query_time_range_override_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "skipQueryValidation".into(),
                    value: &skip_query_validation_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceTypes".into(),
                    value: &target_resource_types_binding,
                },
                register_interface::ObjectField {
                    name: "windowDuration".into(),
                    value: &window_duration_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceAlertsStorageEnabled".into(),
                    value: &workspace_alerts_storage_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduledQueryRulesAlertV2Result {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            auto_mitigation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoMitigationEnabled"),
            ),
            created_with_api_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdWithApiVersion"),
            ),
            criterias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("criterias"),
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
            evaluation_frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("evaluationFrequency"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            is_a_legacy_log_analytics_rule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isALegacyLogAnalyticsRule"),
            ),
            is_workspace_alerts_storage_configured: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isWorkspaceAlertsStorageConfigured"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mute_actions_after_alert_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("muteActionsAfterAlertDuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_time_range_override: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryTimeRangeOverride"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopes"),
            ),
            severity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            skip_query_validation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipQueryValidation"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetResourceTypes"),
            ),
            window_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("windowDuration"),
            ),
            workspace_alerts_storage_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceAlertsStorageEnabled"),
            ),
        }
    }
}
