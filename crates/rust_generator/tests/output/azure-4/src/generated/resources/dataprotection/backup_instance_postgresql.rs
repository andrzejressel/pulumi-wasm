/// Manages a Backup Instance to back up PostgreSQL.
///
/// > **Note:** Before using this resource, there are some prerequisite permissions for configure backup and restore. See more details from <https://docs.microsoft.com/azure/backup/backup-azure-database-postgresql#prerequisite-permissions-for-configure-backup-and-restore>.
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
///   exampleServer:
///     type: azure:postgresql:Server
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: B_Gen5_2
///       storageMb: 5120
///       backupRetentionDays: 7
///       geoRedundantBackupEnabled: false
///       autoGrowEnabled: true
///       administratorLogin: psqladmin
///       administratorLoginPassword: H@Sh1CoR3!
///       version: '9.5'
///       sslEnforcementEnabled: true
///   exampleFirewallRule:
///     type: azure:postgresql:FirewallRule
///     name: example
///     properties:
///       name: AllowAllWindowsAzureIps
///       resourceGroupName: ${example.name}
///       serverName: ${exampleServer.name}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 0.0.0.0
///   exampleDatabase:
///     type: azure:postgresql:Database
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       serverName: ${exampleServer.name}
///       charset: UTF8
///       collation: English_United States.1252
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
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Create
///             - Get
///           secretPermissions:
///             - Set
///             - Get
///             - Delete
///             - Purge
///             - Recover
///         - tenantId: ${exampleBackupVault.identity.tenantId}
///           objectId: ${exampleBackupVault.identity.principalId}
///           keyPermissions:
///             - Create
///             - Get
///           secretPermissions:
///             - Set
///             - Get
///             - Delete
///             - Purge
///             - Recover
///   exampleSecret:
///     type: azure:keyvault:Secret
///     name: example
///     properties:
///       name: example
///       value: Server=${exampleServer.name}.postgres.database.azure.com;Database=${exampleDatabase.name};Port=5432;User Id=psqladmin@${exampleServer.name};Password=H@Sh1CoR3!;Ssl Mode=Require;
///       keyVaultId: ${exampleKeyVault.id}
///   exampleBackupPolicyPostgresql:
///     type: azure:dataprotection:BackupPolicyPostgresql
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       vaultName: ${exampleBackupVault.name}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-23T02:30:00+00:00/P1W
///       defaultRetentionDuration: P4M
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleServer.id}
///       roleDefinitionName: Reader
///       principalId: ${exampleBackupVault.identity.principalId}
///   exampleBackupInstancePostgresql:
///     type: azure:dataprotection:BackupInstancePostgresql
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       vaultId: ${exampleBackupVault.id}
///       databaseId: ${exampleDatabase.id}
///       backupPolicyId: ${exampleBackupPolicyPostgresql.id}
///       databaseCredentialKeyVaultSecretId: ${exampleSecret.versionlessId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Backup Instance PostgreSQL can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupInstancePostgresql:BackupInstancePostgresql example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupInstances/backupInstance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_instance_postgresql {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstancePostgresqlArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID or versionless ID of the key vault secret which stores the connection string of the database.
        #[builder(into, default)]
        pub database_credential_key_vault_secret_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the source database. Changing this forces a new Backup Instance PostgreSQL to be created.
        #[builder(into)]
        pub database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the source database. Changing this forces a new Backup Instance PostgreSQL to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Backup Instance PostgreSQL. Changing this forces a new Backup Instance PostgreSQL to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Backup Vault within which the PostgreSQL Backup Instance should exist. Changing this forces a new Backup Instance PostgreSQL to be created.
        #[builder(into)]
        pub vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstancePostgresqlResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID or versionless ID of the key vault secret which stores the connection string of the database.
        pub database_credential_key_vault_secret_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the source database. Changing this forces a new Backup Instance PostgreSQL to be created.
        pub database_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the source database. Changing this forces a new Backup Instance PostgreSQL to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Backup Instance PostgreSQL. Changing this forces a new Backup Instance PostgreSQL to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Backup Vault within which the PostgreSQL Backup Instance should exist. Changing this forces a new Backup Instance PostgreSQL to be created.
        pub vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupInstancePostgresqlArgs,
    ) -> BackupInstancePostgresqlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_policy_id_binding = args.backup_policy_id.get_output(context);
        let database_credential_key_vault_secret_id_binding = args
            .database_credential_key_vault_secret_id
            .get_output(context);
        let database_id_binding = args.database_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let vault_id_binding = args.vault_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstancePostgresql:BackupInstancePostgresql"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseCredentialKeyVaultSecretId".into(),
                    value: &database_credential_key_vault_secret_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseId".into(),
                    value: &database_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultId".into(),
                    value: &vault_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupInstancePostgresqlResult {
            backup_policy_id: o.get_field("backupPolicyId"),
            database_credential_key_vault_secret_id: o
                .get_field("databaseCredentialKeyVaultSecretId"),
            database_id: o.get_field("databaseId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            vault_id: o.get_field("vaultId"),
        }
    }
}
