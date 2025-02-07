/// Provides a CE Anomaly Monitor.
///
/// ## Example Usage
///
/// There are two main types of a Cost Anomaly Monitor: `DIMENSIONAL` and `CUSTOM`.
///
/// ### Dimensional Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceMonitor = anomaly_monitor::create(
///         "serviceMonitor",
///         AnomalyMonitorArgs::builder()
///             .monitor_dimension("SERVICE")
///             .monitor_type("DIMENSIONAL")
///             .name("AWSServiceMonitor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom Example
///
/// ```yaml
/// resources:
///   test:
///     type: aws:costexplorer:AnomalyMonitor
///     properties:
///       name: AWSCustomAnomalyMonitor
///       monitorType: CUSTOM
///       monitorSpecification:
///         fn::toJSON:
///           And: null
///           CostCategories: null
///           Dimensions: null
///           Not: null
///           Or: null
///           Tags:
///             Key: CostCenter
///             MatchOptions: null
///             Values:
///               - '10000'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_anomaly_monitor` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/anomalyMonitor:AnomalyMonitor example costAnomalyMonitorARN
/// ```
pub mod anomaly_monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnomalyMonitorArgs {
        /// The dimensions to evaluate. Valid values: `SERVICE`.
        #[builder(into, default)]
        pub monitor_dimension: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid JSON representation for the [Expression](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html) object.
        #[builder(into, default)]
        pub monitor_specification: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The possible type values. Valid values: `DIMENSIONAL` | `CUSTOM`.
        #[builder(into)]
        pub monitor_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the monitor.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AnomalyMonitorResult {
        /// ARN of the anomaly monitor.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The dimensions to evaluate. Valid values: `SERVICE`.
        pub monitor_dimension: pulumi_gestalt_rust::Output<Option<String>>,
        /// A valid JSON representation for the [Expression](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html) object.
        pub monitor_specification: pulumi_gestalt_rust::Output<Option<String>>,
        /// The possible type values. Valid values: `DIMENSIONAL` | `CUSTOM`.
        pub monitor_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the monitor.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AnomalyMonitorArgs,
    ) -> AnomalyMonitorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let monitor_dimension_binding = args
            .monitor_dimension
            .get_output(context)
            .get_inner();
        let monitor_specification_binding = args
            .monitor_specification
            .get_output(context)
            .get_inner();
        let monitor_type_binding = args.monitor_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:costexplorer/anomalyMonitor:AnomalyMonitor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "monitorDimension".into(),
                    value: &monitor_dimension_binding,
                },
                register_interface::ObjectField {
                    name: "monitorSpecification".into(),
                    value: &monitor_specification_binding,
                },
                register_interface::ObjectField {
                    name: "monitorType".into(),
                    value: &monitor_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AnomalyMonitorResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            monitor_dimension: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorDimension"),
            ),
            monitor_specification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorSpecification"),
            ),
            monitor_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitorType"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
