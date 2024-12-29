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
///               prefix: /example
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
pub mod logging_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingConfigurationArgs {
        /// The Amazon Resource Name (ARN) of the Network Firewall firewall.
        #[builder(into)]
        pub firewall_arn: pulumi_wasm_rust::Output<String>,
        /// A configuration block describing how AWS Network Firewall performs logging for a firewall. See Logging Configuration below for details.
        #[builder(into)]
        pub logging_configuration: pulumi_wasm_rust::Output<
            super::super::types::networkfirewall::LoggingConfigurationLoggingConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct LoggingConfigurationResult {
        /// The Amazon Resource Name (ARN) of the Network Firewall firewall.
        pub firewall_arn: pulumi_wasm_rust::Output<String>,
        /// A configuration block describing how AWS Network Firewall performs logging for a firewall. See Logging Configuration below for details.
        pub logging_configuration: pulumi_wasm_rust::Output<
            super::super::types::networkfirewall::LoggingConfigurationLoggingConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LoggingConfigurationArgs,
    ) -> LoggingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let firewall_arn_binding = args.firewall_arn.get_inner();
        let logging_configuration_binding = args.logging_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkfirewall/loggingConfiguration:LoggingConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallArn".into(),
                    value: &firewall_arn_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfiguration".into(),
                    value: &logging_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "firewallArn".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoggingConfigurationResult {
            firewall_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallArn").unwrap(),
            ),
            logging_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfiguration").unwrap(),
            ),
        }
    }
}
