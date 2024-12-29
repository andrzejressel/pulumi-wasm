#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventEndpointRoutingConfigFailoverConfigSecondary {
    /// The name of the secondary Region.
    #[builder(into, default)]
    #[serde(rename = "route")]
    pub r#route: Box<Option<String>>,
}
