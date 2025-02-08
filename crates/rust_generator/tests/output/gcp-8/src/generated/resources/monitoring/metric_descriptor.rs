/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or altering it stops data collection and makes the metric type's existing data unusable.
///
///
/// To get more information about MetricDescriptor, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/custom-metrics/)
///
/// ## Example Usage
///
/// ### Monitoring Metric Descriptor Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = metric_descriptor::create(
///         "basic",
///         MetricDescriptorArgs::builder()
///             .description("Daily sales records from all branch stores.")
///             .display_name("metric-descriptor")
///             .labels(
///                 vec![
///                     MetricDescriptorLabel::builder().description("The ID of the store.")
///                     .key("store_id").valueType("STRING").build_struct(),
///                 ],
///             )
///             .launch_stage("BETA")
///             .metadata(
///                 MetricDescriptorMetadata::builder()
///                     .ingestDelay("30s")
///                     .samplePeriod("60s")
///                     .build_struct(),
///             )
///             .metric_kind("GAUGE")
///             .type_("custom.googleapis.com/stores/daily_sales")
///             .unit("{USD}")
///             .value_type("DOUBLE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Monitoring Metric Descriptor Alert
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alertPolicy = alert_policy::create(
///         "alertPolicy",
///         AlertPolicyArgs::builder()
///             .combiner("OR")
///             .conditions(
///                 vec![
///                     AlertPolicyCondition::builder()
///                     .conditionThreshold(AlertPolicyConditionConditionThreshold::builder()
///                     .comparison("COMPARISON_GT").duration("60s")
///                     .filter("metric.type=\"${withAlert.type}\" AND resource.type=\"gce_instance\"")
///                     .build_struct()).displayName("test condition").build_struct(),
///                 ],
///             )
///             .display_name("metric-descriptor")
///             .build_struct(),
///     );
///     let withAlert = metric_descriptor::create(
///         "withAlert",
///         MetricDescriptorArgs::builder()
///             .description("Daily sales records from all branch stores.")
///             .display_name("metric-descriptor")
///             .metric_kind("GAUGE")
///             .type_("custom.googleapis.com/stores/daily_sales")
///             .unit("{USD}")
///             .value_type("DOUBLE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MetricDescriptor can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, MetricDescriptor can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/metricDescriptor:MetricDescriptor default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/metricDescriptor:MetricDescriptor default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/metricDescriptor:MetricDescriptor default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metric_descriptor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricDescriptorArgs {
        /// A detailed description of the metric, which can be used in documentation.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count".
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The set of labels that can be used to describe a specific instance of this metric type. In order to delete a label, the entire resource must be deleted, then created with the desired labels.
        /// Structure is documented below.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::MetricDescriptorLabel>>,
        >,
        /// The launch stage of the metric definition.
        /// Possible values are: `LAUNCH_STAGE_UNSPECIFIED`, `UNIMPLEMENTED`, `PRELAUNCH`, `EARLY_ACCESS`, `ALPHA`, `BETA`, `GA`, `DEPRECATED`.
        #[builder(into, default)]
        pub launch_stage: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Metadata which can be used to guide usage of the metric.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::MetricDescriptorMetadata>,
        >,
        /// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metricKind and valueType might not be supported.
        /// Possible values are: `METRIC_KIND_UNSPECIFIED`, `GAUGE`, `DELTA`, `CUMULATIVE`.
        #[builder(into)]
        pub metric_kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The metric type, including its DNS name prefix. The type is not URL-encoded. All service defined metrics must be prefixed with the service name, in the format of {service name}/{relative metric name}, such as cloudsql.googleapis.com/database/cpu/utilization. The relative metric name must have only upper and lower-case letters, digits, '/' and underscores '_' are allowed. Additionally, the maximum number of characters allowed for the relative_metric_name is 100. All user-defined metric types have the DNS name custom.googleapis.com, external.googleapis.com, or logging.googleapis.com/user/.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The units in which the metric value is reported. It is only applicable if the
        /// valueType is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of
        /// the stored metric values.
        /// Different systems may scale the values to be more easily displayed (so a value of
        /// 0.02KBy might be displayed as 20By, and a value of 3523KBy might be displayed as
        /// 3.5MBy). However, if the unit is KBy, then the value of the metric is always in
        /// thousands of bytes, no matter how it may be displayed.
        /// If you want a custom metric to record the exact number of CPU-seconds used by a job,
        /// you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently
        /// 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as
        /// 12005.
        /// Alternatively, if you want a custom metric to record data in a more granular way, you
        /// can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value
        /// 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).
        /// The supported units are a subset of The Unified Code for Units of Measure standard.
        /// More info can be found in the API documentation
        /// (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors).
        #[builder(into, default)]
        pub unit: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metricKind and valueType might not be supported.
        /// Possible values are: `BOOL`, `INT64`, `DOUBLE`, `STRING`, `DISTRIBUTION`.
        #[builder(into)]
        pub value_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MetricDescriptorResult {
        /// A detailed description of the metric, which can be used in documentation.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count".
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The set of labels that can be used to describe a specific instance of this metric type. In order to delete a label, the entire resource must be deleted, then created with the desired labels.
        /// Structure is documented below.
        pub labels: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::MetricDescriptorLabel>>,
        >,
        /// The launch stage of the metric definition.
        /// Possible values are: `LAUNCH_STAGE_UNSPECIFIED`, `UNIMPLEMENTED`, `PRELAUNCH`, `EARLY_ACCESS`, `ALPHA`, `BETA`, `GA`, `DEPRECATED`.
        pub launch_stage: pulumi_gestalt_rust::Output<Option<String>>,
        /// Metadata which can be used to guide usage of the metric.
        /// Structure is documented below.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::MetricDescriptorMetadata>,
        >,
        /// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metricKind and valueType might not be supported.
        /// Possible values are: `METRIC_KIND_UNSPECIFIED`, `GAUGE`, `DELTA`, `CUMULATIVE`.
        pub metric_kind: pulumi_gestalt_rust::Output<String>,
        /// If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here. This field allows time series to be associated with the intersection of this metric type and the monitored resource types in this list.
        pub monitored_resource_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The resource name of the metric descriptor.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The metric type, including its DNS name prefix. The type is not URL-encoded. All service defined metrics must be prefixed with the service name, in the format of {service name}/{relative metric name}, such as cloudsql.googleapis.com/database/cpu/utilization. The relative metric name must have only upper and lower-case letters, digits, '/' and underscores '_' are allowed. Additionally, the maximum number of characters allowed for the relative_metric_name is 100. All user-defined metric types have the DNS name custom.googleapis.com, external.googleapis.com, or logging.googleapis.com/user/.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The units in which the metric value is reported. It is only applicable if the
        /// valueType is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of
        /// the stored metric values.
        /// Different systems may scale the values to be more easily displayed (so a value of
        /// 0.02KBy might be displayed as 20By, and a value of 3523KBy might be displayed as
        /// 3.5MBy). However, if the unit is KBy, then the value of the metric is always in
        /// thousands of bytes, no matter how it may be displayed.
        /// If you want a custom metric to record the exact number of CPU-seconds used by a job,
        /// you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently
        /// 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as
        /// 12005.
        /// Alternatively, if you want a custom metric to record data in a more granular way, you
        /// can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value
        /// 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).
        /// The supported units are a subset of The Unified Code for Units of Measure standard.
        /// More info can be found in the API documentation
        /// (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors).
        pub unit: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metricKind and valueType might not be supported.
        /// Possible values are: `BOOL`, `INT64`, `DOUBLE`, `STRING`, `DISTRIBUTION`.
        pub value_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MetricDescriptorArgs,
    ) -> MetricDescriptorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let launch_stage_binding = args.launch_stage.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let metric_kind_binding = args.metric_kind.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let unit_binding = args.unit.get_output(context).get_inner();
        let value_type_binding = args.value_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:monitoring/metricDescriptor:MetricDescriptor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "launchStage".into(),
                    value: &launch_stage_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "metricKind".into(),
                    value: &metric_kind_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "unit".into(),
                    value: &unit_binding,
                },
                register_interface::ObjectField {
                    name: "valueType".into(),
                    value: &value_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MetricDescriptorResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            launch_stage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchStage"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            metric_kind: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricKind"),
            ),
            monitored_resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoredResourceTypes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            unit: pulumi_gestalt_rust::__private::into_domain(o.extract_field("unit")),
            value_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("valueType"),
            ),
        }
    }
}
