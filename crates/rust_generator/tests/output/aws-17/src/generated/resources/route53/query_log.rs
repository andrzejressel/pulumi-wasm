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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod query_log {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueryLogArgs {
        /// CloudWatch log group ARN to send query logs.
        #[builder(into)]
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Route53 hosted zone ID to enable query logs.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct QueryLogResult {
        /// The Amazon Resource Name (ARN) of the Query Logging Config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// CloudWatch log group ARN to send query logs.
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::Output<String>,
        /// Route53 hosted zone ID to enable query logs.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueryLogArgs,
    ) -> QueryLogResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloudwatch_log_group_arn_binding = args
            .cloudwatch_log_group_arn
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/queryLog:QueryLog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchLogGroupArn".into(),
                    value: cloudwatch_log_group_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueryLogResult {
            arn: o.get_field("arn"),
            cloudwatch_log_group_arn: o.get_field("cloudwatchLogGroupArn"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
