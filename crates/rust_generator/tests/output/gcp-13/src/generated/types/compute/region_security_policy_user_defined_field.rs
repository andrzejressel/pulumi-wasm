#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionSecurityPolicyUserDefinedField {
    /// The base relative to which 'offset' is measured. Possible values are:
    /// - IPV4: Points to the beginning of the IPv4 header.
    /// - IPV6: Points to the beginning of the IPv6 header.
    /// - TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.
    /// - UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.
    /// Possible values are: `IPV4`, `IPV6`, `TCP`, `UDP`.
    #[builder(into)]
    #[serde(rename = "base")]
    pub r#base: Box<String>,
    /// If specified, apply this mask (bitwise AND) to the field to ignore bits before matching.
    /// Encoded as a hexadecimal number (starting with "0x").
    /// The last byte of the field (in network byte order) corresponds to the least significant byte of the mask.
    #[builder(into, default)]
    #[serde(rename = "mask")]
    pub r#mask: Box<Option<String>>,
    /// Name of the user-defined field, as given in the definition.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Offset of the first byte of the field (in network byte order) relative to 'base'.
    #[builder(into, default)]
    #[serde(rename = "offset")]
    pub r#offset: Box<Option<i32>>,
    /// Size of the field in bytes. Valid values: 1-4.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<i32>>,
}
