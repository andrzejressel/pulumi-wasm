#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HadoopClusterComponentVersion {
    /// The version of Hadoop which should be used for this HDInsight Hadoop Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hadoop")]
    pub r#hadoop: Box<String>,
}