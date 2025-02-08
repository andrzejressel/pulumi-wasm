#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetEc2InboundPermission {
    /// Starting value for a range of allowed port numbers.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Box<i32>,
    /// Range of allowed IP addresses expressed in CIDR notationE.g., `000.000.000.000/[subnet mask]` or `0.0.0.0/[subnet mask]`.
    #[builder(into)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<String>,
    /// Network communication protocol used by the fleetE.g., `TCP` or `UDP`
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Ending value for a range of allowed port numbers. Port numbers are end-inclusive. This value must be higher than `from_port`.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: Box<i32>,
}
