#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactProfileLinkChannel {
    /// Bandwidth in MHz.
    #[builder(into)]
    #[serde(rename = "bandwidthMhz")]
    pub r#bandwidth_mhz: Box<f64>,
    /// Center frequency in MHz.
    #[builder(into)]
    #[serde(rename = "centerFrequencyMhz")]
    pub r#center_frequency_mhz: Box<f64>,
    /// Copy of the modem configuration file such as Kratos QRadio or Kratos QuantumRx. Only valid for downlink directions. If provided, the modem connects to the customer endpoint and sends demodulated data instead of a VITA.49 stream.
    #[builder(into, default)]
    #[serde(rename = "demodulationConfiguration")]
    pub r#demodulation_configuration: Box<Option<String>>,
    /// Customer End point to store/retrieve data during a contact. An `end_point` block as defined below.
    #[builder(into)]
    #[serde(rename = "endPoints")]
    pub r#end_points: Box<Vec<super::super::types::orbital::ContactProfileLinkChannelEndPoint>>,
    /// Copy of the modem configuration file such as Kratos QRadio. Only valid for uplink directions. If provided, the modem connects to the customer endpoint and accepts commands from the customer instead of a VITA.49 stream.
    #[builder(into, default)]
    #[serde(rename = "modulationConfiguration")]
    pub r#modulation_configuration: Box<Option<String>>,
    /// Name of the channel.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
