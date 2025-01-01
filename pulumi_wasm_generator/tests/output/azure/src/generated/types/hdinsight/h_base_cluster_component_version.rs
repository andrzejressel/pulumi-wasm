#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HBaseClusterComponentVersion {
    /// The version of HBase which should be used for this HDInsight HBase Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hbase")]
    pub r#hbase: Box<String>,
}
