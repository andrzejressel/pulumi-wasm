#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataLakeGen2FilesystemAce {
    /// Specifies the Object ID of the Azure Active Directory User or Group that the entry relates to. Only valid for `user` or `group` entries.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the permissions for the entry in `rwx` form. For example, `rwx` gives full permissions but `r--` only gives read permissions.
    /// 
    /// More details on ACLs can be found here: <https://docs.microsoft.com/azure/storage/blobs/data-lake-storage-access-control#access-control-lists-on-files-and-directories>
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    /// Specifies whether the ACE represents an `access` entry or a `default` entry. Default value is `access`.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// Specifies the type of entry. Can be `user`, `group`, `mask` or `other`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
