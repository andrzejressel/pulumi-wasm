#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLoadBalancerAccessLogs {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
}
