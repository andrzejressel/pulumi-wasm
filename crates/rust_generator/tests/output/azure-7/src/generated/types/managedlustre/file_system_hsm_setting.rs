#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FileSystemHsmSetting {
    /// The resource ID of the storage container that is used for hydrating the namespace and archiving from the namespace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: Box<String>,
    /// The import prefix for the Azure Managed Lustre File System. Only blobs in the non-logging container that start with this path/prefix get hydrated into the cluster namespace. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** The roles `Contributor` and `Storage Blob Data Contributor` must be added to the Service Principal `HPC Cache Resource Provider` for the Storage Account. See official docs for more information.
    #[builder(into, default)]
    #[serde(rename = "importPrefix")]
    pub r#import_prefix: Box<Option<String>>,
    /// The resource ID of the storage container that is used for logging events and errors. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loggingContainerId")]
    pub r#logging_container_id: Box<String>,
}
