#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KafkaClusterComponentVersion {
    /// The version of Kafka which should be used for this HDInsight Kafka Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "kafka")]
    pub r#kafka: Box<String>,
}
