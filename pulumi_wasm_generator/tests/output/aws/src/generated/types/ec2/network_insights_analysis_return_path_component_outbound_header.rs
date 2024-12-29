#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisReturnPathComponentOutboundHeader {
    #[builder(into, default)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeaderDestinationPortRange>>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeaderSourcePortRange>>>,
}
