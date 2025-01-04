#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobJobStorageAccount {
    /// The account key for the Azure storage account.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Box<String>,
    /// The name of the Azure storage account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The authentication mode of the storage account. The only supported value is `ConnectionString`. Defaults to `ConnectionString`.
    #[builder(into, default)]
    #[serde(rename = "authenticationMode")]
    pub r#authentication_mode: Box<Option<String>>,
}
