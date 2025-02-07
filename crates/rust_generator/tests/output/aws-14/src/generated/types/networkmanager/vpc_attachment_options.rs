#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpcAttachmentOptions {
    /// Indicates whether appliance mode is supported.
    /// If enabled, traffic flow between a source and destination use the same Availability Zone for the VPC attachment for the lifetime of that flow.
    /// If the VPC attachment is pending acceptance, changing this value will recreate the resource.
    #[builder(into, default)]
    #[serde(rename = "applianceModeSupport")]
    pub r#appliance_mode_support: Box<Option<bool>>,
    /// Indicates whether IPv6 is supported.
    /// If the VPC attachment is pending acceptance, changing this value will recreate the resource.
    #[builder(into, default)]
    #[serde(rename = "ipv6Support")]
    pub r#ipv_6_support: Box<Option<bool>>,
}
