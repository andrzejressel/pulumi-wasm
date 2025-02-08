#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCoreNetworkPolicyDocumentSegmentAction {
    /// Action to take for the chosen segment. Valid values: `create-route`, `share`, `send-via` and `send-to`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// A user-defined string describing the segment action.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// List of strings containing CIDRs. You can define the IPv4 and IPv6 CIDR notation for each AWS Region. For example, `10.1.0.0/16` or `2001:db8::/56`. This is an array of CIDR notation strings.
    #[builder(into, default)]
    #[serde(rename = "destinationCidrBlocks")]
    pub r#destination_cidr_blocks: Box<Option<Vec<String>>>,
    /// A list of strings. Valid values include `["blackhole"]` or a list of attachment ids.
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<String>>>,
    /// String. When `action` is `share`, a `mode` value of `attachment-route` places the attachment and return routes in each of the `share_with` segments. When `action` is `send-via`, indicates the mode used for packets. Valid values: `attachment-route`, `single-hop`, `dual-hop`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Name of the segment.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: Box<String>,
    /// A set subtraction of segments to not share with.
    #[builder(into, default)]
    #[serde(rename = "shareWithExcepts")]
    pub r#share_with_excepts: Box<Option<Vec<String>>>,
    /// A list of strings to share with. Must be a substring is all segments. Valid values include: `["*"]` or `["<segment-names>"]`.
    #[builder(into, default)]
    #[serde(rename = "shareWiths")]
    pub r#share_withs: Box<Option<Vec<String>>>,
    /// The network function groups and any edge overrides associated with the action.
    #[builder(into, default)]
    #[serde(rename = "via")]
    pub r#via: Box<Option<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionVia>>,
    /// The destination segments for the `send-via` or `send-to` `action`.
    #[builder(into, default)]
    #[serde(rename = "whenSentTo")]
    pub r#when_sent_to: Box<Option<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionWhenSentTo>>,
}
