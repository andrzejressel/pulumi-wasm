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
pub mod backup_instance_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstanceDiskArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the source Disk. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Backup Instance Disk. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub snapshot_resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstanceDiskResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the source Disk. Changing this forces a new Backup Instance Disk to be created.
        pub disk_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Backup Instance Disk. Changing this forces a new Backup Instance Disk to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where snapshots are stored. Changing this forces a new Backup Instance Disk to be created.
        pub snapshot_resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Disk should exist. Changing this forces a new Backup Instance Disk to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupInstanceDiskArgs) -> BackupInstanceDiskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args.backup_policy_id.get_inner();
        let disk_id_binding = args.disk_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let snapshot_resource_group_name_binding = args
            .snapshot_resource_group_name
            .get_inner();
        let vault_id_binding = args.vault_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstanceDisk:BackupInstanceDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "diskId".into(),
                    value: &disk_id_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "diskId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "snapshotResourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "vaultId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupInstanceDiskResult {
            backup_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPolicyId").unwrap(),
            ),
            disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            snapshot_resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotResourceGroupName").unwrap(),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultId").unwrap(),
            ),
        }
    }
}
