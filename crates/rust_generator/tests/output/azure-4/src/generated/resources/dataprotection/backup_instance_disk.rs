/// Manages a Backup Instance to back up Disk.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: example
///     properties:
///       name: example-disk
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: '1'
///   exampleBackupVault:
///     type: azure:dataprotection:BackupVault
///     name: example
///     properties:
///       name: example-backup-vault
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datastoreType: VaultStore
///       redundancy: LocallyRedundant
///       identity:
///         type: SystemAssigned
///   example1:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${example.id}
///       roleDefinitionName: Disk Snapshot Contributor
///       principalId: ${exampleBackupVault.identity.principalId}
///   example2:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleManagedDisk.id}
///       roleDefinitionName: Disk Backup Reader
///       principalId: ${exampleBackupVault.identity.principalId}
///   exampleBackupPolicyDisk:
///     type: azure:dataprotection:BackupPolicyDisk
///     name: example
///     properties:
///       name: example-backup-policy
///       vaultId: ${exampleBackupVault.id}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-19T06:33:16+00:00/PT4H
///       defaultRetentionDuration: P7D
///   exampleBackupInstanceDisk:
///     type: azure:dataprotection:BackupInstanceDisk
///     name: example
///     properties:
///       name: example-backup-instance
///       location: ${exampleBackupVault.location}
///       vaultId: ${exampleBackupVault.id}
///       diskId: ${exampleManagedDisk.id}
///       snapshotResourceGroupName: ${example.name}
///       backupPolicyId: ${exampleBackupPolicyDisk.id}
/// ```
///
/// ## Import
///
/// Backup Instance Disks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupInstanceDisk:BackupInstanceDisk example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupInstances/backupInstance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_instance_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstanceDiskArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the source Disk. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub disk_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Backup Instance Disk. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub snapshot_resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Backup Vault within which the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstanceDiskResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the source Disk. Changing this forces a new Backup Instance Disk to be created.
        pub disk_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Backup Instance Disk. Changing this forces a new Backup Instance Disk to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new Backup Instance Disk to be created.
        pub snapshot_resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupInstanceDiskArgs,
    ) -> BackupInstanceDiskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_policy_id_binding = args.backup_policy_id.get_output(context);
        let disk_id_binding = args.disk_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let snapshot_resource_group_name_binding = args
            .snapshot_resource_group_name
            .get_output(context);
        let vault_id_binding = args.vault_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstanceDisk:BackupInstanceDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicyId".into(),
                    value: backup_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskId".into(),
                    value: disk_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotResourceGroupName".into(),
                    value: snapshot_resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultId".into(),
                    value: vault_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupInstanceDiskResult {
            backup_policy_id: o.get_field("backupPolicyId"),
            disk_id: o.get_field("diskId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            snapshot_resource_group_name: o.get_field("snapshotResourceGroupName"),
            vault_id: o.get_field("vaultId"),
        }
    }
}
