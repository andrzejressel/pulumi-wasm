/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: cloudfront-realtime-log-config-example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: cloudfront-realtime-log-config-example
///       role: ${exampleRole.id}
///       policy: ${example.json}
///   exampleRealtimeLogConfig:
///     type: aws:cloudfront:RealtimeLogConfig
///     name: example
///     properties:
///       name: example
///       samplingRate: 75
///       fields:
///         - timestamp
///         - c-ip
///       endpoint:
///         streamType: Kinesis
///         kinesisStreamConfig:
///           roleArn: ${exampleRole.arn}
///           streamArn: ${exampleAwsKinesisStream.arn}
///     options:
///       dependsOn:
///         - ${exampleRolePolicy}
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
///                   - cloudfront.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - kinesis:DescribeStreamSummary
///               - kinesis:DescribeStream
///               - kinesis:PutRecord
///               - kinesis:PutRecords
///             resources:
///               - ${exampleAwsKinesisStream.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront real-time log configurations using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/realtimeLogConfig:RealtimeLogConfig example arn:aws:cloudfront::111122223333:realtime-log-config/ExampleNameForRealtimeLogConfig
/// ```
pub mod realtime_log_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RealtimeLogConfigArgs {
        /// The Amazon Kinesis data streams where real-time log data is sent.
        #[builder(into)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        #[builder(into)]
        pub sampling_rate: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct RealtimeLogConfigResult {
        /// The ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Kinesis data streams where real-time log data is sent.
        pub endpoint: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        pub fields: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        pub sampling_rate: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RealtimeLogConfigArgs,
    ) -> RealtimeLogConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let endpoint_binding = args.endpoint.get_output(context).get_inner();
        let fields_binding = args.fields.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sampling_rate_binding = args.sampling_rate.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/realtimeLogConfig:RealtimeLogConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "samplingRate".into(),
                    value: &sampling_rate_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RealtimeLogConfigResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fields"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            sampling_rate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("samplingRate"),
            ),
        }
    }
}
