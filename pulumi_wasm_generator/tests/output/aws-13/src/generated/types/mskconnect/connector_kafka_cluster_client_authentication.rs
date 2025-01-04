#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorKafkaClusterClientAuthentication {
    /// The type of client authentication used to connect to the Apache Kafka cluster. Valid values: `IAM`, `NONE`. A value of `NONE` means that no client authentication is used. The default value is `NONE`.
    #[builder(into, default)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<Option<String>>,
}
