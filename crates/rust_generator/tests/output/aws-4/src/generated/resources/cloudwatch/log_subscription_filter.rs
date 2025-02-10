/// Provides a CloudWatch Logs subscription filter resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testLambdafunctionLogfilter = log_subscription_filter::create(
///         "testLambdafunctionLogfilter",
///         LogSubscriptionFilterArgs::builder()
///             .destination_arn("${testLogstream.arn}")
///             .distribution("Random")
///             .filter_pattern("logtype test")
///             .log_group("/aws/lambda/example_lambda_name")
///             .name("test_lambdafunction_logfilter")
///             .role_arn("${iamForLambda.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Logs subscription filter using the log group name and subscription filter name separated by `|`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logSubscriptionFilter:LogSubscriptionFilter test_lambdafunction_logfilter "/aws/lambda/example_lambda_name|test_lambdafunction_logfilter"
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_subscription_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogSubscriptionFilterArgs {
        /// The ARN of the destination to deliver matching log events to. Kinesis stream or Lambda function ARN.
        #[builder(into)]
        pub destination_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The method used to distribute log data to the destination. By default log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream. Valid values are "Random" and "ByLogStream".
        #[builder(into, default)]
        pub distribution: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid CloudWatch Logs filter pattern for subscribing to a filtered stream of log events. Use empty string `""` to match everything. For more information, see the [Amazon CloudWatch Logs User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        #[builder(into)]
        pub filter_pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the log group to associate the subscription filter with
        #[builder(into)]
        pub log_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for the subscription filter
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to deliver ingested log events to the destination. If you use Lambda as a destination, you should skip this argument and use `aws.lambda.Permission` resource for granting access from CloudWatch logs to the destination Lambda function.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogSubscriptionFilterResult {
        /// The ARN of the destination to deliver matching log events to. Kinesis stream or Lambda function ARN.
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// The method used to distribute log data to the destination. By default log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream. Valid values are "Random" and "ByLogStream".
        pub distribution: pulumi_gestalt_rust::Output<Option<String>>,
        /// A valid CloudWatch Logs filter pattern for subscribing to a filtered stream of log events. Use empty string `""` to match everything. For more information, see the [Amazon CloudWatch Logs User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        pub filter_pattern: pulumi_gestalt_rust::Output<String>,
        /// The name of the log group to associate the subscription filter with
        pub log_group: pulumi_gestalt_rust::Output<String>,
        /// A name for the subscription filter
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to deliver ingested log events to the destination. If you use Lambda as a destination, you should skip this argument and use `aws.lambda.Permission` resource for granting access from CloudWatch logs to the destination Lambda function.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogSubscriptionFilterArgs,
    ) -> LogSubscriptionFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_arn_binding = args.destination_arn.get_output(context);
        let distribution_binding = args.distribution.get_output(context);
        let filter_pattern_binding = args.filter_pattern.get_output(context);
        let log_group_binding = args.log_group.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logSubscriptionFilter:LogSubscriptionFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationArn".into(),
                    value: destination_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distribution".into(),
                    value: distribution_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterPattern".into(),
                    value: filter_pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroup".into(),
                    value: log_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogSubscriptionFilterResult {
            destination_arn: o.get_field("destinationArn"),
            distribution: o.get_field("distribution"),
            filter_pattern: o.get_field("filterPattern"),
            log_group: o.get_field("logGroup"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
