#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CoreNetworkEdge {
    /// ASN of a core network edge.
    #[builder(into, default)]
    #[serde(rename = "asn")]
    pub r#asn: Box<Option<i32>>,
    /// Region where a core network edge is located.
    #[builder(into, default)]
    #[serde(rename = "edgeLocation")]
    pub r#edge_location: Box<Option<String>>,
    /// Inside IP addresses used for core network edges.
    #[builder(into, default)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Box<Option<Vec<String>>>,
}
