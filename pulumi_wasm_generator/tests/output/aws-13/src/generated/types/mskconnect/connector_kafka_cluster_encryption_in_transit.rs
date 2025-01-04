#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorKafkaClusterEncryptionInTransit {
    /// The type of encryption in transit to the Apache Kafka cluster. Valid values: `PLAINTEXT`, `TLS`. The default values is `PLAINTEXT`.
    #[builder(into, default)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Box<Option<String>>,
}
