#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleRoleLastUsed {
    /// The date and time, in RFC 3339 format, that the role was last used.
    #[builder(into)]
    #[serde(rename = "lastUsedDate")]
    pub r#last_used_date: Box<String>,
    /// The name of the AWS Region in which the role was last used.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
