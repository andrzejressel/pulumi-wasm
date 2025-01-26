/// Creates and manages an AWS IoT topic rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   rule:
///     type: aws:iot:TopicRule
///     properties:
///       name: MyRule
///       description: Example rule
///       enabled: true
///       sql: SELECT * FROM 'topic/test'
///       sqlVersion: 2016-03-23
///       sns:
///         - messageFormat: RAW
///           roleArn: ${role.arn}
///           targetArn: ${mytopic.arn}
///       errorAction:
///         sns:
///           messageFormat: RAW
///           roleArn: ${role.arn}
///           targetArn: ${myerrortopic.arn}
///   mytopic:
///     type: aws:sns:Topic
///     properties:
///       name: mytopic
///   myerrortopic:
///     type: aws:sns:Topic
///     properties:
///       name: myerrortopic
///   myrole:
///     type: aws:iam:Role
///     properties:
///       name: myrole
///       assumeRolePolicy: ${assumeRole.json}
///   mypolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: mypolicy
///     properties:
///       name: mypolicy
///       role: ${myrole.id}
///       policy: ${mypolicy.json}
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
///                   - iot.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   mypolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - sns:Publish
///             resources:
///               - ${mytopic.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Topic Rules using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iot/topicRule:TopicRule rule <name>
/// ```
pub mod topic_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicRuleArgs {
        #[builder(into, default)]
        pub cloudwatch_alarms: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchAlarm>>,
        >,
        #[builder(into, default)]
        pub cloudwatch_logs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchLog>>,
        >,
        #[builder(into, default)]
        pub cloudwatch_metrics: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchMetric>>,
        >,
        /// The description of the rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub dynamodbs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleDynamodb>>,
        >,
        #[builder(into, default)]
        pub dynamodbv2s: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleDynamodbv2>>,
        >,
        #[builder(into, default)]
        pub elasticsearch: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleElasticsearch>>,
        >,
        /// Specifies whether the rule is enabled.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Configuration block with error action to be associated with the rule. See the documentation for `cloudwatch_alarm`, `cloudwatch_logs`, `cloudwatch_metric`, `dynamodb`, `dynamodbv2`, `elasticsearch`, `firehose`, `http`, `iot_analytics`, `iot_events`, `kafka`, `kinesis`, `lambda`, `republish`, `s3`, `sns`, `sqs`, `step_functions`, `timestream` configuration blocks for further configuration details.
        #[builder(into, default)]
        pub error_action: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::TopicRuleErrorAction>,
        >,
        #[builder(into, default)]
        pub firehoses: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleFirehose>>,
        >,
        #[builder(into, default)]
        pub https: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleHttp>>,
        >,
        #[builder(into, default)]
        pub iot_analytics: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleIotAnalytic>>,
        >,
        #[builder(into, default)]
        pub iot_events: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleIotEvent>>,
        >,
        #[builder(into, default)]
        pub kafkas: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleKafka>>,
        >,
        #[builder(into, default)]
        pub kineses: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleKinesis>>,
        >,
        #[builder(into, default)]
        pub lambdas: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleLambda>>,
        >,
        /// The name of the rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub republishes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleRepublish>>,
        >,
        #[builder(into, default)]
        pub s3: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleS3>>,
        >,
        #[builder(into, default)]
        pub sns: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleSns>>,
        >,
        /// The SQL statement used to query the topic. For more information, see AWS IoT SQL Reference (http://docs.aws.amazon.com/iot/latest/developerguide/iot-rules.html#aws-iot-sql-reference) in the AWS IoT Developer Guide.
        #[builder(into)]
        pub sql: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of the SQL rules engine to use when evaluating the rule.
        #[builder(into)]
        pub sql_version: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub sqs: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleSqs>>,
        >,
        #[builder(into, default)]
        pub step_functions: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleStepFunction>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timestreams: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::TopicRuleTimestream>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicRuleResult {
        /// The ARN of the topic rule
        pub arn: pulumi_wasm_rust::Output<String>,
        pub cloudwatch_alarms: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchAlarm>>,
        >,
        pub cloudwatch_logs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchLog>>,
        >,
        pub cloudwatch_metrics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleCloudwatchMetric>>,
        >,
        /// The description of the rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub dynamodbs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleDynamodb>>,
        >,
        pub dynamodbv2s: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleDynamodbv2>>,
        >,
        pub elasticsearch: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleElasticsearch>>,
        >,
        /// Specifies whether the rule is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Configuration block with error action to be associated with the rule. See the documentation for `cloudwatch_alarm`, `cloudwatch_logs`, `cloudwatch_metric`, `dynamodb`, `dynamodbv2`, `elasticsearch`, `firehose`, `http`, `iot_analytics`, `iot_events`, `kafka`, `kinesis`, `lambda`, `republish`, `s3`, `sns`, `sqs`, `step_functions`, `timestream` configuration blocks for further configuration details.
        pub error_action: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::TopicRuleErrorAction>,
        >,
        pub firehoses: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleFirehose>>,
        >,
        pub https: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleHttp>>,
        >,
        pub iot_analytics: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleIotAnalytic>>,
        >,
        pub iot_events: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleIotEvent>>,
        >,
        pub kafkas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleKafka>>,
        >,
        pub kineses: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleKinesis>>,
        >,
        pub lambdas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleLambda>>,
        >,
        /// The name of the rule.
        pub name: pulumi_wasm_rust::Output<String>,
        pub republishes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleRepublish>>,
        >,
        pub s3: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleS3>>,
        >,
        pub sns: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleSns>>,
        >,
        /// The SQL statement used to query the topic. For more information, see AWS IoT SQL Reference (http://docs.aws.amazon.com/iot/latest/developerguide/iot-rules.html#aws-iot-sql-reference) in the AWS IoT Developer Guide.
        pub sql: pulumi_wasm_rust::Output<String>,
        /// The version of the SQL rules engine to use when evaluating the rule.
        pub sql_version: pulumi_wasm_rust::Output<String>,
        pub sqs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleSqs>>,
        >,
        pub step_functions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleStepFunction>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timestreams: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::TopicRuleTimestream>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TopicRuleArgs,
    ) -> TopicRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudwatch_alarms_binding = args
            .cloudwatch_alarms
            .get_output(context)
            .get_inner();
        let cloudwatch_logs_binding = args
            .cloudwatch_logs
            .get_output(context)
            .get_inner();
        let cloudwatch_metrics_binding = args
            .cloudwatch_metrics
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let dynamodbs_binding = args.dynamodbs.get_output(context).get_inner();
        let dynamodbv2s_binding = args.dynamodbv2s.get_output(context).get_inner();
        let elasticsearch_binding = args.elasticsearch.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let error_action_binding = args.error_action.get_output(context).get_inner();
        let firehoses_binding = args.firehoses.get_output(context).get_inner();
        let https_binding = args.https.get_output(context).get_inner();
        let iot_analytics_binding = args.iot_analytics.get_output(context).get_inner();
        let iot_events_binding = args.iot_events.get_output(context).get_inner();
        let kafkas_binding = args.kafkas.get_output(context).get_inner();
        let kineses_binding = args.kineses.get_output(context).get_inner();
        let lambdas_binding = args.lambdas.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let republishes_binding = args.republishes.get_output(context).get_inner();
        let s3_binding = args.s3.get_output(context).get_inner();
        let sns_binding = args.sns.get_output(context).get_inner();
        let sql_binding = args.sql.get_output(context).get_inner();
        let sql_version_binding = args.sql_version.get_output(context).get_inner();
        let sqs_binding = args.sqs.get_output(context).get_inner();
        let step_functions_binding = args.step_functions.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timestreams_binding = args.timestreams.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/topicRule:TopicRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudwatchAlarms".into(),
                    value: &cloudwatch_alarms_binding,
                },
                register_interface::ObjectField {
                    name: "cloudwatchLogs".into(),
                    value: &cloudwatch_logs_binding,
                },
                register_interface::ObjectField {
                    name: "cloudwatchMetrics".into(),
                    value: &cloudwatch_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dynamodbs".into(),
                    value: &dynamodbs_binding,
                },
                register_interface::ObjectField {
                    name: "dynamodbv2s".into(),
                    value: &dynamodbv2s_binding,
                },
                register_interface::ObjectField {
                    name: "elasticsearch".into(),
                    value: &elasticsearch_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "errorAction".into(),
                    value: &error_action_binding,
                },
                register_interface::ObjectField {
                    name: "firehoses".into(),
                    value: &firehoses_binding,
                },
                register_interface::ObjectField {
                    name: "https".into(),
                    value: &https_binding,
                },
                register_interface::ObjectField {
                    name: "iotAnalytics".into(),
                    value: &iot_analytics_binding,
                },
                register_interface::ObjectField {
                    name: "iotEvents".into(),
                    value: &iot_events_binding,
                },
                register_interface::ObjectField {
                    name: "kafkas".into(),
                    value: &kafkas_binding,
                },
                register_interface::ObjectField {
                    name: "kineses".into(),
                    value: &kineses_binding,
                },
                register_interface::ObjectField {
                    name: "lambdas".into(),
                    value: &lambdas_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "republishes".into(),
                    value: &republishes_binding,
                },
                register_interface::ObjectField {
                    name: "s3".into(),
                    value: &s3_binding,
                },
                register_interface::ObjectField {
                    name: "sns".into(),
                    value: &sns_binding,
                },
                register_interface::ObjectField {
                    name: "sql".into(),
                    value: &sql_binding,
                },
                register_interface::ObjectField {
                    name: "sqlVersion".into(),
                    value: &sql_version_binding,
                },
                register_interface::ObjectField {
                    name: "sqs".into(),
                    value: &sqs_binding,
                },
                register_interface::ObjectField {
                    name: "stepFunctions".into(),
                    value: &step_functions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timestreams".into(),
                    value: &timestreams_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cloudwatch_alarms: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudwatchAlarms"),
            ),
            cloudwatch_logs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudwatchLogs"),
            ),
            cloudwatch_metrics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudwatchMetrics"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dynamodbs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dynamodbs"),
            ),
            dynamodbv2s: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dynamodbv2s"),
            ),
            elasticsearch: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticsearch"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            error_action: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("errorAction"),
            ),
            firehoses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firehoses"),
            ),
            https: pulumi_wasm_rust::__private::into_domain(o.extract_field("https")),
            iot_analytics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iotAnalytics"),
            ),
            iot_events: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iotEvents"),
            ),
            kafkas: pulumi_wasm_rust::__private::into_domain(o.extract_field("kafkas")),
            kineses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kineses"),
            ),
            lambdas: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lambdas"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            republishes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("republishes"),
            ),
            s3: pulumi_wasm_rust::__private::into_domain(o.extract_field("s3")),
            sns: pulumi_wasm_rust::__private::into_domain(o.extract_field("sns")),
            sql: pulumi_wasm_rust::__private::into_domain(o.extract_field("sql")),
            sql_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqlVersion"),
            ),
            sqs: pulumi_wasm_rust::__private::into_domain(o.extract_field("sqs")),
            step_functions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stepFunctions"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timestreams: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timestreams"),
            ),
        }
    }
}
