/// Provides a CloudWatch RUM Metrics Destination resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metrics_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricsDestinationArgs {
        /// The name of the CloudWatch RUM app monitor that will send the metrics.
        #[builder(into)]
        pub app_monitor_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines the destination to send the metrics to. Valid values are `CloudWatch` and `Evidently`. If you specify `Evidently`, you must also specify the ARN of the CloudWatchEvidently experiment that is to be the destination and an IAM role that has permission to write to the experiment.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Use this parameter only if Destination is Evidently. This parameter specifies the ARN of the Evidently experiment that will receive the extended metrics.
        #[builder(into, default)]
        pub destination_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This parameter is required if Destination is Evidently. If Destination is CloudWatch, do not use this parameter.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricsDestinationResult {
        /// The name of the CloudWatch RUM app monitor that will send the metrics.
        pub app_monitor_name: pulumi_gestalt_rust::Output<String>,
        /// Defines the destination to send the metrics to. Valid values are `CloudWatch` and `Evidently`. If you specify `Evidently`, you must also specify the ARN of the CloudWatchEvidently experiment that is to be the destination and an IAM role that has permission to write to the experiment.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// Use this parameter only if Destination is Evidently. This parameter specifies the ARN of the Evidently experiment that will receive the extended metrics.
        pub destination_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// This parameter is required if Destination is Evidently. If Destination is CloudWatch, do not use this parameter.
        pub iam_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MetricsDestinationArgs,
    ) -> MetricsDestinationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_monitor_name_binding = args.app_monitor_name.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let destination_arn_binding = args.destination_arn.get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rum/metricsDestination:MetricsDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appMonitorName".into(),
                    value: app_monitor_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationArn".into(),
                    value: destination_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoleArn".into(),
                    value: iam_role_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MetricsDestinationResult {
            app_monitor_name: o.get_field("appMonitorName"),
            destination: o.get_field("destination"),
            destination_arn: o.get_field("destinationArn"),
            iam_role_arn: o.get_field("iamRoleArn"),
        }
    }
}
