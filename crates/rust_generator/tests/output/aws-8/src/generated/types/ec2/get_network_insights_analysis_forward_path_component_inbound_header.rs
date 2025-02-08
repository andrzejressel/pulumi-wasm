#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInsightsAnalysisForwardPathComponentInboundHeader {
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentInboundHeaderDestinationPortRange>>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentInboundHeaderSourcePortRange>>,
}
