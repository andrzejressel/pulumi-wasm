#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkGatewayPolicyGroupPolicyMember {
    /// The name of the Virtual Network Gateway Policy Group Member.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The VPN Policy Member attribute type. Possible values are `AADGroupId`, `CertificateGroupId` and `RadiusAzureGroupId`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The value of attribute that is used for this Virtual Network Gateway Policy Group Member.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
