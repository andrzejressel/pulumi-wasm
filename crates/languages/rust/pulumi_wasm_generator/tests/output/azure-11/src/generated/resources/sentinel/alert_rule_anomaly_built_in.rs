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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertRuleAnomalyBuiltInArgs {
        /// The Display Name of the built-in Anomaly Alert Rule.
        ///
        /// > **Note:** One of `name` or `display_name` block must be specified.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the Built-in Anomaly Alert Rule be enabled?
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The ID of the Log Analytics Workspace. Changing this forces a new Built-in Anomaly Alert Rule to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// mode of the Built-in Anomaly Alert Rule. Possible Values are `Production` and `Flighting`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the built-in Anomaly Alert Rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AlertRuleAnomalyBuiltInArgs,
    ) -> AlertRuleAnomalyBuiltInResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AlertRuleAnomalyBuiltInResult {
            anomaly_settings_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("anomalySettingsVersion"),
            ),
            anomaly_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("anomalyVersion"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("mode")),
            multi_select_observations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiSelectObservations"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            prioritized_exclude_observations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("prioritizedExcludeObservations"),
            ),
            required_data_connectors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requiredDataConnectors"),
            ),
            settings_definition_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("settingsDefinitionId"),
            ),
            single_select_observations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("singleSelectObservations"),
            ),
            tactics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tactics"),
            ),
            techniques: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("techniques"),
            ),
            threshold_observations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thresholdObservations"),
            ),
        }
    }
}
