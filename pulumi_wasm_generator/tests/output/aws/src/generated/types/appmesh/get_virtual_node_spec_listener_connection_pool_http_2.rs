#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerConnectionPoolHttp2 {
    #[builder(into)]
    #[serde(rename = "maxRequests")]
    pub r#max_requests: Box<i32>,
}
