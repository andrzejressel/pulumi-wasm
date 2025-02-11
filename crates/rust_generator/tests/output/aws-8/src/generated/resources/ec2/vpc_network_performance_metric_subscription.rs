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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcNetworkPerformanceMetricSubscriptionArgs,
    ) -> VpcNetworkPerformanceMetricSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_binding = args.destination.get_output(context);
        let metric_binding = args.metric.get_output(context);
        let source_binding = args.source.get_output(context);
        let statistic_binding = args.statistic.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcNetworkPerformanceMetricSubscription:VpcNetworkPerformanceMetricSubscription"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: &destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metric".into(),
                    value: &metric_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statistic".into(),
                    value: &statistic_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcNetworkPerformanceMetricSubscriptionResult {
            destination: o.get_field("destination"),
            metric: o.get_field("metric"),
            period: o.get_field("period"),
            source: o.get_field("source"),
            statistic: o.get_field("statistic"),
        }
    }
}
