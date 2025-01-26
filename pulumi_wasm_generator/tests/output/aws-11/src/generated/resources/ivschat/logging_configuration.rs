/// Resource for managing an AWS IVS (Interactive Video) Chat Logging Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage - Logging to CloudWatch
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create("example", LogGroupArgs::builder().build_struct());
///     let exampleLoggingConfiguration = logging_configuration::create(
///         "exampleLoggingConfiguration",
///         LoggingConfigurationArgs::builder()
///             .destination_configuration(
///                 LoggingConfigurationDestinationConfiguration::builder()
///                     .cloudwatchLogs(
///                         LoggingConfigurationDestinationConfigurationCloudwatchLogs::builder()
///                             .logGroupName("${example.name}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic Usage - Logging to Kinesis Firehose with Extended S3
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kinesis:FirehoseDeliveryStream
///     properties:
///       name: pulumi-kinesis-firehose-extended-s3-example-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${exampleRole.arn}
///         bucketArn: ${exampleBucketV2.arn}
///       tags:
///         LogDeliveryEnabled: 'true'
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucketPrefix: tf-ivschat-logging-bucket
///   exampleBucketAclV2:
///     type: aws:s3:BucketAclV2
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       acl: private
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: firehose_example_role
///       assumeRolePolicy: ${assumeRole.json}
///   exampleLoggingConfiguration:
///     type: aws:ivschat:LoggingConfiguration
///     name: example
///     properties:
///       destinationConfiguration:
///         firehose:
///           deliveryStreamName: ${example.name}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - firehose.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Basic Usage - Logging to S3
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucketName: tf-ivschat-logging
///       forceDestroy: true
///   exampleLoggingConfiguration:
///     type: aws:ivschat:LoggingConfiguration
///     name: example
///     properties:
///       destinationConfiguration:
///         s3:
///           bucketName: ${example.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IVS (Interactive Video) Chat Logging Configuration using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ivschat/loggingConfiguration:LoggingConfiguration example arn:aws:ivschat:us-west-2:326937407773:logging-configuration/MMUQc8wcqZmC
/// ```
pub mod logging_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingConfigurationArgs {
        /// Object containing destination configuration for where chat activity will be logged. This object must contain exactly one of the following children arguments:
        #[builder(into, default)]
        pub destination_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::ivschat::LoggingConfigurationDestinationConfiguration,
            >,
        >,
        /// Logging Configuration name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoggingConfigurationResult {
        /// ARN of the Logging Configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Object containing destination configuration for where chat activity will be logged. This object must contain exactly one of the following children arguments:
        pub destination_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ivschat::LoggingConfigurationDestinationConfiguration,
            >,
        >,
        /// Logging Configuration name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// State of the Logging Configuration.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoggingConfigurationArgs,
    ) -> LoggingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_configuration_binding = args
            .destination_configuration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ivschat/loggingConfiguration:LoggingConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationConfiguration".into(),
                    value: &destination_configuration_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destinationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoggingConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
