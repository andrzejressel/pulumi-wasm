#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkGatewayPolicyGroup {
    /// Is this a Default Virtual Network Gateway Policy Group? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Box<Option<bool>>,
    /// The name of the Virtual Network Gateway Policy Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `policy_member` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "policyMembers")]
    pub r#policy_members: Box<Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroupPolicyMember>>,
    /// The priority for the Virtual Network Gateway Policy Group. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}
