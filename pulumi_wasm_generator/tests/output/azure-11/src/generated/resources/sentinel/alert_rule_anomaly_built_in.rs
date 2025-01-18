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
///   exampleAlertRuleAnomalyBuiltIn:
///     type: azure:sentinel:AlertRuleAnomalyBuiltIn
///     name: example
///     properties:
///       displayName: Potential data staging
///       logAnalyticsWorkspaceId: ${exampleAnalyticsWorkspace.id}
///       mode: Production
///       enabled: false
/// variables:
///   example:
///     fn::invoke:
///       function: azure:sentinel:getAlertRuleAnomaly
///       arguments:
///         logAnalyticsWorkspaceId: ${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}
///         displayName: Potential data staging
/// ```
///
/// ## Import
///
/// Built In Anomaly Alert Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/alertRuleAnomalyBuiltIn:AlertRuleAnomalyBuiltIn example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/securityMLAnalyticsSettings/setting1
/// ```
///
pub mod alert_rule_anomaly_built_in {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyBuiltInArgs {
        /// The Display Name of the built-in Anomaly Alert Rule.
        ///
        /// > **Note:** One of `name` or `display_name` block must be specified.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Built-in Anomaly Alert Rule be enabled?
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Built-in Anomaly Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// mode of the Built-in Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::Output<String>,
        /// The Name of the built-in Anomaly Alert Rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyBuiltInResult {
        /// The version of the Anomaly Security ML Analytics Settings.
        pub anomaly_settings_version: pulumi_wasm_rust::Output<i32>,
        /// The anomaly version of the Anomaly Alert Rule.
        pub anomaly_version: pulumi_wasm_rust::Output<String>,
        /// The description of the threshold observation.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The Display Name of the built-in Anomaly Alert Rule.
        ///
        /// > **Note:** One of `name` or `display_name` block must be specified.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Should the Built-in Anomaly Alert Rule be enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The frequency the Anomaly Alert Rule will be run.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Built-in Anomaly Alert Rule to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// mode of the Built-in Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        pub multi_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyBuiltInMultiSelectObservation,
            >,
        >,
        /// The Name of the built-in Anomaly Alert Rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        pub prioritized_exclude_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyBuiltInPrioritizedExcludeObservation,
            >,
        >,
        /// A `required_data_connector` block as defined below.
        pub required_data_connectors: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyBuiltInRequiredDataConnector,
            >,
        >,
        /// The ID of the anomaly settings definition Id.
        pub settings_definition_id: pulumi_wasm_rust::Output<String>,
        /// A list of `single_select_observation` blocks as defined below.
        pub single_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyBuiltInSingleSelectObservation,
            >,
        >,
        /// A list of categories of attacks by which to classify the rule.
        pub tactics: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of `threshold_observation` blocks as defined below.
        pub threshold_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::sentinel::AlertRuleAnomalyBuiltInThresholdObservation,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AlertRuleAnomalyBuiltInArgs,
    ) -> AlertRuleAnomalyBuiltInResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let mode_binding = args.mode.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/alertRuleAnomalyBuiltIn:AlertRuleAnomalyBuiltIn"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "name".into(),
                    value: &name_binding,
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
        AlertRuleAnomalyBuiltInResult {
            anomaly_settings_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anomalySettingsVersion").unwrap(),
            ),
            anomaly_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anomalyVersion").unwrap(),
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
