#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSigningJobRevocationRecord {
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Box<String>,
    #[builder(into)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<String>,
    #[builder(into)]
    #[serde(rename = "revokedBy")]
    pub r#revoked_by: Box<String>,
}