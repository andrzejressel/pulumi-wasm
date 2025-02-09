/// Manages a LogToMetricAction Scheduled Query Rules resource within Azure Monitor.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: monitoring-resources
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: loganalytics
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///       retentionInDays: 30
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: example-actiongroup
///       resourceGroupName: ${example.name}
///       shortName: exampleact
///       webhookReceivers:
///         - name: callmyapi
///           serviceUri: http://example.com/alert
///   # Example: Creates alert using the new Scheduled Query Rules metric
///   exampleMetricAlert:
///     type: azure:monitoring:MetricAlert
///     name: example
///     properties:
///       name: example-metricalert
///       resourceGroupName: ${example.name}
///       scopes:
///         - ${exampleAnalyticsWorkspace.id}
///       description: Action will be triggered when Average_% Idle Time metric is less than 10.
///       frequency: PT1M
///       windowSize: PT5M
///       criterias:
///         - metricNamespace: Microsoft.OperationalInsights/workspaces
///           metricName: UsedCapacity
///           aggregation: Average
///           operator: LessThan
///           threshold: 10
///       actions:
///         - actionGroupId: ${exampleActionGroup.id}
///   # Example: LogToMetric Action for the named Computer
///   exampleScheduledQueryRulesLog:
///     type: azure:monitoring:ScheduledQueryRulesLog
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       criteria:
///         metricName: Average_% Idle Time
///         dimensions:
///           - name: Computer
///             operator: Include
///             values:
///               - targetVM
///       dataSourceId: ${exampleAnalyticsWorkspace.id}
///       description: Scheduled query rule LogToMetric example
///       enabled: true
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Scheduled Query Rule Log can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/scheduledQueryRulesLog:ScheduledQueryRulesLog example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/scheduledQueryRules/myrulename
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduled_query_rules_log {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesLogArgs {
        /// A list of IDs of Resources referred into query.
        #[builder(into, default)]
        pub authorized_resource_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A `criteria` block as defined below.
        #[builder(into)]
        pub criteria: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::ScheduledQueryRulesLogCriteria,
        >,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        #[builder(into)]
        pub data_source_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the scheduled query rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesLogResult {
        /// A list of IDs of Resources referred into query.
        pub authorized_resource_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `criteria` block as defined below.
        pub criteria: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesLogCriteria,
        >,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduledQueryRulesLogArgs,
    ) -> ScheduledQueryRulesLogResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_resource_ids_binding = args
            .authorized_resource_ids
            .get_output(context);
        let criteria_binding = args.criteria.get_output(context);
        let data_source_id_binding = args.data_source_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/scheduledQueryRulesLog:ScheduledQueryRulesLog"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedResourceIds".into(),
                    value: authorized_resource_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "criteria".into(),
                    value: criteria_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceId".into(),
                    value: data_source_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduledQueryRulesLogResult {
            authorized_resource_ids: o.get_field("authorizedResourceIds"),
            criteria: o.get_field("criteria"),
            data_source_id: o.get_field("dataSourceId"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
