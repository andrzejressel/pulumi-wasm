#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCoreNetworkPolicyDocumentCoreNetworkConfigurationEdgeLocation {
    /// ASN of the Core Network Edge in an AWS Region. By default, the ASN will be a single integer automatically assigned from `asn_ranges`
    #[builder(into, default)]
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<String>>,
    /// The local CIDR blocks for this Core Network Edge for AWS Transit Gateway Connect attachments. By default, this CIDR block will be one or more optional IPv4 and IPv6 CIDR prefixes auto-assigned from `inside_cidr_blocks`.
    #[builder(into, default)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
