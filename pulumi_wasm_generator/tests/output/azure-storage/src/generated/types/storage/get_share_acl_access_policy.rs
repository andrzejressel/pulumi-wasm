#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetShareAclAccessPolicy {
    /// The time at which this Access Policy is valid until.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<String>,
    /// The permissions which should be associated with this Shared Identifier. Possible value is combination of `r` (read), `w` (write), `d` (delete), and `l` (list).
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    /// The time at which this Access Policy is valid from.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}
