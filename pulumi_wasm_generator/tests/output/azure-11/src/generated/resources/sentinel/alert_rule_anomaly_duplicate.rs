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
pub mod alert_rule_anomaly_duplicate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyDuplicateArgs {
        /// The ID of the built-in Anomaly Alert Rule. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        #[builder(into)]
        pub built_in_rule_id: pulumi_wasm_rust::Output<String>,
        /// The Display Name of the built-in Anomaly Alert Rule.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Duplicated Anomaly Alert Rule be enabled?
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// mode of the Duplicated Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        #[builder(into, default)]
        pub multi_select_observations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicateMultiSelectObservation,
                >,
            >,
        >,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        #[builder(into, default)]
        pub prioritized_exclude_observations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::sentinel::AlertRuleAnomalyDuplicatePrioritizedExcludeObservation,
                >,
            >,
        >,
        /// A list of `single_select_observation` blocks as defined below.
        #[builder(into, default)]
        pub single_select_observations: pulumi_wasm_rust::Output<
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
        pub threshold_observations: pulumi_wasm_rust::Output<
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
        pub anomaly_settings_version: pulumi_wasm_rust::Output<i32>,
        /// The anomaly version of the Anomaly Alert Rule.
        pub anomaly_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the built-in Anomaly Alert Rule. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        pub built_in_rule_id: pulumi_wasm_rust::Output<String>,
        /// The description of the Anomaly Alert Rule.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The Display Name of the built-in Anomaly Alert Rule.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Duplicated Anomaly Alert Rule be enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The frequency the Anomaly Alert Rule will be run, such as "P1D".
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Whether the current settings of the Anomaly Alert Rule equals default settings.
        pub is_default_settings: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Duplicated Anomaly Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// mode of the Duplicated Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        pub multi_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateMultiSelectObservation,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        pub prioritized_exclude_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicatePrioritizedExcludeObservation,
            >,
        >,
        /// A `required_data_connector` block as defined below.
        pub required_data_connectors: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateRequiredDataConnector,
            >,
        >,
        /// The ID of the anomaly settings definition Id.
        pub settings_definition_id: pulumi_wasm_rust::Output<String>,
        /// A list of `single_select_observation` blocks as defined below.
        pub single_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyDuplicateSingleSelectObservation,
            >,
        >,
        /// A list of categories of attacks by which to classify the rule.
        pub tactics: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of `threshold_observation` blocks as defined below.
        ///
        /// > **NOTE:** un-specified `multi_select_observation`, `single_select_observation`, `prioritized_exclude_observation` and `threshold_observation` will be inherited from the built-in Anomaly Alert Rule.
        pub threshold_observations: pulumi_wasm_rust::Output<
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
        name: &str,
        args: AlertRuleAnomalyDuplicateArgs,
    ) -> AlertRuleAnomalyDuplicateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let built_in_rule_id_binding = args.built_in_rule_id.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let mode_binding = args.mode.get_inner();
        let multi_select_observations_binding = args
            .multi_select_observations
            .get_inner();
        let prioritized_exclude_observations_binding = args
            .prioritized_exclude_observations
            .get_inner();
        let single_select_observations_binding = args
            .single_select_observations
            .get_inner();
        let threshold_observations_binding = args.threshold_observations.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleAnomalyDuplicate:AlertRuleAnomalyDuplicate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "builtInRuleId".into(),
                    value: &built_in_rule_id_binding,
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
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "multiSelectObservations".into(),
                    value: &multi_select_observations_binding,
                },
                register_interface::ObjectField {
                    name: "prioritizedExcludeObservations".into(),
                    value: &prioritized_exclude_observations_binding,
                },
                register_interface::ObjectField {
                    name: "singleSelectObservations".into(),
                    value: &single_select_observations_binding,
                },
                register_interface::ObjectField {
                    name: "thresholdObservations".into(),
                    value: &threshold_observations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "anomalySettingsVersion".into(),
                },
                register_interface::ResultField {
                    name: "anomalyVersion".into(),
                },
                register_interface::ResultField {
                    name: "builtInRuleId".into(),
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
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "isDefaultSettings".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "multiSelectObservations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "prioritizedExcludeObservations".into(),
                },
                register_interface::ResultField {
                    name: "requiredDataConnectors".into(),
                },
                register_interface::ResultField {
                    name: "settingsDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "singleSelectObservations".into(),
                },
                register_interface::ResultField {
                    name: "tactics".into(),
                },
                register_interface::ResultField {
                    name: "techniques".into(),
                },
                register_interface::ResultField {
                    name: "thresholdObservations".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AlertRuleAnomalyDuplicateResult {
            anomaly_settings_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anomalySettingsVersion").unwrap(),
            ),
            anomaly_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anomalyVersion").unwrap(),
            ),
            built_in_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("builtInRuleId").unwrap(),
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
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            is_default_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefaultSettings").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            multi_select_observations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiSelectObservations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            prioritized_exclude_observations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prioritizedExcludeObservations").unwrap(),
            ),
            required_data_connectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiredDataConnectors").unwrap(),
            ),
            settings_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settingsDefinitionId").unwrap(),
            ),
            single_select_observations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleSelectObservations").unwrap(),
            ),
            tactics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tactics").unwrap(),
            ),
            techniques: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("techniques").unwrap(),
            ),
            threshold_observations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thresholdObservations").unwrap(),
            ),
        }
    }
}
