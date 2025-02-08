#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CacheNfsTargetNamespaceJunction {
    /// The name of the access policy applied to this target. Defaults to `default`.
    #[builder(into, default)]
    #[serde(rename = "accessPolicyName")]
    pub r#access_policy_name: Box<Option<String>>,
    /// The client-facing file path of this NFS target within the HPC Cache NFS Target.
    #[builder(into)]
    #[serde(rename = "namespacePath")]
    pub r#namespace_path: Box<String>,
    /// The NFS export of this NFS target within the HPC Cache NFS Target.
    #[builder(into)]
    #[serde(rename = "nfsExport")]
    pub r#nfs_export: Box<String>,
    /// The relative subdirectory path from the `nfs_export` to map to the `namespace_path`. Defaults to `""`, in which case the whole `nfs_export` is exported.
    #[builder(into, default)]
    #[serde(rename = "targetPath")]
    pub r#target_path: Box<Option<String>>,
}
