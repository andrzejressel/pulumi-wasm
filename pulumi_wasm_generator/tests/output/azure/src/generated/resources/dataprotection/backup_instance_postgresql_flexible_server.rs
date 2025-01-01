/// Manages a Backup Instance to back up PostgreSQL Flexible Server.
///
/// > **Note:** Before using this resource, there are some prerequisite permissions for configure backup and restore. See more details from <https://learn.microsoft.com/azure/backup/backup-azure-database-postgresql-flex-overview>.
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
///   exampleFlexibleServer:
///     type: azure:postgresql:FlexibleServer
///     name: example
///     properties:
///       name: example-postgresqlfs
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       administratorLogin: adminTerraform
///       administratorPassword: QAZwsx123
///       storageMb: 32768
///       version: '12'
///       skuName: GP_Standard_D4s_v3
///       zone: '2'
///   exampleBackupVault:
///     type: azure:dataprotection:BackupVault
///     name: example
///     properties:
///       name: example-backupvault
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datastoreType: VaultStore
///       redundancy: LocallyRedundant
///       softDelete: Off
///       identity:
///         type: SystemAssigned
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${example.id}
///       roleDefinitionName: Reader
///       principalId: ${exampleBackupVault.identity.principalId}
///   example2:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${exampleFlexibleServer.id}
///       roleDefinitionName: PostgreSQL Flexible Server Long Term Retention Backup Role
///       principalId: ${exampleBackupVault.identity.principalId}
///   exampleBackupPolicyPostgresqlFlexibleServer:
///     type: azure:dataprotection:BackupPolicyPostgresqlFlexibleServer
///     name: example
///     properties:
///       name: example-dp
///       vaultId: ${exampleBackupVault.id}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-23T02:30:00+00:00/P1W
///       defaultRetentionRule:
///         lifeCycles:
///           - duration: P4M
///             dataStoreType: VaultStore
///     options:
///       dependsOn:
///         - ${exampleAssignment}
///         - ${example2}
///   exampleBackupInstancePostgresqlFlexibleServer:
///     type: azure:dataprotection:BackupInstancePostgresqlFlexibleServer
///     name: example
///     properties:
///       name: example-dbi
///       location: ${example.location}
///       vaultId: ${exampleBackupVault.id}
///       serverId: ${exampleFlexibleServer.id}
///       backupPolicyId: ${exampleBackupPolicyPostgresqlFlexibleServer.id}
/// ```
///
/// ## Import
///
/// Backup Instance PostgreSQL Flexible Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupInstancePostgresqlFlexibleServer:BackupInstancePostgresqlFlexibleServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupInstances/backupInstance1
/// ```
///
pub mod backup_instance_postgresql_flexible_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupInstancePostgresqlFlexibleServerArgs {
        /// The ID of the Backup Policy.
        #[builder(into)]
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The location of the source database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Backup Instance for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the source server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the PostgreSQL Flexible Server Backup Instance should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupInstancePostgresqlFlexibleServerResult {
        /// The ID of the Backup Policy.
        pub backup_policy_id: pulumi_wasm_rust::Output<String>,
        /// The location of the source database. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Backup Instance for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the source server. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Backup Vault within which the PostgreSQL Flexible Server Backup Instance should exist. Changing this forces a new resource to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BackupInstancePostgresqlFlexibleServerArgs,
    ) -> BackupInstancePostgresqlFlexibleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args.backup_policy_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let vault_id_binding = args.vault_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupInstancePostgresqlFlexibleServer:BackupInstancePostgresqlFlexibleServer"
                .into(),
            name: name.to_string(),
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
                    name: "serverId".into(),
                    value: &server_id_binding,
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
                    name: "serverId".into(),
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
        BackupInstancePostgresqlFlexibleServerResult {
            backup_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPolicyId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultId").unwrap(),
            ),
        }
    }
}
