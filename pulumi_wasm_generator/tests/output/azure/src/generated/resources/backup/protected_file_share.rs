/// Manages an Azure Backup Protected File Share to enable backups for file shares within an Azure Storage Account
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-recovery_vault
///       location: West Europe
///   vault:
///     type: azure:recoveryservices:Vault
///     properties:
///       name: tfex-recovery-vault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   sa:
///     type: azure:storage:Account
///     properties:
///       name: examplesa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleShare:
///     type: azure:storage:Share
///     name: example
///     properties:
///       name: example-share
///       storageAccountName: ${sa.name}
///       quota: 1
///   protection-container:
///     type: azure:backup:ContainerStorageAccount
///     properties:
///       resourceGroupName: ${example.name}
///       recoveryVaultName: ${vault.name}
///       storageAccountId: ${sa.id}
///   examplePolicyFileShare:
///     type: azure:backup:PolicyFileShare
///     name: example
///     properties:
///       name: tfex-recovery-vault-policy
///       resourceGroupName: ${example.name}
///       recoveryVaultName: ${vault.name}
///       backup:
///         frequency: Daily
///         time: 23:00
///       retentionDaily:
///         count: 10
///   share1:
///     type: azure:backup:ProtectedFileShare
///     properties:
///       resourceGroupName: ${example.name}
///       recoveryVaultName: ${vault.name}
///       sourceStorageAccountId: ${["protection-container"].storageAccountId}
///       sourceFileShareName: ${exampleShare.name}
///       backupPolicyId: ${examplePolicyFileShare.id}
/// ```
///
/// ## Import
///
/// Azure Backup Protected File Shares can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:backup/protectedFileShare:ProtectedFileShare item1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/example-recovery-vault/backupFabrics/Azure/protectionContainers/StorageContainer;storage;group2;example-storage-account/protectedItems/AzureFileShare;3f6e3108a45793581bcbd1c61c87a3b2ceeb4ff4bc02a95ce9d1022b23722935"
/// ```
///
/// -> **NOTE** The ID requires quoting as there are semicolons. This user unfriendly ID can be found in the Deployments of the used resourcegroup, look for an Deployment which starts with `ConfigureAFSProtection-`, click then `Go to resource`.
///
pub mod protected_file_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectedFileShareArgs {
        /// Specifies the ID of the backup policy to use. The policy must be an Azure File Share backup policy. Other types are not supported.
        #[builder(into)]
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Azure Backup Protected File Share. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the file share to backup. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_file_share_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage account of the file share to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** The storage account must already be registered with the recovery vault in order to backup shares within the account. You can use the `azure.backup.ContainerStorageAccount` resource or the [Register-AzRecoveryServicesBackupContainer PowerShell cmdlet](https://docs.microsoft.com/powershell/module/az.recoveryservices/register-azrecoveryservicesbackupcontainer?view=azps-3.2.0) to register a storage account with a vault. When using the `azure.backup.ContainerStorageAccount` resource to register, you can use `depends_on` to explicitly declare the dependency. It will make sure that the registration is completed before creating the `azure.backup.ProtectedFileShare` resource.
        #[builder(into)]
        pub source_storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectedFileShareResult {
        /// Specifies the ID of the backup policy to use. The policy must be an Azure File Share backup policy. Other types are not supported.
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Azure Backup Protected File Share. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the file share to backup. Changing this forces a new resource to be created.
        pub source_file_share_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage account of the file share to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** The storage account must already be registered with the recovery vault in order to backup shares within the account. You can use the `azure.backup.ContainerStorageAccount` resource or the [Register-AzRecoveryServicesBackupContainer PowerShell cmdlet](https://docs.microsoft.com/powershell/module/az.recoveryservices/register-azrecoveryservicesbackupcontainer?view=azps-3.2.0) to register a storage account with a vault. When using the `azure.backup.ContainerStorageAccount` resource to register, you can use `depends_on` to explicitly declare the dependency. It will make sure that the registration is completed before creating the `azure.backup.ProtectedFileShare` resource.
        pub source_storage_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProtectedFileShareArgs) -> ProtectedFileShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args.backup_policy_id.get_inner();
        let recovery_vault_name_binding = args.recovery_vault_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let source_file_share_name_binding = args.source_file_share_name.get_inner();
        let source_storage_account_id_binding = args
            .source_storage_account_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:backup/protectedFileShare:ProtectedFileShare".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceFileShareName".into(),
                    value: &source_file_share_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceStorageAccountId".into(),
                    value: &source_storage_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceFileShareName".into(),
                },
                register_interface::ResultField {
                    name: "sourceStorageAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProtectedFileShareResult {
            backup_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPolicyId").unwrap(),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_file_share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceFileShareName").unwrap(),
            ),
            source_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceStorageAccountId").unwrap(),
            ),
        }
    }
}