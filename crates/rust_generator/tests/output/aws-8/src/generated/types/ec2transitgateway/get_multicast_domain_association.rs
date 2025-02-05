#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMulticastDomainAssociation {
    /// The ID of the subnet associated with the transit gateway multicast domain.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// The ID of the transit gateway attachment.
    #[builder(into)]
    #[serde(rename = "transitGatewayAttachmentId")]
    pub r#transit_gateway_attachment_id: Box<String>,
}
