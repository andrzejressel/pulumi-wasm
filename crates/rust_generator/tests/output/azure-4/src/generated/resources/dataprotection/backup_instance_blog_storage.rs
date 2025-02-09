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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_instance_blog_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstanceBlogStorageArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Backup Instance Blob Storage. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of the container names of the source Storage Account.
        ///
        /// > **Note:** The `storage_account_container_names` should be specified in the vaulted backup policy/operational and vaulted hybrid backup policy. Removing the `storage_account_container_names` will force a new resource to be created since it can't be removed once specified.
        #[builder(into, default)]
        pub storage_account_container_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The ID of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Backup Vault within which the Backup Instance Blob Storage should exist. Changing this forces a new Backup Instance Blob Storage to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstanceBlogStorageResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Backup Instance Blob Storage. Changing this forces a new Backup Instance Blob Storage to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The list of the container names of the source Storage Account.
        ///
        /// > **Note:** The `storage_account_container_names` should be specified in the vaulted backup policy/operational and vaulted hybrid backup policy. Removing the `storage_account_container_names` will force a new resource to be created since it can't be removed once specified.
        pub storage_account_container_names: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The ID of the source Storage Account. Changing this forces a new Backup Instance Blob Storage to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Backup Vault within which the Backup Instance Blob Storage should exist. Changing this forces a new Backup Instance Blob Storage to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupInstanceBlogStorageArgs,
    ) -> BackupInstanceBlogStorageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_policy_id_binding = args.backup_policy_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_container_names_binding = args
            .storage_account_container_names
            .get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let vault_id_binding = args.vault_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstanceBlogStorage:BackupInstanceBlogStorage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicyId".into(),
                    value: backup_policy_id_binding.get_id(),
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
                    name: "storageAccountContainerNames".into(),
                    value: storage_account_container_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultId".into(),
                    value: vault_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupInstanceBlogStorageResult {
            backup_policy_id: o.get_field("backupPolicyId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            storage_account_container_names: o.get_field("storageAccountContainerNames"),
            storage_account_id: o.get_field("storageAccountId"),
            vault_id: o.get_field("vaultId"),
        }
    }
}
