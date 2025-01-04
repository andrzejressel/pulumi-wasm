#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterEncryptionInfoEncryptionInTransit {
    /// Encryption setting for data in transit between clients and brokers. Valid values: `TLS`, `TLS_PLAINTEXT`, and `PLAINTEXT`. Default value is `TLS`.
    #[builder(into, default)]
    #[serde(rename = "clientBroker")]
    pub r#client_broker: Box<Option<String>>,
    /// Whether data communication among broker nodes is encrypted. Default value: `true`.
    #[builder(into, default)]
    #[serde(rename = "inCluster")]
    pub r#in_cluster: Box<Option<bool>>,
}
