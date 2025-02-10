/// Manages a Metric Alert within Azure Monitor.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleMetricAlert = metric_alert::create(
///         "exampleMetricAlert",
///         MetricAlertArgs::builder()
///             .actions(
///                 vec![
///                     MetricAlertAction::builder().actionGroupId("${main.id}")
///                     .build_struct(),
///                 ],
///             )
///             .criterias(
///                 vec![
///                     MetricAlertCriteria::builder().aggregation("Total")
///                     .dimensions(vec![MetricAlertCriteriaDimension::builder()
///                     .name("ApiName").operator("Include").values(vec!["*",])
///                     .build_struct(),]).metricName("Transactions")
///                     .metricNamespace("Microsoft.Storage/storageAccounts")
///                     .operator("GreaterThan").threshold(50).build_struct(),
///                 ],
///             )
///             .description(
///                 "Action will be triggered when Transactions count is greater than 50.",
///             )
///             .name("example-metricalert")
///             .resource_group_name("${example.name}")
///             .scopes(vec!["${toMonitor.id}",])
///             .build_struct(),
///     );
///     let main = action_group::create(
///         "main",
///         ActionGroupArgs::builder()
///             .name("example-actiongroup")
///             .resource_group_name("${example.name}")
///             .short_name("exampleact")
///             .webhook_receivers(
///                 vec![
///                     ActionGroupWebhookReceiver::builder().name("callmyapi")
///                     .serviceUri("http://example.com/alert").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let toMonitor = account::create(
///         "toMonitor",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorageaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Metric Alerts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/metricAlert:MetricAlert main /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.Insights/metricAlerts/example-metricalert
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metric_alert {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricAlertArgs {
        /// One or more `action` blocks as defined below.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::MetricAlertAction>>,
        >,
        /// A `application_insights_web_test_location_availability_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub application_insights_web_test_location_availability_criteria: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria,
            >,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `true`.
        #[builder(into, default)]
        pub auto_mitigate: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more (static) `criteria` blocks as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub criterias: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::MetricAlertCriteria>>,
        >,
        /// The description of this Metric Alert.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `dynamic_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub dynamic_criteria: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::MetricAlertDynamicCriteria>,
        >,
        /// Should this Metric Alert be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The evaluation frequency of this Metric Alert, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M` and `PT1H`. Defaults to `PT1M`.
        #[builder(into, default)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Metric Alert. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Metric Alert instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A set of strings of resource IDs at which the metric criteria should be applied.
        #[builder(into)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The severity of this Metric Alert. Possible values are `0`, `1`, `2`, `3` and `4`. Defaults to `3`.
        #[builder(into, default)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        #[builder(into, default)]
        pub target_resource_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource type (e.g. `Microsoft.Compute/virtualMachines`) of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        #[builder(into, default)]
        pub target_resource_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The period of time that is used to monitor alert activity, represented in ISO 8601 duration format. This value must be greater than `frequency`. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M`, `PT1H`, `PT6H`, `PT12H` and `P1D`. Defaults to `PT5M`.
        #[builder(into, default)]
        pub window_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricAlertResult {
        /// One or more `action` blocks as defined below.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::MetricAlertAction>>,
        >,
        /// A `application_insights_web_test_location_availability_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub application_insights_web_test_location_availability_criteria: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::monitoring::MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria,
            >,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `true`.
        pub auto_mitigate: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more (static) `criteria` blocks as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub criterias: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::MetricAlertCriteria>>,
        >,
        /// The description of this Metric Alert.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `dynamic_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub dynamic_criteria: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::MetricAlertDynamicCriteria>,
        >,
        /// Should this Metric Alert be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The evaluation frequency of this Metric Alert, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M` and `PT1H`. Defaults to `PT1M`.
        pub frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Metric Alert. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Metric Alert instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A set of strings of resource IDs at which the metric criteria should be applied.
        pub scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The severity of this Metric Alert. Possible values are `0`, `1`, `2`, `3` and `4`. Defaults to `3`.
        pub severity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        pub target_resource_location: pulumi_gestalt_rust::Output<String>,
        /// The resource type (e.g. `Microsoft.Compute/virtualMachines`) of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        pub target_resource_type: pulumi_gestalt_rust::Output<String>,
        /// The period of time that is used to monitor alert activity, represented in ISO 8601 duration format. This value must be greater than `frequency`. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M`, `PT1H`, `PT6H`, `PT12H` and `P1D`. Defaults to `PT5M`.
        pub window_size: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MetricAlertArgs,
    ) -> MetricAlertResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let application_insights_web_test_location_availability_criteria_binding = args
            .application_insights_web_test_location_availability_criteria
            .get_output(context);
        let auto_mitigate_binding = args.auto_mitigate.get_output(context);
        let criterias_binding = args.criterias.get_output(context);
        let description_binding = args.description.get_output(context);
        let dynamic_criteria_binding = args.dynamic_criteria.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let severity_binding = args.severity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_resource_location_binding = args
            .target_resource_location
            .get_output(context);
        let target_resource_type_binding = args.target_resource_type.get_output(context);
        let window_size_binding = args.window_size.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/metricAlert:MetricAlert".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsWebTestLocationAvailabilityCriteria"
                        .into(),
                    value: application_insights_web_test_location_availability_criteria_binding
                        .get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoMitigate".into(),
                    value: auto_mitigate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "criterias".into(),
                    value: criterias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicCriteria".into(),
                    value: dynamic_criteria_binding.get_id(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: scopes_binding.get_id(),
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
                    name: "targetResourceLocation".into(),
                    value: target_resource_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceType".into(),
                    value: target_resource_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "windowSize".into(),
                    value: window_size_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MetricAlertResult {
            actions: o.get_field("actions"),
            application_insights_web_test_location_availability_criteria: o
                .get_field("applicationInsightsWebTestLocationAvailabilityCriteria"),
            auto_mitigate: o.get_field("autoMitigate"),
            criterias: o.get_field("criterias"),
            description: o.get_field("description"),
            dynamic_criteria: o.get_field("dynamicCriteria"),
            enabled: o.get_field("enabled"),
            frequency: o.get_field("frequency"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scopes: o.get_field("scopes"),
            severity: o.get_field("severity"),
            tags: o.get_field("tags"),
            target_resource_location: o.get_field("targetResourceLocation"),
            target_resource_type: o.get_field("targetResourceType"),
            window_size: o.get_field("windowSize"),
        }
    }
}
