/// Manages a Metric Alert within Azure Monitor.
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
pub mod metric_alert {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricAlertArgs {
        /// One or more `action` blocks as defined below.
        #[builder(into, default)]
        pub actions: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::MetricAlertAction>>,
        >,
        /// A `application_insights_web_test_location_availability_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub application_insights_web_test_location_availability_criteria: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::monitoring::MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria,
            >,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `true`.
        #[builder(into, default)]
        pub auto_mitigate: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// One or more (static) `criteria` blocks as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub criterias: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::MetricAlertCriteria>>,
        >,
        /// The description of this Metric Alert.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `dynamic_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        #[builder(into, default)]
        pub dynamic_criteria: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::monitoring::MetricAlertDynamicCriteria>,
        >,
        /// Should this Metric Alert be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The evaluation frequency of this Metric Alert, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M` and `PT1H`. Defaults to `PT1M`.
        #[builder(into, default)]
        pub frequency: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Metric Alert. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Metric Alert instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A set of strings of resource IDs at which the metric criteria should be applied.
        #[builder(into)]
        pub scopes: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The severity of this Metric Alert. Possible values are `0`, `1`, `2`, `3` and `4`. Defaults to `3`.
        #[builder(into, default)]
        pub severity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        #[builder(into, default)]
        pub target_resource_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource type (e.g. `Microsoft.Compute/virtualMachines`) of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        #[builder(into, default)]
        pub target_resource_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The period of time that is used to monitor alert activity, represented in ISO 8601 duration format. This value must be greater than `frequency`. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M`, `PT1H`, `PT6H`, `PT12H` and `P1D`. Defaults to `PT5M`.
        #[builder(into, default)]
        pub window_size: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricAlertResult {
        /// One or more `action` blocks as defined below.
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::MetricAlertAction>>,
        >,
        /// A `application_insights_web_test_location_availability_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub application_insights_web_test_location_availability_criteria: pulumi_wasm_rust::Output<
            Option<
                super::super::types::monitoring::MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria,
            >,
        >,
        /// Should the alerts in this Metric Alert be auto resolved? Defaults to `true`.
        pub auto_mitigate: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more (static) `criteria` blocks as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub criterias: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::MetricAlertCriteria>>,
        >,
        /// The description of this Metric Alert.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A `dynamic_criteria` block as defined below.
        ///
        /// > **NOTE** One of either `criteria`, `dynamic_criteria` or `application_insights_web_test_location_availability_criteria` must be specified.
        pub dynamic_criteria: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::MetricAlertDynamicCriteria>,
        >,
        /// Should this Metric Alert be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The evaluation frequency of this Metric Alert, represented in ISO 8601 duration format. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M` and `PT1H`. Defaults to `PT1M`.
        pub frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Metric Alert. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Metric Alert instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A set of strings of resource IDs at which the metric criteria should be applied.
        pub scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The severity of this Metric Alert. Possible values are `0`, `1`, `2`, `3` and `4`. Defaults to `3`.
        pub severity: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        pub target_resource_location: pulumi_wasm_rust::Output<String>,
        /// The resource type (e.g. `Microsoft.Compute/virtualMachines`) of the target resource.
        ///
        /// > This is Required when using a Subscription as scope, a Resource Group as scope or Multiple Scopes.
        pub target_resource_type: pulumi_wasm_rust::Output<String>,
        /// The period of time that is used to monitor alert activity, represented in ISO 8601 duration format. This value must be greater than `frequency`. Possible values are `PT1M`, `PT5M`, `PT15M`, `PT30M`, `PT1H`, `PT6H`, `PT12H` and `P1D`. Defaults to `PT5M`.
        pub window_size: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MetricAlertArgs,
    ) -> MetricAlertResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let application_insights_web_test_location_availability_criteria_binding = args
            .application_insights_web_test_location_availability_criteria
            .get_output(context)
            .get_inner();
        let auto_mitigate_binding = args.auto_mitigate.get_output(context).get_inner();
        let criterias_binding = args.criterias.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dynamic_criteria_binding = args
            .dynamic_criteria
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let frequency_binding = args.frequency.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let severity_binding = args.severity.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_resource_location_binding = args
            .target_resource_location
            .get_output(context)
            .get_inner();
        let target_resource_type_binding = args
            .target_resource_type
            .get_output(context)
            .get_inner();
        let window_size_binding = args.window_size.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/metricAlert:MetricAlert".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "applicationInsightsWebTestLocationAvailabilityCriteria"
                        .into(),
                    value: &application_insights_web_test_location_availability_criteria_binding,
                },
                register_interface::ObjectField {
                    name: "autoMitigate".into(),
                    value: &auto_mitigate_binding,
                },
                register_interface::ObjectField {
                    name: "criterias".into(),
                    value: &criterias_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dynamicCriteria".into(),
                    value: &dynamic_criteria_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
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
                    name: "targetResourceLocation".into(),
                    value: &target_resource_location_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceType".into(),
                    value: &target_resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "windowSize".into(),
                    value: &window_size_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MetricAlertResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            application_insights_web_test_location_availability_criteria: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationInsightsWebTestLocationAvailabilityCriteria"),
            ),
            auto_mitigate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoMitigate"),
            ),
            criterias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("criterias"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dynamic_criteria: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dynamicCriteria"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(o.extract_field("scopes")),
            severity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("severity"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_resource_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetResourceLocation"),
            ),
            target_resource_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetResourceType"),
            ),
            window_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("windowSize"),
            ),
        }
    }
}
