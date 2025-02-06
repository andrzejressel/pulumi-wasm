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
pub mod scheduled_query_rules_alert {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertArgs {
        /// An `action` block as defined below.
        #[builder(into)]
        pub action: pulumi_wasm_rust::InputOrOutput<
            super::super::types::monitoring::ScheduledQueryRulesAlertAction,
        >,
        /// List of Resource IDs referred into query.
        #[builder(into, default)]
        pub authorized_resource_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `false`.
        /// > **NOTE** `auto_mitigation_enabled` and `throttling` are mutually exclusive and cannot both be set.
        #[builder(into, default)]
        pub auto_mitigation_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        #[builder(into)]
        pub data_source_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description of the scheduled query rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Frequency (in minutes) at which rule condition should be evaluated. Values must be between 5 and 1440 (inclusive).
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Log search query.
        #[builder(into)]
        pub query: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of query results. Possible values are `ResultCount` and `Number`. Default is `ResultCount`. If set to `ResultCount`, `query` must include an `AggregatedValue` column of a numeric type, for example, `Heartbeat | summarize AggregatedValue = count() by bin(TimeGenerated, 5m)`.
        #[builder(into, default)]
        pub query_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Severity of the alert. Possible values include: 0, 1, 2, 3, or 4.
        #[builder(into, default)]
        pub severity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time (in minutes) for which Alerts should be throttled or suppressed. Values must be between 0 and 10000 (inclusive).
        #[builder(into, default)]
        pub throttling: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Time window for which data needs to be fetched for query (must be greater than or equal to `frequency`). Values must be between 5 and 2880 (inclusive).
        #[builder(into)]
        pub time_window: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A `trigger` block as defined below.
        #[builder(into)]
        pub trigger: pulumi_wasm_rust::InputOrOutput<
            super::super::types::monitoring::ScheduledQueryRulesAlertTrigger,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesAlertResult {
        /// An `action` block as defined below.
        pub action: pulumi_wasm_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesAlertAction,
        >,
        /// List of Resource IDs referred into query.
        pub authorized_resource_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `false`.
        /// > **NOTE** `auto_mitigation_enabled` and `throttling` are mutually exclusive and cannot both be set.
        pub auto_mitigation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Frequency (in minutes) at which rule condition should be evaluated. Values must be between 5 and 1440 (inclusive).
        pub frequency: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Log search query.
        pub query: pulumi_wasm_rust::Output<String>,
        /// The type of query results. Possible values are `ResultCount` and `Number`. Default is `ResultCount`. If set to `ResultCount`, `query` must include an `AggregatedValue` column of a numeric type, for example, `Heartbeat | summarize AggregatedValue = count() by bin(TimeGenerated, 5m)`.
        pub query_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Severity of the alert. Possible values include: 0, 1, 2, 3, or 4.
        pub severity: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time (in minutes) for which Alerts should be throttled or suppressed. Values must be between 0 and 10000 (inclusive).
        pub throttling: pulumi_wasm_rust::Output<Option<i32>>,
        /// Time window for which data needs to be fetched for query (must be greater than or equal to `frequency`). Values must be between 5 and 2880 (inclusive).
        pub time_window: pulumi_wasm_rust::Output<i32>,
        /// A `trigger` block as defined below.
        pub trigger: pulumi_wasm_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesAlertTrigger,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ScheduledQueryRulesAlertArgs,
    ) -> ScheduledQueryRulesAlertResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let authorized_resource_ids_binding = args
            .authorized_resource_ids
            .get_output(context)
            .get_inner();
        let auto_mitigation_enabled_binding = args
            .auto_mitigation_enabled
            .get_output(context)
            .get_inner();
        let data_source_id_binding = args.data_source_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let frequency_binding = args.frequency.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_binding = args.query.get_output(context).get_inner();
        let query_type_binding = args.query_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let severity_binding = args.severity.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let throttling_binding = args.throttling.get_output(context).get_inner();
        let time_window_binding = args.time_window.get_output(context).get_inner();
        let trigger_binding = args.trigger.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/scheduledQueryRulesAlert:ScheduledQueryRulesAlert"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedResourceIds".into(),
                    value: &authorized_resource_ids_binding,
                },
                register_interface::ObjectField {
                    name: "autoMitigationEnabled".into(),
                    value: &auto_mitigation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceId".into(),
                    value: &data_source_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
                register_interface::ObjectField {
                    name: "queryType".into(),
                    value: &query_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "throttling".into(),
                    value: &throttling_binding,
                },
                register_interface::ObjectField {
                    name: "timeWindow".into(),
                    value: &time_window_binding,
                },
                register_interface::ObjectField {
                    name: "trigger".into(),
                    value: &trigger_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduledQueryRulesAlertResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            authorized_resource_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizedResourceIds"),
            ),
            auto_mitigation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoMitigationEnabled"),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSourceId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            query: pulumi_wasm_rust::__private::into_domain(o.extract_field("query")),
            query_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queryType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            throttling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("throttling"),
            ),
            time_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeWindow"),
            ),
            trigger: pulumi_wasm_rust::__private::into_domain(o.extract_field("trigger")),
        }
    }
}
