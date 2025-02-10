/// Manages a Duplicated Anomaly Alert Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: example-law
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: PerGB2018
///   exampleLogAnalyticsWorkspaceOnboarding:
///     type: azure:sentinel:LogAnalyticsWorkspaceOnboarding
///     name: example
///     properties:
///       workspaceId: ${exampleAnalyticsWorkspace.id}
///       customerManagedKeyEnabled: false
///   exampleAlertRuleAnomalyDuplicate:
///     type: azure:sentinel:AlertRuleAnomalyDuplicate
///     name: example
///     properties:
///       displayName: example duplicated UEBA Anomalous Sign In
///       logAnalyticsWorkspaceId: ${exampleAnalyticsWorkspace.id}
///       builtInRuleId: ${example.id}
///       enabled: true
///       mode: Flighting
///       thresholdObservations:
///         - name: Anomaly score threshold
///           value: '0.6'
/// variables:
///   example:
///     fn::invoke:
///       function: azure:sentinel:getAlertRuleAnomaly
///       arguments:
///         logAnalyticsWorkspaceId: ${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}
///         displayName: UEBA Anomalous Sign In
/// ```
///
/// ## Import
///
/// Built In Anomaly Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleAnomalyDuplicate:AlertRuleAnomalyDuplicate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/securityMLAnalyticsSettings/setting1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_rule_anomaly_duplicate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyDuplicateArgs {
        /// The ID of the built-in Anomaly Alert Rule. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        #[builder(into)]
        pub built_in_rule_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Display Name of the built-in Anomaly Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Duplicated Anomaly Alert Rule be enabled?
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// mode of the Duplicated Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        #[builder(into, default)]
        pub multi_select_observations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicateMultiSelectObservation,
                >,
            >,
        >,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        #[builder(into, default)]
        pub prioritized_exclude_observations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicatePrioritizedExcludeObservation,
                >,
            >,
        >,
        /// A list of `single_select_observation` blocks as defined below.
        #[builder(into, default)]
        pub single_select_observations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicateSingleSelectObservation,
                >,
            >,
        >,
        /// A list of `threshold_observation` blocks as defined below.
        ///
        /// > **NOTE:** un-specified `multi_select_observation`, `single_select_observation`, `prioritized_exclude_observation` and `threshold_observation` will be inherited from the built-in Anomaly Alert Rule.
        #[builder(into, default)]
        pub threshold_observations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicateThresholdObservation,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyDuplicateResult {
        /// The version of the Anomaly Security ML Analytics Settings.
        pub anomaly_settings_version: pulumi_gestalt_rust::Output<i32>,
        /// The anomaly version of the Anomaly Alert Rule.
        pub anomaly_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the built-in Anomaly Alert Rule. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        pub built_in_rule_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the Anomaly Alert Rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The Display Name of the built-in Anomaly Alert Rule.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Should the Duplicated Anomaly Alert Rule be enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The frequency the Anomaly Alert Rule will be run, such as "P1D".
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// Whether the current settings of the Anomaly Alert Rule equals default settings.
        pub is_default_settings: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// mode of the Duplicated Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        pub multi_select_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateMultiSelectObservation,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        pub prioritized_exclude_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicatePrioritizedExcludeObservation,
            >,
        >,
        /// A `required_data_connector` block as defined below.
        pub required_data_connectors: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateRequiredDataConnector,
            >,
        >,
        /// The ID of the anomaly settings definition Id.
        pub settings_definition_id: pulumi_gestalt_rust::Output<String>,
        /// A list of `single_select_observation` blocks as defined below.
        pub single_select_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateSingleSelectObservation,
            >,
        >,
        /// A list of categories of attacks by which to classify the rule.
        pub tactics: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of `threshold_observation` blocks as defined below.
        ///
        /// > **NOTE:** un-specified `multi_select_observation`, `single_select_observation`, `prioritized_exclude_observation` and `threshold_observation` will be inherited from the built-in Anomaly Alert Rule.
        pub threshold_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateThresholdObservation,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertRuleAnomalyDuplicateArgs,
    ) -> AlertRuleAnomalyDuplicateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let built_in_rule_id_binding = args.built_in_rule_id.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let mode_binding = args.mode.get_output(context);
        let multi_select_observations_binding = args
            .multi_select_observations
            .get_output(context);
        let prioritized_exclude_observations_binding = args
            .prioritized_exclude_observations
            .get_output(context);
        let single_select_observations_binding = args
            .single_select_observations
            .get_output(context);
        let threshold_observations_binding = args
            .threshold_observations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleAnomalyDuplicate:AlertRuleAnomalyDuplicate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "builtInRuleId".into(),
                    value: built_in_rule_id_binding.get_id(),
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
                    name: "logAnalyticsWorkspaceId".into(),
                    value: log_analytics_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiSelectObservations".into(),
                    value: multi_select_observations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prioritizedExcludeObservations".into(),
                    value: prioritized_exclude_observations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singleSelectObservations".into(),
                    value: single_select_observations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thresholdObservations".into(),
                    value: threshold_observations_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertRuleAnomalyDuplicateResult {
            anomaly_settings_version: o.get_field("anomalySettingsVersion"),
            anomaly_version: o.get_field("anomalyVersion"),
            built_in_rule_id: o.get_field("builtInRuleId"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            frequency: o.get_field("frequency"),
            is_default_settings: o.get_field("isDefaultSettings"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            mode: o.get_field("mode"),
            multi_select_observations: o.get_field("multiSelectObservations"),
            name: o.get_field("name"),
            prioritized_exclude_observations: o
                .get_field("prioritizedExcludeObservations"),
            required_data_connectors: o.get_field("requiredDataConnectors"),
            settings_definition_id: o.get_field("settingsDefinitionId"),
            single_select_observations: o.get_field("singleSelectObservations"),
            tactics: o.get_field("tactics"),
            techniques: o.get_field("techniques"),
            threshold_observations: o.get_field("thresholdObservations"),
        }
    }
}
