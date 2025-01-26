pub mod get_alert_rule_anomaly {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleAnomalyArgs {
        /// The display name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        ///
        /// > **NOTE** One of `name` or `display_name` must be specified.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The guid of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleAnomalyResult {
        /// The version of the Anomaly Security ML Analytics Settings.
        pub anomaly_settings_version: pulumi_wasm_rust::Output<i32>,
        /// The anomaly version of the Anomaly Alert Rule.
        pub anomaly_version: pulumi_wasm_rust::Output<String>,
        /// The description of the threshold observation.
        pub description: pulumi_wasm_rust::Output<String>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Is the Anomaly Alert Rule enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The frequency the Anomaly Alert Rule will be run.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        pub mode: pulumi_wasm_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        pub multi_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyMultiSelectObservation,
            >,
        >,
        /// The name of the threshold observation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        pub prioritized_exclude_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyPrioritizedExcludeObservation,
            >,
        >,
        /// A `required_data_connector` block as defined below.
        pub required_data_connectors: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyRequiredDataConnector,
            >,
        >,
        /// The ID of the anomaly settings definition Id.
        pub settings_definition_id: pulumi_wasm_rust::Output<String>,
        /// A list of `single_select_observation` blocks as defined below.
        pub single_select_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalySingleSelectObservation,
            >,
        >,
        /// A list of categories of attacks by which to classify the rule.
        pub tactics: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of `threshold_observation` blocks as defined below.
        pub threshold_observations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyThresholdObservation,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAlertRuleAnomalyArgs,
    ) -> GetAlertRuleAnomalyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:sentinel/getAlertRuleAnomaly:getAlertRuleAnomaly".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAlertRuleAnomalyResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
