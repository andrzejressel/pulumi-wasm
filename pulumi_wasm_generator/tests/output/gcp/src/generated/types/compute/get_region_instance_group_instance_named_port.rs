#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionInstanceGroupInstanceNamedPort {
    /// The name of the instance group.  One of `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Integer port number
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
