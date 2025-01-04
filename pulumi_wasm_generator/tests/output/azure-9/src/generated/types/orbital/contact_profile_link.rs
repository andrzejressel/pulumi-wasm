#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactProfileLink {
    /// A list of contact profile link channels. A `channels` block as defined below.
    #[builder(into)]
    #[serde(rename = "channels")]
    pub r#channels: Box<Vec<super::super::types::orbital::ContactProfileLinkChannel>>,
    /// Direction of the link. Possible values are `Uplink` and `Downlink`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// Name of the link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Polarization of the link. Possible values are `LHCP`, `RHCP`, `linearVertical` and `linearHorizontal`.
    #[builder(into)]
    #[serde(rename = "polarization")]
    pub r#polarization: Box<String>,
}
