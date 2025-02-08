#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupInitContainerVolume {
    /// Boolean as to whether the mounted volume should be an empty directory. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Box<Option<bool>>,
    /// A `git_repo` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "gitRepo")]
    pub r#git_repo: Box<Option<super::super::types::containerservice::GroupInitContainerVolumeGitRepo>>,
    /// The path on which this volume is to be mounted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// The name of the volume mount. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specify if the volume is to be mounted as read only or not. The default value is `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// A map of secrets that will be mounted as files in the volume. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Exactly one of `empty_dir` volume, `git_repo` volume, `secret` volume or storage account volume (`share_name`, `storage_account_name`, and `storage_account_key`) must be specified.
    /// 
    /// > **Note** when using a storage account volume, all of `share_name`, `storage_account_name`, and `storage_account_key` must be specified.
    /// 
    /// > **Note:** The secret values must be supplied as Base64 encoded strings. The secret values are decoded to their original values when mounted in the volume on the container.
    #[builder(into, default)]
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Azure storage share that is to be mounted as a volume. This must be created on the storage account specified as above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<Option<String>>,
    /// The access key for the Azure Storage account specified as above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "storageAccountKey")]
    pub r#storage_account_key: Box<Option<String>>,
    /// The Azure storage account from which the volume is to be mounted. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Box<Option<String>>,
}
