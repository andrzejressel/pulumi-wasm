#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinkBandwidth {
    /// Download speed in Mbps.
    #[builder(into, default)]
    #[serde(rename = "downloadSpeed")]
    pub r#download_speed: Box<Option<i32>>,
    /// Upload speed in Mbps.
    #[builder(into, default)]
    #[serde(rename = "uploadSpeed")]
    pub r#upload_speed: Box<Option<i32>>,
}
