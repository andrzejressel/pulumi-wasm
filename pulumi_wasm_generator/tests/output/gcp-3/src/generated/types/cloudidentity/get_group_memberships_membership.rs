#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupMembershipsMembership {
    /// The time when the Membership was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// The parent Group resource under which to lookup the Membership names. Must be of the form groups/{group_id}.
    #[builder(into)]
    #[serde(rename = "group")]
    pub r#group: Box<String>,
    /// EntityKey of the member.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "memberKeys")]
    pub r#member_keys: Box<Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipMemberKey>>,
    /// The name of the MembershipRole. One of OWNER, MANAGER, MEMBER.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// EntityKey of the member.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preferredMemberKeys")]
    pub r#preferred_member_keys: Box<Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipPreferredMemberKey>>,
    /// The MembershipRoles that apply to the Membership. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Box<Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipRole>>,
    /// The type of the membership.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// The time when the Membership was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}
