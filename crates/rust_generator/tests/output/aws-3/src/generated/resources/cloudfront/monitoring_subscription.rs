/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod monitoring_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionArgs {
        /// The ID of the distribution that you are enabling metrics for.
        #[builder(into)]
        pub distribution_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        #[builder(into)]
        pub monitoring_subscription: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitoringSubscriptionResult {
        /// The ID of the distribution that you are enabling metrics for.
        pub distribution_id: pulumi_gestalt_rust::Output<String>,
        /// A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
        pub monitoring_subscription: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscription,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MonitoringSubscriptionArgs,
    ) -> MonitoringSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let distribution_id_binding = args
            .distribution_id
            .get_output(context)
            .get_inner();
        let monitoring_subscription_binding = args
            .monitoring_subscription
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/monitoringSubscription:MonitoringSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitoringSubscriptionResult {
            distribution_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distributionId"),
            ),
            monitoring_subscription: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringSubscription"),
            ),
        }
    }
}
