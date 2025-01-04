#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallFirewallStatusSyncStateAttachment {
    /// The identifier of the firewall endpoint that AWS Network Firewall has instantiated in the subnet. You use this to identify the firewall endpoint in the VPC route tables, when you redirect the VPC traffic through the endpoint.
    #[builder(into)]
    #[serde(rename = "endpointId")]
    pub r#endpoint_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// The unique identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
