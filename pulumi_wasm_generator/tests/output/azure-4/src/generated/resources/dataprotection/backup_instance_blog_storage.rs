/// Manages a Backup Instance Blob Storage.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
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
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleAccount.id}
///       roleDefinitionName: Storage Account Backup Contributor
///       principalId: ${exampleBackupVault.identity.principalId}
///   exampleBackupPolicyBlobStorage:
///     type: azure:dataprotection:BackupPolicyBlobStorage
///     name: example
///     properties:
///       name: example-backup-policy
///       vaultId: ${exampleBackupVault.id}
///       operationalDefaultRetentionDuration: P30D
///   exampleBackupInstanceBlogStorage:
///     type: azure:dataprotection:BackupInstanceBlogStorage
///     name: example
///     properties:
///       name: example-backup-instance
///       vaultId: ${exampleBackupVault.id}
///       location: ${example.location}
///       storageAccountId: ${exampleAccount.id}
///       backupPolicyId: ${exampleBackupPolicyBlobStorage.id}
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// ```
///
/// ## Import
///
/// Backup Instance Blob Storages can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupInstanceBlogStorage:BackupInstanceBlogStorage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupInstances/backupInstance1
/// ```
///
pub mod backup_instance_blog_storage {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstanceBlogStorageArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The location of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Backup Instance Blob Storage. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of the container names of the source Storage Account.
        ///
        /// > **Note:** The `storage_account_container_names` should be specified in the vaulted backup policy/operational and vaulted hybrid backup policy. Removing the `storage_account_container_names` will force a new resource to be created since it can't be removed once specified.
        #[builder(into, default)]
        pub storage_account_container_names: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Blob Storage should exist. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstanceBlogStorageResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The location of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Backup Instance Blob Storage. Changing this forces a new Backup Instance Blob Storage to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The list of the container names of the source Storage Account.
        ///
        /// > **Note:** The `storage_account_container_names` should be specified in the vaulted backup policy/operational and vaulted hybrid backup policy. Removing the `storage_account_container_names` will force a new resource to be created since it can't be removed once specified.
        pub storage_account_container_names: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Blob Storage should exist. Changing this forces a new Backup Instance Blob Storage to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BackupInstanceBlogStorageArgs,
    ) -> BackupInstanceBlogStorageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args.backup_policy_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_container_names_binding = args
            .storage_account_container_names
            .get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let vault_id_binding = args.vault_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstanceBlogStorage:BackupInstanceBlogStorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding,
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
                    name: "storageAccountContainerNames".into(),
                    value: &storage_account_container_names_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountContainerNames".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
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
        BackupInstanceBlogStorageResult {
            backup_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPolicyId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_container_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountContainerNames").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultId").unwrap(),
            ),
        }
    }
}
