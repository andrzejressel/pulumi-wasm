#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupMembershipRole {
    /// The MembershipRole expiry details, only supported for MEMBER role.
    /// Other roles cannot be accompanied with MEMBER role having expiry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "expiryDetail")]
    pub r#expiry_detail: Box<Option<super::super::types::cloudidentity::GroupMembershipRoleExpiryDetail>>,
    /// The name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER.
    /// Possible values are: `OWNER`, `MANAGER`, `MEMBER`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
