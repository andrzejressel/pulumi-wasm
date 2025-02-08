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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protected_file_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectedFileShareArgs {
        /// Specifies the ID of the backup policy to use. The policy must be an Azure File Share backup policy. Other types are not supported.
        #[builder(into)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the Azure Backup Protected File Share. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the file share to backup. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_file_share_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the storage account of the file share to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** The storage account must already be registered with the recovery vault in order to backup shares within the account. You can use the `azure.backup.ContainerStorageAccount` resource or the [Register-AzRecoveryServicesBackupContainer PowerShell cmdlet](https://docs.microsoft.com/powershell/module/az.recoveryservices/register-azrecoveryservicesbackupcontainer?view=azps-3.2.0) to register a storage account with a vault. When using the `azure.backup.ContainerStorageAccount` resource to register, you can use `depends_on` to explicitly declare the dependency. It will make sure that the registration is completed before creating the `azure.backup.ProtectedFileShare` resource.
        #[builder(into)]
        pub source_storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectedFileShareResult {
        /// Specifies the ID of the backup policy to use. The policy must be an Azure File Share backup policy. Other types are not supported.
        pub backup_policy_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Azure Backup Protected File Share. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the file share to backup. Changing this forces a new resource to be created.
        pub source_file_share_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the storage account of the file share to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** The storage account must already be registered with the recovery vault in order to backup shares within the account. You can use the `azure.backup.ContainerStorageAccount` resource or the [Register-AzRecoveryServicesBackupContainer PowerShell cmdlet](https://docs.microsoft.com/powershell/module/az.recoveryservices/register-azrecoveryservicesbackupcontainer?view=azps-3.2.0) to register a storage account with a vault. When using the `azure.backup.ContainerStorageAccount` resource to register, you can use `depends_on` to explicitly declare the dependency. It will make sure that the registration is completed before creating the `azure.backup.ProtectedFileShare` resource.
        pub source_storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProtectedFileShareArgs,
    ) -> ProtectedFileShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args
            .backup_policy_id
            .get_output(context)
            .get_inner();
        let recovery_vault_name_binding = args
            .recovery_vault_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let source_file_share_name_binding = args
            .source_file_share_name
            .get_output(context)
            .get_inner();
        let source_storage_account_id_binding = args
            .source_storage_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:backup/protectedFileShare:ProtectedFileShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProtectedFileShareResult {
            backup_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupPolicyId"),
            ),
            recovery_vault_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            source_file_share_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceFileShareName"),
            ),
            source_storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceStorageAccountId"),
            ),
        }
    }
}
