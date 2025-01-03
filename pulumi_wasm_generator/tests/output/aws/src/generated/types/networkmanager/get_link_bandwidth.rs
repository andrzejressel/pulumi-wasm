#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinkBandwidth {
    /// Download speed in Mbps.
    #[builder(into)]
    #[serde(rename = "downloadSpeed")]
    pub r#download_speed: Box<i32>,
    /// Upload speed in Mbps.
    #[builder(into)]
    #[serde(rename = "uploadSpeed")]
    pub r#upload_speed: Box<i32>,
}
