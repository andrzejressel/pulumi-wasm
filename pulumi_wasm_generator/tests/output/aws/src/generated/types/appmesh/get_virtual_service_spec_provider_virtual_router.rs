#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualServiceSpecProviderVirtualRouter {
    #[builder(into)]
    #[serde(rename = "virtualRouterName")]
    pub r#virtual_router_name: Box<String>,
}
