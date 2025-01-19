/// Provides a Route53 query logging configuration resource.
///
/// > **NOTE:** There are restrictions on the configuration of query logging. Notably,
/// the CloudWatch log group must be in the `us-east-1` region,
/// a permissive CloudWatch log resource policy must be in place, and
/// the Route53 hosted zone must be public.
/// See [Configuring Logging for DNS Queries](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html?console_help=true#query-logs-configuring) for additional details.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   awsRoute53ExampleCom:
///     type: aws:cloudwatch:LogGroup
///     name: aws_route53_example_com
///     properties:
///       name: /aws/route53/${exampleCom.name}
///       retentionInDays: 30
///   route53-query-logging-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: route53-query-logging-policy
///     properties:
///       policyDocument: ${["route53-query-logging-policy"].json}
///       policyName: route53-query-logging-policy
///   # Example Route53 zone with query logging
///   exampleCom:
///     type: aws:route53:Zone
///     name: example_com
///     properties:
///       name: example.com
///   exampleComQueryLog:
///     type: aws:route53:QueryLog
///     name: example_com
///     properties:
///       cloudwatchLogGroupArn: ${awsRoute53ExampleCom.arn}
///       zoneId: ${exampleCom.zoneId}
///     options:
///       dependsOn:
///         - ${["route53-query-logging-policyLogResourcePolicy"]}
/// variables:
///   # Example CloudWatch log resource policy to allow Route53 to write logs
///   # to any log group under /aws/route53/*
///   route53-query-logging-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             resources:
///               - arn:aws:logs:*:*:log-group:/aws/route53/*
///             principals:
///               - identifiers:
///                   - route53.amazonaws.com
///                 type: Service
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 query logging configurations using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/queryLog:QueryLog example_com xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
pub mod query_log {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueryLogArgs {
        /// CloudWatch log group ARN to send query logs.
        #[builder(into)]
        pub cloudwatch_log_group_arn: pulumi_wasm_rust::Output<String>,
        /// Route53 hosted zone ID to enable query logs.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct QueryLogResult {
        /// The Amazon Resource Name (ARN) of the Query Logging Config.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// CloudWatch log group ARN to send query logs.
        pub cloudwatch_log_group_arn: pulumi_wasm_rust::Output<String>,
        /// Route53 hosted zone ID to enable query logs.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueryLogArgs) -> QueryLogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudwatch_log_group_arn_binding = args.cloudwatch_log_group_arn.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/queryLog:QueryLog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudwatchLogGroupArn".into(),
                    value: &cloudwatch_log_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cloudwatchLogGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueryLogResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cloudwatch_log_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudwatchLogGroupArn").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
