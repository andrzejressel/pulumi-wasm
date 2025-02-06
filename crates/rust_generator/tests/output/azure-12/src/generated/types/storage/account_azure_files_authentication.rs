#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountAzureFilesAuthentication {
    /// A `active_directory` block as defined below. Required when `directory_type` is `AD`.
    #[builder(into, default)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Box<Option<super::super::types::storage::AccountAzureFilesAuthenticationActiveDirectory>>,
    /// Specifies the default share level permissions applied to all users. Possible values are `StorageFileDataSmbShareReader`, `StorageFileDataSmbShareContributor`, `StorageFileDataSmbShareElevatedContributor`, or `None`.
    #[builder(into, default)]
    #[serde(rename = "defaultShareLevelPermission")]
    pub r#default_share_level_permission: Box<Option<String>>,
    /// Specifies the directory service used. Possible values are `AADDS`, `AD` and `AADKERB`.
    #[builder(into)]
    #[serde(rename = "directoryType")]
    pub r#directory_type: Box<String>,
}
