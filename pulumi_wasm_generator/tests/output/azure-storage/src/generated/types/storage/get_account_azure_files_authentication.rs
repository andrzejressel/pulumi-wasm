#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountAzureFilesAuthentication {
    /// An `active_directory` block as documented below.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Box<Vec<super::super::types::storage::GetAccountAzureFilesAuthenticationActiveDirectory>>,
    /// The default share level permissions applied to all users.
    #[builder(into)]
    #[serde(rename = "defaultShareLevelPermission")]
    pub r#default_share_level_permission: Box<String>,
    /// The directory service used for this Storage Account.
    #[builder(into)]
    #[serde(rename = "directoryType")]
    pub r#directory_type: Box<String>,
}
