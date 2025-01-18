/// Manages an Azure Native New Relic Tag Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleMonitor = monitor::create(
///         "exampleMonitor",
///         MonitorArgs::builder()
///             .location("${example.location}")
///             .name("example-nrm")
///             .plan(
///                 MonitorPlan::builder()
///                     .effectiveDate("2023-06-06T00:00:00Z")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .user(
///                 MonitorUser::builder()
///                     .email("user@example.com")
///                     .firstName("Example")
///                     .lastName("User")
///                     .phoneNumber("+12313803556")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTagRule = tag_rule::create(
///         "exampleTagRule",
///         TagRuleArgs::builder()
///             .activity_log_enabled(true)
///             .azure_active_directory_log_enabled(true)
///             .log_tag_filters(
///                 vec![
///                     TagRuleLogTagFilter::builder().action("Include").name("key")
///                     .value("value").build_struct(),
///                 ],
///             )
///             .metric_enabled(true)
///             .metric_tag_filters(
///                 vec![
///                     TagRuleMetricTagFilter::builder().action("Exclude").name("key")
///                     .value("value").build_struct(),
///                 ],
///             )
///             .monitor_id("${exampleMonitor.id}")
///             .subscription_log_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Native New Relic Tag Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:newrelic/tagRule:TagRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/NewRelic.Observability/monitors/monitor1/tagRules/ruleSet1
/// ```
///
pub mod tag_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagRuleArgs {
        /// Whether activity logs from Azure resources should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Azure Active Directory logs should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub azure_active_directory_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `log_tag_filter` block as defined below.
        #[builder(into, default)]
        pub log_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleLogTagFilter>>,
        >,
        /// Whether metrics should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub metric_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `metric_tag_filter` block as defined below.
        #[builder(into, default)]
        pub metric_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleMetricTagFilter>>,
        >,
        /// Specifies the ID of the New Relic Monitor this Tag Rule should be created within. Changing this forces a new Azure Native New Relic Tag Rule to be created.
        #[builder(into)]
        pub monitor_id: pulumi_wasm_rust::Output<String>,
        /// Whether subscription logs should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub subscription_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TagRuleResult {
        /// Whether activity logs from Azure resources should be sent for the Monitor resource. Defaults to `false`.
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether Azure Active Directory logs should be sent for the Monitor resource. Defaults to `false`.
        pub azure_active_directory_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `log_tag_filter` block as defined below.
        pub log_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleLogTagFilter>>,
        >,
        /// Whether metrics should be sent for the Monitor resource. Defaults to `false`.
        pub metric_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `metric_tag_filter` block as defined below.
        pub metric_tag_filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleMetricTagFilter>>,
        >,
        /// Specifies the ID of the New Relic Monitor this Tag Rule should be created within. Changing this forces a new Azure Native New Relic Tag Rule to be created.
        pub monitor_id: pulumi_wasm_rust::Output<String>,
        /// Whether subscription logs should be sent for the Monitor resource. Defaults to `false`.
        pub subscription_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TagRuleArgs) -> TagRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activity_log_enabled_binding = args.activity_log_enabled.get_inner();
        let azure_active_directory_log_enabled_binding = args
            .azure_active_directory_log_enabled
            .get_inner();
        let log_tag_filters_binding = args.log_tag_filters.get_inner();
        let metric_enabled_binding = args.metric_enabled.get_inner();
        let metric_tag_filters_binding = args.metric_tag_filters.get_inner();
        let monitor_id_binding = args.monitor_id.get_inner();
        let subscription_log_enabled_binding = args.subscription_log_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:newrelic/tagRule:TagRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activityLogEnabled".into(),
                    value: &activity_log_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "azureActiveDirectoryLogEnabled".into(),
                    value: &azure_active_directory_log_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "logTagFilters".into(),
                    value: &log_tag_filters_binding,
                },
                register_interface::ObjectField {
                    name: "metricEnabled".into(),
                    value: &metric_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "metricTagFilters".into(),
                    value: &metric_tag_filters_binding,
                },
                register_interface::ObjectField {
                    name: "monitorId".into(),
                    value: &monitor_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionLogEnabled".into(),
                    value: &subscription_log_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activityLogEnabled".into(),
                },
                register_interface::ResultField {
                    name: "azureActiveDirectoryLogEnabled".into(),
                },
                register_interface::ResultField {
                    name: "logTagFilters".into(),
                },
                register_interface::ResultField {
                    name: "metricEnabled".into(),
                },
                register_interface::ResultField {
                    name: "metricTagFilters".into(),
                },
                register_interface::ResultField {
                    name: "monitorId".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionLogEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagRuleResult {
            activity_log_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activityLogEnabled").unwrap(),
            ),
            azure_active_directory_log_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureActiveDirectoryLogEnabled").unwrap(),
            ),
            log_tag_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logTagFilters").unwrap(),
            ),
            metric_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricEnabled").unwrap(),
            ),
            metric_tag_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricTagFilters").unwrap(),
            ),
            monitor_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorId").unwrap(),
            ),
            subscription_log_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionLogEnabled").unwrap(),
            ),
        }
    }
}
