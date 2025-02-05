#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterBinaryAuthorization {
    /// Enable Binary Authorization for this cluster.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Mode of operation for Binary Authorization policy evaluation.
    #[builder(into, default)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Box<Option<String>>,
}
