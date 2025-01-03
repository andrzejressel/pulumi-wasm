#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BatchEnvironmentConfigPeripheralsConfig {
    /// Resource name of an existing Dataproc Metastore service.
    #[builder(into, default)]
    #[serde(rename = "metastoreService")]
    pub r#metastore_service: Box<Option<String>>,
    /// The Spark History Server configuration for the workload.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sparkHistoryServerConfig")]
    pub r#spark_history_server_config: Box<Option<super::super::types::dataproc::BatchEnvironmentConfigPeripheralsConfigSparkHistoryServerConfig>>,
}
