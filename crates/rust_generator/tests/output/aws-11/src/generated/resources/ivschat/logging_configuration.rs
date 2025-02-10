/// Resource for managing an AWS IVS (Interactive Video) Chat Logging Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage - Logging to CloudWatch
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logging_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingConfigurationArgs {
        /// Object containing destination configuration for where chat activity will be logged. This object must contain exactly one of the following children arguments:
        #[builder(into, default)]
        pub destination_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ivschat::LoggingConfigurationDestinationConfiguration,
            >,
        >,
        /// Logging Configuration name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoggingConfigurationResult {
        /// ARN of the Logging Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Object containing destination configuration for where chat activity will be logged. This object must contain exactly one of the following children arguments:
        pub destination_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ivschat::LoggingConfigurationDestinationConfiguration,
            >,
        >,
        /// Logging Configuration name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the Logging Configuration.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
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
        let destination_configuration_binding = args
            .destination_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ivschat/loggingConfiguration:LoggingConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationConfiguration".into(),
                    value: destination_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoggingConfigurationResult {
            arn: o.get_field("arn"),
            destination_configuration: o.get_field("destinationConfiguration"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
