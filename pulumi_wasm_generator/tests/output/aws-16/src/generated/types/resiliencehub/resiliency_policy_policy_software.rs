#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResiliencyPolicyPolicySoftware {
    /// Recovery Point Objective (RPO) as a Go duration.
    #[builder(into)]
    #[serde(rename = "rpo")]
    pub r#rpo: Box<String>,
    /// Recovery Time Objective (RTO) as a Go duration.
    #[builder(into)]
    #[serde(rename = "rto")]
    pub r#rto: Box<String>,
}
