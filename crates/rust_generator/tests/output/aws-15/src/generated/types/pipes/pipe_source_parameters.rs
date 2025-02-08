#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeSourceParameters {
    /// The parameters for using an Active MQ broker as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "activemqBrokerParameters")]
    pub r#activemq_broker_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersActivemqBrokerParameters>>,
    /// The parameters for using a DynamoDB stream as a source.  Detailed below.
    #[builder(into, default)]
    #[serde(rename = "dynamodbStreamParameters")]
    pub r#dynamodb_stream_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersDynamodbStreamParameters>>,
    /// The collection of event patterns used to [filter events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes-event-filtering.html). Detailed below.
    #[builder(into, default)]
    #[serde(rename = "filterCriteria")]
    pub r#filter_criteria: Box<Option<super::super::types::pipes::PipeSourceParametersFilterCriteria>>,
    /// The parameters for using a Kinesis stream as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "kinesisStreamParameters")]
    pub r#kinesis_stream_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersKinesisStreamParameters>>,
    /// The parameters for using an MSK stream as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "managedStreamingKafkaParameters")]
    pub r#managed_streaming_kafka_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersManagedStreamingKafkaParameters>>,
    /// The parameters for using a Rabbit MQ broker as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "rabbitmqBrokerParameters")]
    pub r#rabbitmq_broker_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersRabbitmqBrokerParameters>>,
    /// The parameters for using a self-managed Apache Kafka stream as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "selfManagedKafkaParameters")]
    pub r#self_managed_kafka_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersSelfManagedKafkaParameters>>,
    /// The parameters for using a Amazon SQS stream as a source. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "sqsQueueParameters")]
    pub r#sqs_queue_parameters: Box<Option<super::super::types::pipes::PipeSourceParametersSqsQueueParameters>>,
}
