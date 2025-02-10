/// Manages an AlertingAction Scheduled Query Rules resource within Azure Monitor.
///
/// > **Warning** This resource is using an older AzureRM API version which is known to cause problems e.g. with custom webhook properties not included in triggered alerts. This resource is superseded by the azure.monitoring.ScheduledQueryRulesAlertV2 resource using newer API versions.
///
/// ## Import
///
/// Scheduled Query Rule Alerts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/scheduledQueryRulesAlert:ScheduledQueryRulesAlert example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/scheduledQueryRules/myrulename
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduled_query_rules_alert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertArgs {
        /// An `action` block as defined below.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::ScheduledQueryRulesAlertAction,
        >,
        /// List of Resource IDs referred into query.
        #[builder(into, default)]
        pub authorized_resource_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `false`.
        /// > **NOTE** `auto_mitigation_enabled` and `throttling` are mutually exclusive and cannot both be set.
        #[builder(into, default)]
        pub auto_mitigation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        #[builder(into)]
        pub data_source_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the scheduled query rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Frequency (in minutes) at which rule condition should be evaluated. Values must be between 5 and 1440 (inclusive).
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Log search query.
        #[builder(into)]
        pub query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of query results. Possible values are `ResultCount` and `Number`. Default is `ResultCount`. If set to `ResultCount`, `query` must include an `AggregatedValue` column of a numeric type, for example, `Heartbeat | summarize AggregatedValue = count() by bin(TimeGenerated, 5m)`.
        #[builder(into, default)]
        pub query_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Severity of the alert. Possible values include: 0, 1, 2, 3, or 4.
        #[builder(into, default)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time (in minutes) for which Alerts should be throttled or suppressed. Values must be between 0 and 10000 (inclusive).
        #[builder(into, default)]
        pub throttling: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Time window for which data needs to be fetched for query (must be greater than or equal to `frequency`). Values must be between 5 and 2880 (inclusive).
        #[builder(into)]
        pub time_window: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A `trigger` block as defined below.
        #[builder(into)]
        pub trigger: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::ScheduledQueryRulesAlertTrigger,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertResult {
        /// An `action` block as defined below.
        pub action: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesAlertAction,
        >,
        /// List of Resource IDs referred into query.
        pub authorized_resource_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `false`.
        /// > **NOTE** `auto_mitigation_enabled` and `throttling` are mutually exclusive and cannot both be set.
        pub auto_mitigation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Frequency (in minutes) at which rule condition should be evaluated. Values must be between 5 and 1440 (inclusive).
        pub frequency: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Log search query.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// The type of query results. Possible values are `ResultCount` and `Number`. Default is `ResultCount`. If set to `ResultCount`, `query` must include an `AggregatedValue` column of a numeric type, for example, `Heartbeat | summarize AggregatedValue = count() by bin(TimeGenerated, 5m)`.
        pub query_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Severity of the alert. Possible values include: 0, 1, 2, 3, or 4.
        pub severity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time (in minutes) for which Alerts should be throttled or suppressed. Values must be between 0 and 10000 (inclusive).
        pub throttling: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Time window for which data needs to be fetched for query (must be greater than or equal to `frequency`). Values must be between 5 and 2880 (inclusive).
        pub time_window: pulumi_gestalt_rust::Output<i32>,
        /// A `trigger` block as defined below.
        pub trigger: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesAlertTrigger,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduledQueryRulesAlertArgs,
    ) -> ScheduledQueryRulesAlertResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let authorized_resource_ids_binding = args
            .authorized_resource_ids
            .get_output(context);
        let auto_mitigation_enabled_binding = args
            .auto_mitigation_enabled
            .get_output(context);
        let data_source_id_binding = args.data_source_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_binding = args.query.get_output(context);
        let query_type_binding = args.query_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let severity_binding = args.severity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throttling_binding = args.throttling.get_output(context);
        let time_window_binding = args.time_window.get_output(context);
        let trigger_binding = args.trigger.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/scheduledQueryRulesAlert:ScheduledQueryRulesAlert"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizedResourceIds".into(),
                    value: authorized_resource_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoMitigationEnabled".into(),
                    value: auto_mitigation_enabled_binding.get_id(),
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
                    name: "frequency".into(),
                    value: frequency_binding.get_id(),
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
                    name: "query".into(),
                    value: query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryType".into(),
                    value: query_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "severity".into(),
                    value: severity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throttling".into(),
                    value: throttling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeWindow".into(),
                    value: time_window_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trigger".into(),
                    value: trigger_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduledQueryRulesAlertResult {
            action: o.get_field("action"),
            authorized_resource_ids: o.get_field("authorizedResourceIds"),
            auto_mitigation_enabled: o.get_field("autoMitigationEnabled"),
            data_source_id: o.get_field("dataSourceId"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            frequency: o.get_field("frequency"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            query: o.get_field("query"),
            query_type: o.get_field("queryType"),
            resource_group_name: o.get_field("resourceGroupName"),
            severity: o.get_field("severity"),
            tags: o.get_field("tags"),
            throttling: o.get_field("throttling"),
            time_window: o.get_field("timeWindow"),
            trigger: o.get_field("trigger"),
        }
    }
}
