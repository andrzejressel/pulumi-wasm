#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfig {
    /// If True, restore all namespaced resources in the Backup.
    /// Setting this field to False will result in an error.
    #[builder(into, default)]
    #[serde(rename = "allNamespaces")]
    pub r#all_namespaces: Box<Option<bool>>,
    /// Defines the behavior for handling the situation where cluster-scoped resources
    /// being restored already exist in the target cluster.
    /// This MUST be set to a value other than `CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED`
    /// if `clusterResourceRestoreScope` is anyting other than `noGroupKinds`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy
    /// for more information on each policy option.
    /// Possible values are: `USE_EXISTING_VERSION`, `USE_BACKUP_VERSION`.
    #[builder(into, default)]
    #[serde(rename = "clusterResourceConflictPolicy")]
    pub r#cluster_resource_conflict_policy: Box<Option<String>>,
    /// Identifies the cluster-scoped resources to restore from the Backup.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clusterResourceRestoreScope")]
    pub r#cluster_resource_restore_scope: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScope>>,
    /// A list of selected namespaces excluded from restoration.
    /// All namespaces except those in this list will be restored.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludedNamespaces")]
    pub r#excluded_namespaces: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigExcludedNamespaces>>,
    /// Defines the behavior for handling the situation where sets of namespaced resources
    /// being restored already exist in the target cluster.
    /// This MUST be set to a value other than `NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED`
    /// if the `namespacedResourceRestoreScope` is anything other than `noNamespaces`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode
    /// for more information on each mode.
    /// Possible values are: `DELETE_AND_RESTORE`, `FAIL_ON_CONFLICT`, `MERGE_SKIP_ON_CONFLICT`, `MERGE_REPLACE_VOLUME_ON_CONFLICT`, `MERGE_REPLACE_ON_CONFLICT`.
    #[builder(into, default)]
    #[serde(rename = "namespacedResourceRestoreMode")]
    pub r#namespaced_resource_restore_mode: Box<Option<String>>,
    /// Do not restore any namespaced resources if set to "True".
    /// Specifying this field to "False" is not allowed.
    #[builder(into, default)]
    #[serde(rename = "noNamespaces")]
    pub r#no_namespaces: Box<Option<bool>>,
    /// It contains custom ordering to use on a Restore.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "restoreOrder")]
    pub r#restore_order: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrder>>,
    /// A list of selected ProtectedApplications to restore.
    /// The listed ProtectedApplications and all the resources
    /// to which they refer will be restored.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "selectedApplications")]
    pub r#selected_applications: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigSelectedApplications>>,
    /// A list of selected namespaces to restore from the Backup.
    /// The listed Namespaces and all resources contained in them will be restored.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "selectedNamespaces")]
    pub r#selected_namespaces: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigSelectedNamespaces>>,
    /// A list of transformation rules to be applied against Kubernetes
    /// resources as they are selected for restoration from a Backup.
    /// Rules are executed in order defined - this order matters,
    /// as changes made by a rule may impact the filtering logic of subsequent
    /// rules. An empty list means no transformation will occur.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "transformationRules")]
    pub r#transformation_rules: Box<Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRule>>>,
    /// Specifies the mechanism to be used to restore volume data.
    /// This should be set to a value other than `NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED`
    /// if the `namespacedResourceRestoreScope` is anything other than `noNamespaces`.
    /// If not specified, it will be treated as `NO_VOLUME_DATA_RESTORATION`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy
    /// for more information on each policy option.
    /// Possible values are: `RESTORE_VOLUME_DATA_FROM_BACKUP`, `REUSE_VOLUME_HANDLE_FROM_BACKUP`, `NO_VOLUME_DATA_RESTORATION`.
    #[builder(into, default)]
    #[serde(rename = "volumeDataRestorePolicy")]
    pub r#volume_data_restore_policy: Box<Option<String>>,
    /// A table that binds volumes by their scope to a restore policy. Bindings
    /// must have a unique scope. Any volumes not scoped in the bindings are
    /// subject to the policy defined in volume_data_restore_policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumeDataRestorePolicyBindings")]
    pub r#volume_data_restore_policy_bindings: Box<Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigVolumeDataRestorePolicyBinding>>>,
}
