/// Provides a resource to manage an Infrastructure Performance subscription.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcNetworkPerformanceMetricSubscriptionArgs {
        /// The target Region or Availability Zone that the metric subscription is enabled for. For example, `eu-west-1`.
        #[builder(into)]
        pub destination: pulumi_wasm_rust::Output<String>,
        /// The metric used for the enabled subscription. Valid values: `aggregate-latency`. Default: `aggregate-latency`.
        #[builder(into, default)]
        pub metric: pulumi_wasm_rust::Output<Option<String>>,
        /// The source Region or Availability Zone that the metric subscription is enabled for. For example, `us-east-1`.
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
        /// The statistic used for the enabled subscription. Valid values: `p50`. Default: `p50`.
        #[builder(into, default)]
        pub statistic: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcNetworkPerformanceMetricSubscriptionResult {
        /// The target Region or Availability Zone that the metric subscription is enabled for. For example, `eu-west-1`.
        pub destination: pulumi_wasm_rust::Output<String>,
        /// The metric used for the enabled subscription. Valid values: `aggregate-latency`. Default: `aggregate-latency`.
        pub metric: pulumi_wasm_rust::Output<Option<String>>,
        /// The data aggregation time for the subscription.
        pub period: pulumi_wasm_rust::Output<String>,
        /// The source Region or Availability Zone that the metric subscription is enabled for. For example, `us-east-1`.
        pub source: pulumi_wasm_rust::Output<String>,
        /// The statistic used for the enabled subscription. Valid values: `p50`. Default: `p50`.
        pub statistic: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcNetworkPerformanceMetricSubscriptionArgs,
    ) -> VpcNetworkPerformanceMetricSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_binding = args.destination.get_inner();
        let metric_binding = args.metric.get_inner();
        let source_binding = args.source.get_inner();
        let statistic_binding = args.statistic.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcNetworkPerformanceMetricSubscription:VpcNetworkPerformanceMetricSubscription"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "metric".into(),
                },
                register_interface::ResultField {
                    name: "period".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "statistic".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcNetworkPerformanceMetricSubscriptionResult {
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            metric: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metric").unwrap(),
            ),
            period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("period").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            statistic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statistic").unwrap(),
            ),
        }
    }
}
