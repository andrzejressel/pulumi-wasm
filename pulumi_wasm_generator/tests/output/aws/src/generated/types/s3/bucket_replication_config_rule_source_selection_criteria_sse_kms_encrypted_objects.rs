#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketReplicationConfigRuleSourceSelectionCriteriaSseKmsEncryptedObjects {
    /// Whether the existing objects should be replicated. Either `"Enabled"` or `"Disabled"`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
