/// Represents a Restore Plan instance.
///
///
/// To get more information about RestorePlan, see:
///
/// * [API documentation](https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/projects.locations.restorePlans)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke)
///
/// ## Example Usage
///
/// ### Gkebackup Restoreplan All Namespaces
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let allNs = restore_plan::create(
///         "allNs",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("restore-all-ns")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .allNamespaces(true)
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .allGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("FAIL_ON_CONFLICT")
///                     .volumeDataRestorePolicy("RESTORE_VOLUME_DATA_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("restore-all-ns")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("restore-all-ns-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Rollback Namespace
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rollback-ns")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("rollback-ns-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let rollbackNs = restore_plan::create(
///         "rollbackNs",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rollback-ns-rp")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .selectedGroupKinds(
///                                 vec![
///                                     RestorePlanRestoreConfigClusterResourceRestoreScopeSelectedGroupKind::builder()
///                                     .resourceGroup("apiextension.k8s.io")
///                                     .resourceKind("CustomResourceDefinition").build_struct(),
///                                     RestorePlanRestoreConfigClusterResourceRestoreScopeSelectedGroupKind::builder()
///                                     .resourceGroup("storage.k8s.io")
///                                     .resourceKind("StorageClass").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("DELETE_AND_RESTORE")
///                     .selectedNamespaces(
///                         RestorePlanRestoreConfigSelectedNamespaces::builder()
///                             .namespaces(vec!["my-ns",])
///                             .build_struct(),
///                     )
///                     .volumeDataRestorePolicy("RESTORE_VOLUME_DATA_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Protected Application
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rollback-app")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("rollback-app-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let rollbackApp = restore_plan::create(
///         "rollbackApp",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rollback-app-rp")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .noGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("DELETE_AND_RESTORE")
///                     .selectedApplications(
///                         RestorePlanRestoreConfigSelectedApplications::builder()
///                             .namespacedNames(
///                                 vec![
///                                     RestorePlanRestoreConfigSelectedApplicationsNamespacedName::builder()
///                                     .name("my-app").namespace("my-ns").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .volumeDataRestorePolicy("REUSE_VOLUME_HANDLE_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan All Cluster Resources
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let allClusterResources = restore_plan::create(
///         "allClusterResources",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("all-groupkinds-rp")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .allGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("FAIL_ON_CONFLICT")
///                     .noNamespaces(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("all-groupkinds")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("all-groupkinds-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Rename Namespace
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rename-ns")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("rename-ns-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let renameNs = restore_plan::create(
///         "renameNs",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("rename-ns-rp")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .noGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("FAIL_ON_CONFLICT")
///                     .selectedNamespaces(
///                         RestorePlanRestoreConfigSelectedNamespaces::builder()
///                             .namespaces(vec!["ns1",])
///                             .build_struct(),
///                     )
///                     .transformationRules(
///                         vec![
///                             RestorePlanRestoreConfigTransformationRule::builder()
///                             .description("rename namespace from ns1 to ns2")
///                             .fieldActions(vec![RestorePlanRestoreConfigTransformationRuleFieldAction::builder()
///                             .op("REPLACE").path("/metadata/name").value("ns2")
///                             .build_struct(),])
///                             .resourceFilter(RestorePlanRestoreConfigTransformationRuleResourceFilter::builder()
///                             .groupKinds(vec![RestorePlanRestoreConfigTransformationRuleResourceFilterGroupKind::builder()
///                             .resourceKind("Namespace").build_struct(),])
///                             .jsonPath(".metadata[?(@.name == 'ns1')]").build_struct())
///                             .build_struct(),
///                             RestorePlanRestoreConfigTransformationRule::builder()
///                             .description("move all resources from ns1 to ns2")
///                             .fieldActions(vec![RestorePlanRestoreConfigTransformationRuleFieldAction::builder()
///                             .op("REPLACE").path("/metadata/namespace").value("ns2")
///                             .build_struct(),])
///                             .resourceFilter(RestorePlanRestoreConfigTransformationRuleResourceFilter::builder()
///                             .namespaces(vec!["ns1",]).build_struct()).build_struct(),
///                         ],
///                     )
///                     .volumeDataRestorePolicy("REUSE_VOLUME_HANDLE_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Second Transformation
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:Cluster
///     properties:
///       name: transform-rule-cluster
///       location: us-central1
///       initialNodeCount: 1
///       workloadIdentityConfig:
///         workloadPool: my-project-name.svc.id.goog
///       addonsConfig:
///         gkeBackupAgentConfig:
///           enabled: true
///       deletionProtection: true
///       network: default
///       subnetwork: default
///   basic:
///     type: gcp:gkebackup:BackupPlan
///     properties:
///       name: transform-rule
///       cluster: ${primary.id}
///       location: us-central1
///       backupConfig:
///         includeVolumeData: true
///         includeSecrets: true
///         allNamespaces: true
///   transformRule:
///     type: gcp:gkebackup:RestorePlan
///     name: transform_rule
///     properties:
///       name: transform-rule-rp
///       description: copy nginx env variables
///       labels:
///         app: nginx
///       location: us-central1
///       backupPlan: ${basic.id}
///       cluster: ${primary.id}
///       restoreConfig:
///         excludedNamespaces:
///           namespaces:
///             - my-ns
///         namespacedResourceRestoreMode: DELETE_AND_RESTORE
///         volumeDataRestorePolicy: RESTORE_VOLUME_DATA_FROM_BACKUP
///         clusterResourceRestoreScope:
///           excludedGroupKinds:
///             - resourceGroup: apiextension.k8s.io
///               resourceKind: CustomResourceDefinition
///         clusterResourceConflictPolicy: USE_EXISTING_VERSION
///         transformationRules:
///           - description: Copy environment variables from the nginx container to the install init container.
///             resourceFilter:
///               groupKinds:
///                 - resourceKind: Pod
///                   resourceGroup: ""
///               jsonPath: .metadata[?(@.name == 'nginx')]
///             fieldActions:
///               - op: COPY
///                 path: /spec/initContainers/0/env
///                 fromPath: /spec/containers/0/env
/// ```
/// ### Gkebackup Restoreplan Gitops Mode
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("gitops-mode")
///             .build_struct(),
///     );
///     let gitopsMode = restore_plan::create(
///         "gitopsMode",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("gitops-mode")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .allNamespaces(true)
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .allGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("MERGE_SKIP_ON_CONFLICT")
///                     .volumeDataRestorePolicy("RESTORE_VOLUME_DATA_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("gitops-mode-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Restore Order
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("restore-order")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("restore-order-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let restoreOrder = restore_plan::create(
///         "restoreOrder",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("restore-order")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .allNamespaces(true)
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .allGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("FAIL_ON_CONFLICT")
///                     .restoreOrder(
///                         RestorePlanRestoreConfigRestoreOrder::builder()
///                             .groupKindDependencies(
///                                 vec![
///                                     RestorePlanRestoreConfigRestoreOrderGroupKindDependency::builder()
///                                     .requiring(RestorePlanRestoreConfigRestoreOrderGroupKindDependencyRequiring::builder()
///                                     .resourceGroup("stable.example.com").resourceKind("kindB")
///                                     .build_struct())
///                                     .satisfying(RestorePlanRestoreConfigRestoreOrderGroupKindDependencySatisfying::builder()
///                                     .resourceGroup("stable.example.com").resourceKind("kindA")
///                                     .build_struct()).build_struct(),
///                                     RestorePlanRestoreConfigRestoreOrderGroupKindDependency::builder()
///                                     .requiring(RestorePlanRestoreConfigRestoreOrderGroupKindDependencyRequiring::builder()
///                                     .resourceGroup("stable.example.com").resourceKind("kindC")
///                                     .build_struct())
///                                     .satisfying(RestorePlanRestoreConfigRestoreOrderGroupKindDependencySatisfying::builder()
///                                     .resourceGroup("stable.example.com").resourceKind("kindB")
///                                     .build_struct()).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .volumeDataRestorePolicy("RESTORE_VOLUME_DATA_FROM_BACKUP")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Gkebackup Restoreplan Volume Res
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = backup_plan::create(
///         "basic",
///         BackupPlanArgs::builder()
///             .backup_config(
///                 BackupPlanBackupConfig::builder()
///                     .allNamespaces(true)
///                     .includeSecrets(true)
///                     .includeVolumeData(true)
///                     .build_struct(),
///             )
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("volume-res")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .addons_config(
///                 ClusterAddonsConfig::builder()
///                     .gkeBackupAgentConfig(
///                         ClusterAddonsConfigGkeBackupAgentConfig::builder()
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .initial_node_count(1)
///             .location("us-central1")
///             .name("volume-res-cluster")
///             .network("default")
///             .subnetwork("default")
///             .workload_identity_config(
///                 ClusterWorkloadIdentityConfig::builder()
///                     .workloadPool("my-project-name.svc.id.goog")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let volumeRes = restore_plan::create(
///         "volumeRes",
///         RestorePlanArgs::builder()
///             .backup_plan("${basic.id}")
///             .cluster("${primary.id}")
///             .location("us-central1")
///             .name("volume-res")
///             .restore_config(
///                 RestorePlanRestoreConfig::builder()
///                     .allNamespaces(true)
///                     .clusterResourceConflictPolicy("USE_EXISTING_VERSION")
///                     .clusterResourceRestoreScope(
///                         RestorePlanRestoreConfigClusterResourceRestoreScope::builder()
///                             .allGroupKinds(true)
///                             .build_struct(),
///                     )
///                     .namespacedResourceRestoreMode("FAIL_ON_CONFLICT")
///                     .volumeDataRestorePolicy("NO_VOLUME_DATA_RESTORATION")
///                     .volumeDataRestorePolicyBindings(
///                         vec![
///                             RestorePlanRestoreConfigVolumeDataRestorePolicyBinding::builder()
///                             .policy("RESTORE_VOLUME_DATA_FROM_BACKUP")
///                             .volumeType("GCE_PERSISTENT_DISK").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RestorePlan can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/restorePlans/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, RestorePlan can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkebackup/restorePlan:RestorePlan default projects/{{project}}/locations/{{location}}/restorePlans/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkebackup/restorePlan:RestorePlan default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkebackup/restorePlan:RestorePlan default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod restore_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestorePlanArgs {
        /// A reference to the BackupPlan from which Backups may be used
        /// as the source for Restores created via this RestorePlan.
        #[builder(into)]
        pub backup_plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source cluster from which Restores will be created via this RestorePlan.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User specified descriptive string for this RestorePlan.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description: A set of custom labels supplied by the user. A list of key->value pairs. Example: { "name": "wrench",
        /// "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The region of the Restore Plan.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The full name of the BackupPlan Resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the configuration of Restores created via this RestorePlan.
        /// Structure is documented below.
        #[builder(into)]
        pub restore_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkebackup::RestorePlanRestoreConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct RestorePlanResult {
        /// A reference to the BackupPlan from which Backups may be used
        /// as the source for Restores created via this RestorePlan.
        pub backup_plan: pulumi_gestalt_rust::Output<String>,
        /// The source cluster from which Restores will be created via this RestorePlan.
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// User specified descriptive string for this RestorePlan.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Description: A set of custom labels supplied by the user. A list of key->value pairs. Example: { "name": "wrench",
        /// "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The region of the Restore Plan.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The full name of the BackupPlan Resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines the configuration of Restores created via this RestorePlan.
        /// Structure is documented below.
        pub restore_config: pulumi_gestalt_rust::Output<
            super::super::types::gkebackup::RestorePlanRestoreConfig,
        >,
        /// The State of the RestorePlan.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Detailed description of why RestorePlan is in its current state.
        pub state_reason: pulumi_gestalt_rust::Output<String>,
        /// Server generated, unique identifier of UUID format.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RestorePlanArgs,
    ) -> RestorePlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_plan_binding = args.backup_plan.get_output(context).get_inner();
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let restore_config_binding = args.restore_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkebackup/restorePlan:RestorePlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPlan".into(),
                    value: &backup_plan_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "restoreConfig".into(),
                    value: &restore_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RestorePlanResult {
            backup_plan: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupPlan"),
            ),
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            restore_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restoreConfig"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateReason"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
