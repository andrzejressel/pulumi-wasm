/// Manages an Azure Native New Relic Tag Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagRuleArgs {
        /// Whether activity logs from Azure resources should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub activity_log_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Azure Active Directory logs should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub azure_active_directory_log_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A `log_tag_filter` block as defined below.
        #[builder(into, default)]
        pub log_tag_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::newrelic::TagRuleLogTagFilter>>,
        >,
        /// Whether metrics should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub metric_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `metric_tag_filter` block as defined below.
        #[builder(into, default)]
        pub metric_tag_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::newrelic::TagRuleMetricTagFilter>>,
        >,
        /// Specifies the ID of the New Relic Monitor this Tag Rule should be created within. Changing this forces a new Azure Native New Relic Tag Rule to be created.
        #[builder(into)]
        pub monitor_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether subscription logs should be sent for the Monitor resource. Defaults to `false`.
        #[builder(into, default)]
        pub subscription_log_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TagRuleResult {
        /// Whether activity logs from Azure resources should be sent for the Monitor resource. Defaults to `false`.
        pub activity_log_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Azure Active Directory logs should be sent for the Monitor resource. Defaults to `false`.
        pub azure_active_directory_log_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A `log_tag_filter` block as defined below.
        pub log_tag_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleLogTagFilter>>,
        >,
        /// Whether metrics should be sent for the Monitor resource. Defaults to `false`.
        pub metric_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `metric_tag_filter` block as defined below.
        pub metric_tag_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::newrelic::TagRuleMetricTagFilter>>,
        >,
        /// Specifies the ID of the New Relic Monitor this Tag Rule should be created within. Changing this forces a new Azure Native New Relic Tag Rule to be created.
        pub monitor_id: pulumi_gestalt_rust::Output<String>,
        /// Whether subscription logs should be sent for the Monitor resource. Defaults to `false`.
        pub subscription_log_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TagRuleArgs,
    ) -> TagRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activity_log_enabled_binding_1 = args
            .activity_log_enabled
            .get_output(context);
        let activity_log_enabled_binding = activity_log_enabled_binding_1.get_inner();
        let azure_active_directory_log_enabled_binding_1 = args
            .azure_active_directory_log_enabled
            .get_output(context);
        let azure_active_directory_log_enabled_binding = azure_active_directory_log_enabled_binding_1
            .get_inner();
        let log_tag_filters_binding_1 = args.log_tag_filters.get_output(context);
        let log_tag_filters_binding = log_tag_filters_binding_1.get_inner();
        let metric_enabled_binding_1 = args.metric_enabled.get_output(context);
        let metric_enabled_binding = metric_enabled_binding_1.get_inner();
        let metric_tag_filters_binding_1 = args.metric_tag_filters.get_output(context);
        let metric_tag_filters_binding = metric_tag_filters_binding_1.get_inner();
        let monitor_id_binding_1 = args.monitor_id.get_output(context);
        let monitor_id_binding = monitor_id_binding_1.get_inner();
        let subscription_log_enabled_binding_1 = args
            .subscription_log_enabled
            .get_output(context);
        let subscription_log_enabled_binding = subscription_log_enabled_binding_1
            .get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagRuleResult {
            activity_log_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activityLogEnabled"),
            ),
            azure_active_directory_log_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureActiveDirectoryLogEnabled"),
            ),
            log_tag_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logTagFilters"),
            ),
            metric_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricEnabled"),
            ),
            metric_tag_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricTagFilters"),
            ),
            monitor_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorId"),
            ),
            subscription_log_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionLogEnabled"),
            ),
        }
    }
}
