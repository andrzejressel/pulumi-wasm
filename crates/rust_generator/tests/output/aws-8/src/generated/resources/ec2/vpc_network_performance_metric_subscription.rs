/// Provides a resource to manage an Infrastructure Performance subscription.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_network_performance_metric_subscription::create(
///         "example",
///         VpcNetworkPerformanceMetricSubscriptionArgs::builder()
///             .destination("us-west-1")
///             .source("us-east-1")
///             .build_struct(),
///     );
/// }
/// ```
pub mod vpc_network_performance_metric_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcNetworkPerformanceMetricSubscriptionArgs {
        /// The target Region or Availability Zone that the metric subscription is enabled for. For example, `eu-west-1`.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metric used for the enabled subscription. Valid values: `aggregate-latency`. Default: `aggregate-latency`.
        #[builder(into, default)]
        pub metric: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source Region or Availability Zone that the metric subscription is enabled for. For example, `us-east-1`.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The statistic used for the enabled subscription. Valid values: `p50`. Default: `p50`.
        #[builder(into, default)]
        pub statistic: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcNetworkPerformanceMetricSubscriptionResult {
        /// The target Region or Availability Zone that the metric subscription is enabled for. For example, `eu-west-1`.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// The metric used for the enabled subscription. Valid values: `aggregate-latency`. Default: `aggregate-latency`.
        pub metric: pulumi_gestalt_rust::Output<Option<String>>,
        /// The data aggregation time for the subscription.
        pub period: pulumi_gestalt_rust::Output<String>,
        /// The source Region or Availability Zone that the metric subscription is enabled for. For example, `us-east-1`.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// The statistic used for the enabled subscription. Valid values: `p50`. Default: `p50`.
        pub statistic: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcNetworkPerformanceMetricSubscriptionArgs,
    ) -> VpcNetworkPerformanceMetricSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_binding = args.destination.get_output(context).get_inner();
        let metric_binding = args.metric.get_output(context).get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let statistic_binding = args.statistic.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcNetworkPerformanceMetricSubscription:VpcNetworkPerformanceMetricSubscription"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "metric".into(),
                    value: &metric_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "statistic".into(),
                    value: &statistic_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcNetworkPerformanceMetricSubscriptionResult {
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            metric: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metric"),
            ),
            period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("period"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            statistic: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statistic"),
            ),
        }
    }
}
