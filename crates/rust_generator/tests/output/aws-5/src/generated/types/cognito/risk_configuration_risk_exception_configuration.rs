#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RiskConfigurationRiskExceptionConfiguration {
    /// Overrides the risk decision to always block the pre-authentication requests.
    /// The IP range is in CIDR notation, a compact representation of an IP address and its routing prefix.
    /// Can contain a maximum of 200 items.
    #[builder(into, default)]
    #[serde(rename = "blockedIpRangeLists")]
    pub r#blocked_ip_range_lists: Box<Option<Vec<String>>>,
    /// Risk detection isn't performed on the IP addresses in this range list.
    /// The IP range is in CIDR notation.
    /// Can contain a maximum of 200 items.
    #[builder(into, default)]
    #[serde(rename = "skippedIpRangeLists")]
    pub r#skipped_ip_range_lists: Box<Option<Vec<String>>>,
}
