#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpcIpamPoolCidrCidrAuthorizationContext {
    /// The plain-text authorization message for the prefix and account.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// The signed authorization message for the prefix and account.
    #[builder(into, default)]
    #[serde(rename = "signature")]
    pub r#signature: Box<Option<String>>,
}