pub mod get_alert_rule_anomaly {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleAnomalyArgs {
        /// The display name of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        ///
        /// > **NOTE** One of `name` or `display_name` must be specified.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Log Analytics Workspace.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The guid of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleAnomalyResult {
        /// The version of the Anomaly Security ML Analytics Settings.
        pub anomaly_settings_version: pulumi_gestalt_rust::Output<i32>,
        /// The anomaly version of the Anomaly Alert Rule.
        pub anomaly_version: pulumi_gestalt_rust::Output<String>,
        /// The description of the threshold observation.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Is the Anomaly Alert Rule enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The frequency the Anomaly Alert Rule will be run.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// A list of `multi_select_observation` blocks as defined below.
        pub multi_select_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyMultiSelectObservation,
            >,
        >,
        /// The name of the threshold observation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `prioritized_exclude_observation` blocks as defined below.
        pub prioritized_exclude_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyPrioritizedExcludeObservation,
            >,
        >,
        /// A `required_data_connector` block as defined below.
        pub required_data_connectors: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalyRequiredDataConnector,
            >,
        >,
        /// The ID of the anomaly settings definition Id.
        pub settings_definition_id: pulumi_gestalt_rust::Output<String>,
        /// A list of `single_select_observation` blocks as defined below.
        pub single_select_observations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::sentinel::GetAlertRuleAnomalySingleSelectObservation,
            >,
        >,
        /// A list of categories of attacks by which to classify the rule.
        pub tactics: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of techniques of attacks by which to classify the rule.
        pub techniques: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of `threshold_observation` blocks as defined below.
        pub threshold_observations: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAlertRuleAnomalyArgs,
    ) -> GetAlertRuleAnomalyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAlertRuleAnomalyResult {
            anomaly_settings_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("anomalySettingsVersion"),
            ),
            anomaly_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("anomalyVersion"),
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
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            multi_select_observations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiSelectObservations"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            prioritized_exclude_observations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prioritizedExcludeObservations"),
            ),
            required_data_connectors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requiredDataConnectors"),
            ),
            settings_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("settingsDefinitionId"),
            ),
            single_select_observations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("singleSelectObservations"),
            ),
            tactics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tactics"),
            ),
            techniques: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("techniques"),
            ),
            threshold_observations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thresholdObservations"),
            ),
        }
    }
}
