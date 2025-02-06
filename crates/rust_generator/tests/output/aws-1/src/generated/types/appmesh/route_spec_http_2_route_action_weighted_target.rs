#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecHttp2RouteActionWeightedTarget {
    /// The targeted port of the weighted object.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Virtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualNode")]
    pub r#virtual_node: Box<String>,
    /// Relative weight of the weighted target. An integer between 0 and 100.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
