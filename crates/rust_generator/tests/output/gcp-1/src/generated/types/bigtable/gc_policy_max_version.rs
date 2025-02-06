#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GcPolicyMaxVersion {
    /// Number of version before applying the GC policy.
    /// 
    /// -----
    /// `gc_rules` include 2 fields:
    #[builder(into)]
    #[serde(rename = "number")]
    pub r#number: Box<i32>,
}
