#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcIamPoolCidrsIpamPoolCidr {
    /// A network CIDR.
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    /// The provisioning state of that CIDR.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
