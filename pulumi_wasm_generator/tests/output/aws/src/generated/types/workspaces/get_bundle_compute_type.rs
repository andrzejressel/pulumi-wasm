#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBundleComputeType {
    /// Name of the bundle. You cannot combine this parameter with `bundle_id`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}