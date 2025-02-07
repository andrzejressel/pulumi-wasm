/// Manages a Backup Instance to back up a Kubernetes Cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   snap:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-snap
///       location: West Europe
///   exampleBackupVault:
///     type: azure:dataprotection:BackupVault
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datastoreType: VaultStore
///       redundancy: LocallyRedundant
///       identity:
///         type: SystemAssigned
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: dns
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///         hostEncryptionEnabled: true
///       identity:
///         type: SystemAssigned
///   aksClusterTrustedAccess:
///     type: azure:containerservice:ClusterTrustedAccessRoleBinding
///     name: aks_cluster_trusted_access
///     properties:
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       name: example
///       roles:
///         - Microsoft.DataProtection/backupVaults/backup-operator
///       sourceResourceId: ${exampleBackupVault.id}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleKubernetesClusterExtension:
///     type: azure:containerservice:KubernetesClusterExtension
///     name: example
///     properties:
///       name: example
///       clusterId: ${exampleKubernetesCluster.id}
///       extensionType: Microsoft.DataProtection.Kubernetes
///       releaseTrain: stable
///       releaseNamespace: dataprotection-microsoft
///       configurationSettings:
///         configuration.backupStorageLocation.bucket: ${exampleContainer.name}
///         configuration.backupStorageLocation.config.resourceGroup: ${example.name}
///         configuration.backupStorageLocation.config.storageAccount: ${exampleAccount.name}
///         configuration.backupStorageLocation.config.subscriptionId: ${current.subscriptionId}
///         credentials.tenantId: ${current.tenantId}
///   testExtensionAndStorageAccountPermission:
///     type: azure:authorization:Assignment
///     name: test_extension_and_storage_account_permission
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Account Contributor
///       principalId: ${exampleKubernetesClusterExtension.aksAssignedIdentities[0].principalId}
///   testVaultMsiReadOnCluster:
///     type: azure:authorization:Assignment
///     name: test_vault_msi_read_on_cluster
///     properties:
///       scope: ${exampleKubernetesCluster.id}
///       roleDefinitionName: Reader
///       principalId: ${exampleBackupVault.identity.principalId}
///   testVaultMsiReadOnSnapRg:
///     type: azure:authorization:Assignment
///     name: test_vault_msi_read_on_snap_rg
///     properties:
///       scope: ${snap.id}
///       roleDefinitionName: Reader
///       principalId: ${exampleBackupVault.identity.principalId}
///   testVaultMsiSnapshotContributorOnSnapRg:
///     type: azure:authorization:Assignment
///     name: test_vault_msi_snapshot_contributor_on_snap_rg
///     properties:
///       scope: ${snap.id}
///       roleDefinitionName: Disk Snapshot Contributor
///       principalId: ${exampleBackupVault.identity.principalId}
///   testVaultDataOperatorOnSnapRg:
///     type: azure:authorization:Assignment
///     name: test_vault_data_operator_on_snap_rg
///     properties:
///       scope: ${snap.id}
///       roleDefinitionName: Data Operator for Managed Disks
///       principalId: ${exampleBackupVault.identity.principalId}
///   testVaultDataContributorOnStorage:
///     type: azure:authorization:Assignment
///     name: test_vault_data_contributor_on_storage
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Blob Data Contributor
///       principalId: ${exampleBackupVault.identity.principalId}
///   testClusterMsiContributorOnSnapRg:
///     type: azure:authorization:Assignment
///     name: test_cluster_msi_contributor_on_snap_rg
///     properties:
///       scope: ${snap.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleKubernetesCluster.identity.principalId}
///   exampleBackupPolicyKubernetesCluster:
///     type: azure:dataprotection:BackupPolicyKubernetesCluster
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       vaultName: ${exampleBackupVault.name}
///       backupRepeatingTimeIntervals:
///         - R/2023-05-23T02:30:00+00:00/P1W
///       retentionRules:
///         - name: Daily
///           priority: 25
///           lifeCycles:
///             - duration: P84D
///               dataStoreType: OperationalStore
///           criteria:
///             daysOfWeeks:
///               - Thursday
///             monthsOfYears:
///               - November
///             weeksOfMonths:
///               - First
///             scheduledBackupTimes:
///               - 2023-05-23T02:30:00Z
///       defaultRetentionRule:
///         lifeCycles:
///           - duration: P14D
///             dataStoreType: OperationalStore
///   exampleBackupInstanceKubernetesCluster:
///     type: azure:dataprotection:BackupInstanceKubernetesCluster
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       vaultId: ${exampleBackupVault.id}
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       snapshotResourceGroupName: ${snap.name}
///       backupPolicyId: ${exampleBackupPolicyKubernetesCluster.id}
///       backupDatasourceParameters:
///         excludedNamespaces:
///           - test-excluded-namespaces
///         excludedResourceTypes:
///           - exvolumesnapshotcontents.snapshot.storage.k8s.io
///         clusterScopedResourcesEnabled: true
///         includedNamespaces:
///           - test-included-namespaces
///         includedResourceTypes:
///           - involumesnapshotcontents.snapshot.storage.k8s.io
///         labelSelectors:
///           - kubernetes.io/metadata.name:test
///         volumeSnapshotEnabled: true
///     options:
///       dependsOn:
///         - ${testExtensionAndStorageAccountPermission}
///         - ${testVaultMsiReadOnCluster}
///         - ${testVaultMsiReadOnSnapRg}
///         - ${testClusterMsiContributorOnSnapRg}
///         - ${testVaultMsiSnapshotContributorOnSnapRg}
///         - ${testVaultDataOperatorOnSnapRg}
///         - ${testVaultDataContributorOnStorage}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Backup Instance Kubernetes Cluster can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupInstanceKubernetesCluster:BackupInstanceKubernetesCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupInstances/backupInstance1
/// ```
///
pub mod backup_instance_kubernetes_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstanceKubernetesClusterArgs {
        /// A `backup_datasource_parameters` block as defined below.
        #[builder(into, default)]
        pub backup_datasource_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataprotection::BackupInstanceKubernetesClusterBackupDatasourceParameters,
            >,
        >,
        /// The ID of the Backup Policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Kubernetes Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the Backup Instance Kubernetes Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Backup Instance Kubernetes Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new resource to be created.
        #[builder(into)]
        pub snapshot_resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Backup Vault within which the Backup Instance Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstanceKubernetesClusterResult {
        /// A `backup_datasource_parameters` block as defined below.
        pub backup_datasource_parameters: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataprotection::BackupInstanceKubernetesClusterBackupDatasourceParameters,
            >,
        >,
        /// The ID of the Backup Policy. Changing this forces a new resource to be created.
        pub backup_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Kubernetes Cluster. Changing this forces a new resource to be created.
        pub kubernetes_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the Backup Instance Kubernetes Cluster. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Backup Instance Kubernetes Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new resource to be created.
        pub snapshot_resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackupInstanceKubernetesClusterArgs,
    ) -> BackupInstanceKubernetesClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_datasource_parameters_binding = args
            .backup_datasource_parameters
            .get_output(context)
            .get_inner();
        let backup_policy_id_binding = args
            .backup_policy_id
            .get_output(context)
            .get_inner();
        let kubernetes_cluster_id_binding = args
            .kubernetes_cluster_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let snapshot_resource_group_name_binding = args
            .snapshot_resource_group_name
            .get_output(context)
            .get_inner();
        let vault_id_binding = args.vault_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstanceKubernetesCluster:BackupInstanceKubernetesCluster"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupDatasourceParameters".into(),
                    value: &backup_datasource_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding,
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
                    name: "snapshotResourceGroupName".into(),
                    value: &snapshot_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "vaultId".into(),
                    value: &vault_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupInstanceKubernetesClusterResult {
            backup_datasource_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupDatasourceParameters"),
            ),
            backup_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupPolicyId"),
            ),
            kubernetes_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubernetesClusterId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            snapshot_resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotResourceGroupName"),
            ),
            vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vaultId"),
            ),
        }
    }
}
