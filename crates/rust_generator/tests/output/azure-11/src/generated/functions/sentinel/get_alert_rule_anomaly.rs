#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetAlertRuleAnomalyArgs,
    ) -> GetAlertRuleAnomalyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:sentinel/getAlertRuleAnomaly:getAlertRuleAnomaly".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAlertRuleAnomalyResult {
            anomaly_settings_version: o.get_field("anomalySettingsVersion"),
            anomaly_version: o.get_field("anomalyVersion"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enabled: o.get_field("enabled"),
            frequency: o.get_field("frequency"),
            id: o.get_field("id"),
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
