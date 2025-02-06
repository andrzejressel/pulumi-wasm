#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MultiRegionAccessPointDetailsRegion {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "bucketAccountId")]
    pub r#bucket_account_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}
