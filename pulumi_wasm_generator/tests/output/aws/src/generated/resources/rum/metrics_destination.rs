/// Provides a CloudWatch RUM Metrics Destination resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = metrics_destination::create(
///         "example",
///         MetricsDestinationArgs::builder()
///             .app_monitor_name("${exampleAwsRumAppMonitor.name}")
///             .destination("CloudWatch")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch RUM Metrics Destination using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rum/metricsDestination:MetricsDestination example example
/// ```
pub mod metrics_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricsDestinationArgs {
        /// The name of the CloudWatch RUM app monitor that will send the metrics.
        #[builder(into)]
        pub app_monitor_name: pulumi_wasm_rust::Output<String>,
        /// Defines the destination to send the metrics to. Valid values are `CloudWatch` and `Evidently`. If you specify `Evidently`, you must also specify the ARN of the CloudWatchEvidently experiment that is to be the destination and an IAM role that has permission to write to the experiment.
        #[builder(into)]
        pub destination: pulumi_wasm_rust::Output<String>,
        /// Use this parameter only if Destination is Evidently. This parameter specifies the ARN of the Evidently experiment that will receive the extended metrics.
        #[builder(into, default)]
        pub destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// This parameter is required if Destination is Evidently. If Destination is CloudWatch, do not use this parameter.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricsDestinationResult {
        /// The name of the CloudWatch RUM app monitor that will send the metrics.
        pub app_monitor_name: pulumi_wasm_rust::Output<String>,
        /// Defines the destination to send the metrics to. Valid values are `CloudWatch` and `Evidently`. If you specify `Evidently`, you must also specify the ARN of the CloudWatchEvidently experiment that is to be the destination and an IAM role that has permission to write to the experiment.
        pub destination: pulumi_wasm_rust::Output<String>,
        /// Use this parameter only if Destination is Evidently. This parameter specifies the ARN of the Evidently experiment that will receive the extended metrics.
        pub destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// This parameter is required if Destination is Evidently. If Destination is CloudWatch, do not use this parameter.
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MetricsDestinationArgs) -> MetricsDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_monitor_name_binding = args.app_monitor_name.get_inner();
        let destination_binding = args.destination.get_inner();
        let destination_arn_binding = args.destination_arn.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rum/metricsDestination:MetricsDestination".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appMonitorName".into(),
                    value: &app_monitor_name_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appMonitorName".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetricsDestinationResult {
            app_monitor_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appMonitorName").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
        }
    }
}