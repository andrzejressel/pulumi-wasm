#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecGrpcRouteMatchMetadataMatchRange {
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}
