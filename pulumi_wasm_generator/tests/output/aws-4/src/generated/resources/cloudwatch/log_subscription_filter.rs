/// Provides a CloudWatch Logs subscription filter resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod log_subscription_filter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogSubscriptionFilterArgs {
        /// The ARN of the destination to deliver matching log events to. Kinesis stream or Lambda function ARN.
        #[builder(into)]
        pub destination_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The method used to distribute log data to the destination. By default log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream. Valid values are "Random" and "ByLogStream".
        #[builder(into, default)]
        pub distribution: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A valid CloudWatch Logs filter pattern for subscribing to a filtered stream of log events. Use empty string `""` to match everything. For more information, see the [Amazon CloudWatch Logs User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        #[builder(into)]
        pub filter_pattern: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the log group to associate the subscription filter with
        #[builder(into)]
        pub log_group: pulumi_wasm_rust::InputOrOutput<String>,
        /// A name for the subscription filter
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to deliver ingested log events to the destination. If you use Lambda as a destination, you should skip this argument and use `aws.lambda.Permission` resource for granting access from CloudWatch logs to the destination Lambda function.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogSubscriptionFilterResult {
        /// The ARN of the destination to deliver matching log events to. Kinesis stream or Lambda function ARN.
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// The method used to distribute log data to the destination. By default log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream. Valid values are "Random" and "ByLogStream".
        pub distribution: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid CloudWatch Logs filter pattern for subscribing to a filtered stream of log events. Use empty string `""` to match everything. For more information, see the [Amazon CloudWatch Logs User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        pub filter_pattern: pulumi_wasm_rust::Output<String>,
        /// The name of the log group to associate the subscription filter with
        pub log_group: pulumi_wasm_rust::Output<String>,
        /// A name for the subscription filter
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to deliver ingested log events to the destination. If you use Lambda as a destination, you should skip this argument and use `aws.lambda.Permission` resource for granting access from CloudWatch logs to the destination Lambda function.
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LogSubscriptionFilterArgs,
    ) -> LogSubscriptionFilterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_arn_binding = args
            .destination_arn
            .get_output(context)
            .get_inner();
        let distribution_binding = args.distribution.get_output(context).get_inner();
        let filter_pattern_binding = args.filter_pattern.get_output(context).get_inner();
        let log_group_binding = args.log_group.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logSubscriptionFilter:LogSubscriptionFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "distribution".into(),
                    value: &distribution_binding,
                },
                register_interface::ObjectField {
                    name: "filterPattern".into(),
                    value: &filter_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "logGroup".into(),
                    value: &log_group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "distribution".into(),
                },
                register_interface::ResultField {
                    name: "filterPattern".into(),
                },
                register_interface::ResultField {
                    name: "logGroup".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogSubscriptionFilterResult {
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            distribution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distribution").unwrap(),
            ),
            filter_pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterPattern").unwrap(),
            ),
            log_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroup").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}
