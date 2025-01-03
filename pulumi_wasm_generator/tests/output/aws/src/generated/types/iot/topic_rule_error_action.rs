#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorAction {
    #[builder(into, default)]
    #[serde(rename = "cloudwatchAlarm")]
    pub r#cloudwatch_alarm: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchAlarm>>,
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchLogs>>,
    #[builder(into, default)]
    #[serde(rename = "cloudwatchMetric")]
    pub r#cloudwatch_metric: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchMetric>>,
    #[builder(into, default)]
    #[serde(rename = "dynamodb")]
    pub r#dynamodb: Box<Option<super::super::types::iot::TopicRuleErrorActionDynamodb>>,
    #[builder(into, default)]
    #[serde(rename = "dynamodbv2")]
    pub r#dynamodbv_2: Box<Option<super::super::types::iot::TopicRuleErrorActionDynamodbv2>>,
    #[builder(into, default)]
    #[serde(rename = "elasticsearch")]
    pub r#elasticsearch: Box<Option<super::super::types::iot::TopicRuleErrorActionElasticsearch>>,
    #[builder(into, default)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<super::super::types::iot::TopicRuleErrorActionFirehose>>,
    #[builder(into, default)]
    #[serde(rename = "http")]
    pub r#http: Box<Option<super::super::types::iot::TopicRuleErrorActionHttp>>,
    #[builder(into, default)]
    #[serde(rename = "iotAnalytics")]
    pub r#iot_analytics: Box<Option<super::super::types::iot::TopicRuleErrorActionIotAnalytics>>,
    #[builder(into, default)]
    #[serde(rename = "iotEvents")]
    pub r#iot_events: Box<Option<super::super::types::iot::TopicRuleErrorActionIotEvents>>,
    #[builder(into, default)]
    #[serde(rename = "kafka")]
    pub r#kafka: Box<Option<super::super::types::iot::TopicRuleErrorActionKafka>>,
    #[builder(into, default)]
    #[serde(rename = "kinesis")]
    pub r#kinesis: Box<Option<super::super::types::iot::TopicRuleErrorActionKinesis>>,
    #[builder(into, default)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<Option<super::super::types::iot::TopicRuleErrorActionLambda>>,
    #[builder(into, default)]
    #[serde(rename = "republish")]
    pub r#republish: Box<Option<super::super::types::iot::TopicRuleErrorActionRepublish>>,
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::iot::TopicRuleErrorActionS3>>,
    #[builder(into, default)]
    #[serde(rename = "sns")]
    pub r#sns: Box<Option<super::super::types::iot::TopicRuleErrorActionSns>>,
    #[builder(into, default)]
    #[serde(rename = "sqs")]
    pub r#sqs: Box<Option<super::super::types::iot::TopicRuleErrorActionSqs>>,
    #[builder(into, default)]
    #[serde(rename = "stepFunctions")]
    pub r#step_functions: Box<Option<super::super::types::iot::TopicRuleErrorActionStepFunctions>>,
    #[builder(into, default)]
    #[serde(rename = "timestream")]
    pub r#timestream: Box<Option<super::super::types::iot::TopicRuleErrorActionTimestream>>,
}
