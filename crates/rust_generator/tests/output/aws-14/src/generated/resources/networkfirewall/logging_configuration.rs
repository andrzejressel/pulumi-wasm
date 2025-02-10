/// Provides an AWS Network Firewall Logging Configuration Resource
///
/// ## Example Usage
///
/// ### Logging to S3
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:LoggingConfiguration
///     properties:
///       firewallArn: ${exampleAwsNetworkfirewallFirewall.arn}
///       loggingConfiguration:
///         logDestinationConfigs:
///           - logDestination:
///               bucketName: ${exampleAwsS3Bucket.bucket}
///               prefix: example
///             logDestinationType: S3
///             logType: FLOW
/// ```
///
/// ### Logging to CloudWatch
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:LoggingConfiguration
///     properties:
///       firewallArn: ${exampleAwsNetworkfirewallFirewall.arn}
///       loggingConfiguration:
///         logDestinationConfigs:
///           - logDestination:
///               logGroup: ${exampleAwsCloudwatchLogGroup.name}
///             logDestinationType: CloudWatchLogs
///             logType: ALERT
/// ```
///
/// ### Logging to Kinesis Data Firehose
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:LoggingConfiguration
///     properties:
///       firewallArn: ${exampleAwsNetworkfirewallFirewall.arn}
///       loggingConfiguration:
///         logDestinationConfigs:
///           - logDestination:
///               deliveryStream: ${exampleAwsKinesisFirehoseDeliveryStream.name}
///             logDestinationType: KinesisDataFirehose
///             logType: TLS
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall Logging Configurations using the `firewall_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:networkfirewall/loggingConfiguration:LoggingConfiguration example arn:aws:network-firewall:us-west-1:123456789012:firewall/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logging_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingConfigurationArgs {
        /// The Amazon Resource Name (ARN) of the Network Firewall firewall.
        #[builder(into)]
        pub firewall_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A configuration block describing how AWS Network Firewall performs logging for a firewall. See Logging Configuration below for details.
        #[builder(into)]
        pub logging_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkfirewall::LoggingConfigurationLoggingConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct LoggingConfigurationResult {
        /// The Amazon Resource Name (ARN) of the Network Firewall firewall.
        pub firewall_arn: pulumi_gestalt_rust::Output<String>,
        /// A configuration block describing how AWS Network Firewall performs logging for a firewall. See Logging Configuration below for details.
        pub logging_configuration: pulumi_gestalt_rust::Output<
            super::super::types::networkfirewall::LoggingConfigurationLoggingConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggingConfigurationArgs,
    ) -> LoggingConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let firewall_arn_binding = args.firewall_arn.get_output(context);
        let logging_configuration_binding = args
            .logging_configuration
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkfirewall/loggingConfiguration:LoggingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallArn".into(),
                    value: firewall_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfiguration".into(),
                    value: logging_configuration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoggingConfigurationResult {
            firewall_arn: o.get_field("firewallArn"),
            logging_configuration: o.get_field("loggingConfiguration"),
        }
    }
}
