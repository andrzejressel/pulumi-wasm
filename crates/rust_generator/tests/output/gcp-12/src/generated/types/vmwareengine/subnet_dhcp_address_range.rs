#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubnetDhcpAddressRange {
    /// (Output)
    /// The first IP address of the range.
    #[builder(into, default)]
    #[serde(rename = "firstAddress")]
    pub r#first_address: Box<Option<String>>,
    /// (Output)
    /// The last IP address of the range.
    #[builder(into, default)]
    #[serde(rename = "lastAddress")]
    pub r#last_address: Box<Option<String>>,
}
