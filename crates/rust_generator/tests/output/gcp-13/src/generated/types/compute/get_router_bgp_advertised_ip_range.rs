#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouterBgpAdvertisedIpRange {
    /// User-specified description for the IP range.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The IP range to advertise. The value must be a
    /// CIDR-formatted string.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<String>,
}
