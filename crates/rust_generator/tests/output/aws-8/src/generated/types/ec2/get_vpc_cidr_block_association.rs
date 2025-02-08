#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVpcCidrBlockAssociation {
    /// Association ID for the IPv4 CIDR block.
    #[builder(into)]
    #[serde(rename = "associationId")]
    pub r#association_id: Box<String>,
    /// Cidr block of the desired VPC.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
    /// Current state of the desired VPC.
    /// Can be either `"pending"` or `"available"`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
