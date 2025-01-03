#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGceSetupServiceAccount {
    /// Optional. Email address of the service account.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// (Output)
    /// Output only. The list of scopes to be made available for this
    /// service account. Set by the CLH to https://www.googleapis.com/auth/cloud-platform
    #[builder(into, default)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
}
