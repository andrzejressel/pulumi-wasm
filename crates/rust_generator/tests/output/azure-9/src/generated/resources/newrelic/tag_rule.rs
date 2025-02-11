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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagRuleArgs,
    ) -> TagRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let activity_log_enabled_binding = args.activity_log_enabled.get_output(context);
        let azure_active_directory_log_enabled_binding = args
            .azure_active_directory_log_enabled
            .get_output(context);
        let log_tag_filters_binding = args.log_tag_filters.get_output(context);
        let metric_enabled_binding = args.metric_enabled.get_output(context);
        let metric_tag_filters_binding = args.metric_tag_filters.get_output(context);
        let monitor_id_binding = args.monitor_id.get_output(context);
        let subscription_log_enabled_binding = args
            .subscription_log_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:newrelic/tagRule:TagRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activityLogEnabled".into(),
                    value: &activity_log_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureActiveDirectoryLogEnabled".into(),
                    value: &azure_active_directory_log_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logTagFilters".into(),
                    value: &log_tag_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricEnabled".into(),
                    value: &metric_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricTagFilters".into(),
                    value: &metric_tag_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitorId".into(),
                    value: &monitor_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionLogEnabled".into(),
                    value: &subscription_log_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagRuleResult {
            activity_log_enabled: o.get_field("activityLogEnabled"),
            azure_active_directory_log_enabled: o
                .get_field("azureActiveDirectoryLogEnabled"),
            log_tag_filters: o.get_field("logTagFilters"),
            metric_enabled: o.get_field("metricEnabled"),
            metric_tag_filters: o.get_field("metricTagFilters"),
            monitor_id: o.get_field("monitorId"),
            subscription_log_enabled: o.get_field("subscriptionLogEnabled"),
        }
    }
}
