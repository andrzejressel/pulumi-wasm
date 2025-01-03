#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupTransitiveMembershipsMembership {
    /// Resource name for this member.
    #[builder(into)]
    #[serde(rename = "member")]
    pub r#member: Box<String>,
    /// EntityKey of the member. Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace.
    #[builder(into)]
    #[serde(rename = "preferredMemberKeys")]
    pub r#preferred_member_keys: Box<Vec<super::super::types::cloudidentity::GetGroupTransitiveMembershipsMembershipPreferredMemberKey>>,
    /// The relation between the group and the transitive member. The value can be DIRECT, INDIRECT, or DIRECT_AND_INDIRECT
    #[builder(into)]
    #[serde(rename = "relationType")]
    pub r#relation_type: Box<String>,
    /// The membership role details
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Box<Vec<super::super::types::cloudidentity::GetGroupTransitiveMembershipsMembershipRole>>,
}
