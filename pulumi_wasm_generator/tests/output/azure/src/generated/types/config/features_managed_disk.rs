#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesManagedDisk {
    #[builder(into, default)]
    #[serde(rename = "expandWithoutDowntime")]
    pub r#expand_without_downtime: Box<Option<bool>>,
}
