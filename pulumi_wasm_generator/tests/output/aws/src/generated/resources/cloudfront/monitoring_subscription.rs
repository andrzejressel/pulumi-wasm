/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = monitoring_subscription::create(
///         "example",
///         MonitoringSubscriptionArgs::builder()
///             .distribution_id("${exampleAwsCloudfrontDistribution.id}")
///             .monitoring_subscription(
///                 MonitoringSubscriptionMonitoringSubscription::builder()
///                     .realtimeMetricsSubscriptionConfig(
///                         MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig::builder()
///                             .realtimeMetricsSubscriptionStatus("Enabled")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront monitoring subscription using the id. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/monitoringSubscription:MonitoringSubscription example E3QYSUHO4VYRGB
/// ```
pub mod monitoring_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionArgs {
        /// The ID of the distribution that you are enabling metrics for.
        #[builder(into)]
        pub distribution_id: pulumi_wasm_rust::Output<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        #[builder(into)]
        pub monitoring_subscription: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionResult {
        /// The ID of the distribution that you are enabling metrics for.
        pub distribution_id: pulumi_wasm_rust::Output<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        pub monitoring_subscription: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MonitoringSubscriptionArgs,
    ) -> MonitoringSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let distribution_id_binding = args.distribution_id.get_inner();
        let monitoring_subscription_binding = args.monitoring_subscription.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/monitoringSubscription:MonitoringSubscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "distributionId".into(),
                    value: &distribution_id_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringSubscription".into(),
                    value: &monitoring_subscription_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "distributionId".into(),
                },
                register_interface::ResultField {
                    name: "monitoringSubscription".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MonitoringSubscriptionResult {
            distribution_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributionId").unwrap(),
            ),
            monitoring_subscription: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringSubscription").unwrap(),
            ),
        }
    }
}