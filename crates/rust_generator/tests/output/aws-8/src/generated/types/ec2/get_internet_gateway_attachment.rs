#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInternetGatewayAttachment {
    /// Current state of the attachment between the gateway and the VPC. Present only if a VPC is attached
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// ID of an attached VPC.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
