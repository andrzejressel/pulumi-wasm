#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TopicRuleErrorActionKafka {
    /// Properties of the Apache Kafka producer client. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/apache-kafka-rule-action.html).
    #[builder(into)]
    #[serde(rename = "clientProperties")]
    pub r#client_properties: Box<std::collections::HashMap<String, String>>,
    /// The ARN of Kafka action's VPC `aws.iot.TopicRuleDestination`.
    #[builder(into)]
    #[serde(rename = "destinationArn")]
    pub r#destination_arn: Box<String>,
    /// The list of Kafka headers that you specify. Nested arguments below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::iot::TopicRuleErrorActionKafkaHeader>>>,
    /// The Kafka message key.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// The Kafka message partition.
    #[builder(into, default)]
    #[serde(rename = "partition")]
    pub r#partition: Box<Option<String>>,
    /// The Kafka topic for messages to be sent to the Kafka broker.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
