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
pub mod scheduled_query_rules_log {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesLogArgs {
        /// A list of IDs of Resources referred into query.
        #[builder(into, default)]
        pub authorized_resource_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `criteria` block as defined below.
        #[builder(into)]
        pub criteria: pulumi_wasm_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesLogCriteria,
        >,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        #[builder(into)]
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// The description of the scheduled query rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledQueryRulesLogResult {
        /// A list of IDs of Resources referred into query.
        pub authorized_resource_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `criteria` block as defined below.
        pub criteria: pulumi_wasm_rust::Output<
            super::super::types::monitoring::ScheduledQueryRulesLogCriteria,
        >,
        /// The resource URI over which log search query is to be run. Changing this forces a new resource to be created.
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this scheduled query rule is enabled. Default is `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Azure Region where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the scheduled query rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the scheduled query rule instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ScheduledQueryRulesLogArgs,
    ) -> ScheduledQueryRulesLogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorized_resource_ids_binding = args.authorized_resource_ids.get_inner();
        let criteria_binding = args.criteria.get_inner();
        let data_source_id_binding = args.data_source_id.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/scheduledQueryRulesLog:ScheduledQueryRulesLog"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizedResourceIds".into(),
                    value: &authorized_resource_ids_binding,
                },
                register_interface::ObjectField {
                    name: "criteria".into(),
                    value: &criteria_binding,
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
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizedResourceIds".into(),
                },
                register_interface::ResultField {
                    name: "criteria".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScheduledQueryRulesLogResult {
            authorized_resource_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedResourceIds").unwrap(),
            ),
            criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("criteria").unwrap(),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
