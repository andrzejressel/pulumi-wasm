#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpacecraftLink {
    /// Bandwidth in Mhz.
    #[builder(into)]
    #[serde(rename = "bandwidthMhz")]
    pub r#bandwidth_mhz: Box<f64>,
    /// Center frequency in Mhz.
    /// 
    /// > **Note:** The value of `center_frequency_mhz +/- bandwidth_mhz / 2` should fall in one of these ranges: `Uplink/LHCP`: [2025, 2120]; `Uplink/Linear`: [399, 403],[435, 438],[449, 451]; `Uplink/RHCP`: [399, 403],[435, 438],[449, 451],[2025, 2120]; `Downlink/LHCP`: [2200, 2300], [7500, 8400]; `Downlink/Linear`: [399, 403], [435, 438], [449, 451]; Downlink/Linear`: [399, 403], [435, 438], [449, 451], [2200, 2300], [7500, 8400]
    #[builder(into)]
    #[serde(rename = "centerFrequencyMhz")]
    pub r#center_frequency_mhz: Box<f64>,
    /// Direction if the communication. Possible values are `Uplink` and `Downlink`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// Name of the link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Polarization. Possible values are `RHCP`, `LHCP`, `linearVertical` and `linearHorizontal`.
    #[builder(into)]
    #[serde(rename = "polarization")]
    pub r#polarization: Box<String>,
}
