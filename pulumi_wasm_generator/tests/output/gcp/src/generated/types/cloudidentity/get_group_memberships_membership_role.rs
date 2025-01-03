#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMembershipsMembershipRole {
    /// The MembershipRole expiry details, only supported for MEMBER role.
    /// Other roles cannot be accompanied with MEMBER role having expiry.
    #[builder(into)]
    #[serde(rename = "expiryDetails")]
    pub r#expiry_details: Box<Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipRoleExpiryDetail>>,
    /// The name of the MembershipRole. One of OWNER, MANAGER, MEMBER.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
