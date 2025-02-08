#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BrokerInstance {
    /// The URL of the [ActiveMQ Web Console](http://activemq.apache.org/web-console.html) or the [RabbitMQ Management UI](https://www.rabbitmq.com/management.html#external-monitoring) depending on `engine_type`.
    #[builder(into, default)]
    #[serde(rename = "consoleUrl")]
    pub r#console_url: Box<Option<String>>,
    /// Broker's wire-level protocol endpoints in the following order & format referenceable e.g., as `instances.0.endpoints.0` (SSL):
    /// * For `ActiveMQ`:
    /// * `ssl://broker-id.mq.us-west-2.amazonaws.com:61617`
    /// * `amqp+ssl://broker-id.mq.us-west-2.amazonaws.com:5671`
    /// * `stomp+ssl://broker-id.mq.us-west-2.amazonaws.com:61614`
    /// * `mqtt+ssl://broker-id.mq.us-west-2.amazonaws.com:8883`
    /// * `wss://broker-id.mq.us-west-2.amazonaws.com:61619`
    /// * For `RabbitMQ`:
    /// * `amqps://broker-id.mq.us-west-2.amazonaws.com:5671`
    #[builder(into, default)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Option<Vec<String>>>,
    /// IP Address of the broker.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
}
