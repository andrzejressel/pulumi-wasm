#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionDiskAsyncPrimaryDisk {
    /// Primary disk for asynchronous disk replication.
    #[builder(into)]
    #[serde(rename = "disk")]
    pub r#disk: Box<String>,
}
