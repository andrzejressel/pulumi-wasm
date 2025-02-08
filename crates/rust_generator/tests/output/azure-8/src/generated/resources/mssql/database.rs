/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: example-sqlserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: 4dm1n157r470r
///       administratorLoginPassword: 4-v3ry-53cr37-p455w0rd
///   exampleDatabase:
///     type: azure:mssql:Database
///     name: example
///     properties:
///       name: example-db
///       serverId: ${exampleServer.id}
///       collation: SQL_Latin1_General_CP1_CI_AS
///       licenseType: LicenseIncluded
///       maxSizeGb: 2
///       skuName: S0
///       enclaveType: VBS
///       tags:
///         foo: bar
/// ```
///
///
/// ### Transparent Data Encryption(TDE) With A Customer Managed Key(CMK) During Create
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-admin
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleServer:
///     type: azure:mssql:Server
///     name: example
///     properties:
///       name: example-sqlserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: 4dm1n157r470r
///       administratorLoginPassword: 4-v3ry-53cr37-p455w0rd
///   exampleDatabase:
///     type: azure:mssql:Database
///     name: example
///     properties:
///       name: example-db
///       serverId: ${exampleServer.id}
///       collation: SQL_Latin1_General_CP1_CI_AS
///       licenseType: LicenseIncluded
///       maxSizeGb: 4
///       readScale: true
///       skuName: S0
///       zoneRedundant: true
///       enclaveType: VBS
///       tags:
///         foo: bar
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       transparentDataEncryptionKeyVaultKeyId: ${exampleKey.id}
///   # Create a key vault with access policies which allow for the current user to get, list, create, delete, update, recover, purge and getRotationPolicy for the key vault key and also add a key vault access policy for the Microsoft Sql Server instance User Managed Identity to get, wrap, and unwrap key(s)
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: mssqltdeexample
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       enabledForDiskEncryption: true
///       tenantId: ${exampleUserAssignedIdentity.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: true
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Get
///             - List
///             - Create
///             - Delete
///             - Update
///             - Recover
///             - Purge
///             - GetRotationPolicy
///         - tenantId: ${exampleUserAssignedIdentity.tenantId}
///           objectId: ${exampleUserAssignedIdentity.principalId}
///           keyPermissions:
///             - Get
///             - WrapKey
///             - UnwrapKey
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-key
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - unwrapKey
///         - wrapKey
///     options:
///       dependsOn:
///         - ${exampleKeyVault}
/// ```
///
/// ## Import
///
/// SQL Database can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/database:Database example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Sql/servers/server1/databases/example1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// Time in minutes after which database is automatically paused. A value of `-1` means that automatic pause is disabled. This property is only settable for Serverless databases.
        #[builder(into, default)]
        pub auto_pause_delay_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the collation of the database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub collation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The create mode of the database. Possible values are `Copy`, `Default`, `OnlineSecondary`, `PointInTimeRestore`, `Recovery`, `Restore`, `RestoreExternalBackup`, `RestoreExternalBackupSecondary`, `RestoreLongTermRetentionBackup` and `Secondary`. Mutually exclusive with `import`. Changing this forces a new resource to be created. Defaults to `Default`.
        #[builder(into, default)]
        pub create_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the source database from which to create the new database. This should only be used for databases with `create_mode` values that use another database as reference. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When configuring a secondary database, please be aware of the constraints for the `sku_name` property, as noted below, for both the primary and secondary databases. The `sku_name` of the secondary database may be inadvertently changed to match that of the primary when an incompatible combination of SKUs is detected by the provider.
        #[builder(into, default)]
        pub creation_source_database_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the ID of the elastic pool containing this database.
        #[builder(into, default)]
        pub elastic_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the database. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** `enclave_type` is currently not supported for DW (e.g, DataWarehouse) and DC-series SKUs.
        ///
        /// > **NOTE:** Geo Replicated and Failover databases must have the same `enclave_type`.
        ///
        /// > **NOTE:** The default value for the `enclave_type` field is unset not `Default`.
        #[builder(into, default)]
        pub enclave_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean that specifies if the Geo Backup Policy is enabled. Defaults to `true`.
        ///
        /// > **NOTE:** `geo_backup_enabled` is only applicable for DataWarehouse SKUs (DW*). This setting is ignored for all other SKUs.
        #[builder(into, default)]
        pub geo_backup_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::DatabaseIdentity>,
        >,
        /// A `import` block as documented below. Mutually exclusive with `create_mode`.
        #[builder(into, default)]
        pub import: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::DatabaseImport>,
        >,
        /// A boolean that specifies if this is a ledger database. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ledger_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `long_term_retention_policy` block as defined below.
        #[builder(into, default)]
        pub long_term_retention_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::DatabaseLongTermRetentionPolicy>,
        >,
        /// The name of the Public Maintenance Configuration window to apply to the database. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`. Defaults to `SQL_Default`.
        ///
        /// > **NOTE:** `maintenance_configuration_name` is only applicable if `elastic_pool_id` is not set.
        #[builder(into, default)]
        pub maintenance_configuration_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The max size of the database in gigabytes.
        ///
        /// > **NOTE:** This value should not be configured when the `create_mode` is `Secondary` or `OnlineSecondary`, as the sizing of the primary is then used as per [Azure documentation](https://docs.microsoft.com/azure/azure-sql/database/single-database-scale#geo-replicated-database).
        #[builder(into, default)]
        pub max_size_gb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimal capacity that database will always have allocated, if not paused. This property is only settable for Serverless databases.
        #[builder(into, default)]
        pub min_capacity: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The name of the MS SQL Database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of readonly secondary replicas associated with the database to which readonly application intent connections may be routed. This property is only settable for Hyperscale edition databases.
        #[builder(into, default)]
        pub read_replica_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica. This property is only settable for Premium and Business Critical databases.
        #[builder(into, default)]
        pub read_scale: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the database to be recovered. This property is only applicable when the `create_mode` is `Recovery`.
        #[builder(into, default)]
        pub recover_database_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Recovery Services Recovery Point Id to be restored. This property is only applicable when the `create_mode` is `Recovery`.
        #[builder(into, default)]
        pub recovery_point_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the database to be restored. This property is only applicable when the `create_mode` is `Restore`.
        #[builder(into, default)]
        pub restore_dropped_database_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the long term retention backup to be restored. This property is only applicable when the `create_mode` is `RestoreLongTermRetentionBackup`.
        #[builder(into, default)]
        pub restore_long_term_retention_backup_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database. This property is only settable for `create_mode`= `PointInTimeRestore` databases.
        #[builder(into, default)]
        pub restore_point_in_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the sample schema to apply when creating this database. Possible value is `AdventureWorksLT`.
        #[builder(into, default)]
        pub sample_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How do you want your replica to be made? Valid values include `Geo` and `Named`. Defaults to `Geo`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub secondary_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the MS SQL Server on which to create the database. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This setting is still required for "Serverless" SKUs
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `short_term_retention_policy` block as defined below.
        #[builder(into, default)]
        pub short_term_retention_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::DatabaseShortTermRetentionPolicy>,
        >,
        /// Specifies the name of the SKU used by the database. For example, `GP_S_Gen5_2`,`HS_Gen4_1`,`BC_Gen5_2`, `ElasticPool`, `Basic`,`S0`, `P2` ,`DW100c`, `DS100`. Changing this from the HyperScale service tier to another service tier will create a new resource.
        ///
        /// > **NOTE:** The default `sku_name` value may differ between Azure locations depending on local availability of Gen4/Gen5 capacity. When databases are replicated using the `creation_source_database_id` property, the source (primary) database cannot have a higher SKU service tier than any secondary databases. When changing the `sku_name` of a database having one or more secondary databases, this resource will first update any secondary databases as necessary. In such cases it's recommended to use the same `sku_name` in your configuration for all related databases, as not doing so may cause an unresolvable diff during subsequent plans.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the storage account type used to store backups for this database. Possible values are `Geo`, `GeoZone`, `Local` and `Zone`. Defaults to `Geo`.
        #[builder(into, default)]
        pub storage_account_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Threat detection policy configuration. The `threat_detection_policy` block supports fields documented below.
        #[builder(into, default)]
        pub threat_detection_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mssql::DatabaseThreatDetectionPolicy>,
        >,
        /// If set to true, Transparent Data Encryption will be enabled on the database. Defaults to `true`.
        ///
        /// > **NOTE:** `transparent_data_encryption_enabled` can only be set to `false` on DW (e.g, DataWarehouse) server SKUs.
        #[builder(into, default)]
        pub transparent_data_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean flag to specify whether TDE automatically rotates the encryption Key to latest version or not. Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** When the `sku_name` is `DW100c`, the `transparent_data_encryption_key_automatic_rotation_enabled` and the `transparent_data_encryption_key_vault_key_id` properties should not be specified, as database-level CMK is not supported for Data Warehouse SKUs.
        #[builder(into, default)]
        pub transparent_data_encryption_key_automatic_rotation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The fully versioned `Key Vault` `Key` URL (e.g. `'https://<YourVaultName>.vault.azure.net/keys/<YourKeyName>/<YourKeyVersion>`) to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        ///
        /// > **NOTE:** To successfully deploy a `Microsoft SQL Database` in CMK/BYOK TDE the `Key Vault` must have `Soft-delete` and `purge protection` enabled to protect from data loss due to accidental key and/or key vault deletion. The `Key Vault` and the `Microsoft SQL Server` `User Managed Identity Instance` must belong to the same `Azure Active Directory` `tenant`.
        #[builder(into, default)]
        pub transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones. This property is only settable for Premium and Business Critical databases.
        #[builder(into, default)]
        pub zone_redundant: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// Time in minutes after which database is automatically paused. A value of `-1` means that automatic pause is disabled. This property is only settable for Serverless databases.
        pub auto_pause_delay_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the collation of the database. Changing this forces a new resource to be created.
        pub collation: pulumi_gestalt_rust::Output<String>,
        /// The create mode of the database. Possible values are `Copy`, `Default`, `OnlineSecondary`, `PointInTimeRestore`, `Recovery`, `Restore`, `RestoreExternalBackup`, `RestoreExternalBackupSecondary`, `RestoreLongTermRetentionBackup` and `Secondary`. Mutually exclusive with `import`. Changing this forces a new resource to be created. Defaults to `Default`.
        pub create_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the source database from which to create the new database. This should only be used for databases with `create_mode` values that use another database as reference. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When configuring a secondary database, please be aware of the constraints for the `sku_name` property, as noted below, for both the primary and secondary databases. The `sku_name` of the secondary database may be inadvertently changed to match that of the primary when an incompatible combination of SKUs is detected by the provider.
        pub creation_source_database_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the elastic pool containing this database.
        pub elastic_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the database. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** `enclave_type` is currently not supported for DW (e.g, DataWarehouse) and DC-series SKUs.
        ///
        /// > **NOTE:** Geo Replicated and Failover databases must have the same `enclave_type`.
        ///
        /// > **NOTE:** The default value for the `enclave_type` field is unset not `Default`.
        pub enclave_type: pulumi_gestalt_rust::Output<String>,
        /// A boolean that specifies if the Geo Backup Policy is enabled. Defaults to `true`.
        ///
        /// > **NOTE:** `geo_backup_enabled` is only applicable for DataWarehouse SKUs (DW*). This setting is ignored for all other SKUs.
        pub geo_backup_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::DatabaseIdentity>,
        >,
        /// A `import` block as documented below. Mutually exclusive with `create_mode`.
        pub import: pulumi_gestalt_rust::Output<
            Option<super::super::types::mssql::DatabaseImport>,
        >,
        /// A boolean that specifies if this is a ledger database. Defaults to `false`. Changing this forces a new resource to be created.
        pub ledger_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// A `long_term_retention_policy` block as defined below.
        pub long_term_retention_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::DatabaseLongTermRetentionPolicy,
        >,
        /// The name of the Public Maintenance Configuration window to apply to the database. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`. Defaults to `SQL_Default`.
        ///
        /// > **NOTE:** `maintenance_configuration_name` is only applicable if `elastic_pool_id` is not set.
        pub maintenance_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The max size of the database in gigabytes.
        ///
        /// > **NOTE:** This value should not be configured when the `create_mode` is `Secondary` or `OnlineSecondary`, as the sizing of the primary is then used as per [Azure documentation](https://docs.microsoft.com/azure/azure-sql/database/single-database-scale#geo-replicated-database).
        pub max_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// Minimal capacity that database will always have allocated, if not paused. This property is only settable for Serverless databases.
        pub min_capacity: pulumi_gestalt_rust::Output<f64>,
        /// The name of the MS SQL Database. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of readonly secondary replicas associated with the database to which readonly application intent connections may be routed. This property is only settable for Hyperscale edition databases.
        pub read_replica_count: pulumi_gestalt_rust::Output<i32>,
        /// If enabled, connections that have application intent set to readonly in their connection string may be routed to a readonly secondary replica. This property is only settable for Premium and Business Critical databases.
        pub read_scale: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the database to be recovered. This property is only applicable when the `create_mode` is `Recovery`.
        pub recover_database_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Recovery Services Recovery Point Id to be restored. This property is only applicable when the `create_mode` is `Recovery`.
        pub recovery_point_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the database to be restored. This property is only applicable when the `create_mode` is `Restore`.
        pub restore_dropped_database_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the long term retention backup to be restored. This property is only applicable when the `create_mode` is `RestoreLongTermRetentionBackup`.
        pub restore_long_term_retention_backup_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database. This property is only settable for `create_mode`= `PointInTimeRestore` databases.
        pub restore_point_in_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the sample schema to apply when creating this database. Possible value is `AdventureWorksLT`.
        pub sample_name: pulumi_gestalt_rust::Output<String>,
        /// How do you want your replica to be made? Valid values include `Geo` and `Named`. Defaults to `Geo`. Changing this forces a new resource to be created.
        pub secondary_type: pulumi_gestalt_rust::Output<String>,
        /// The id of the MS SQL Server on which to create the database. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This setting is still required for "Serverless" SKUs
        pub server_id: pulumi_gestalt_rust::Output<String>,
        /// A `short_term_retention_policy` block as defined below.
        pub short_term_retention_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::DatabaseShortTermRetentionPolicy,
        >,
        /// Specifies the name of the SKU used by the database. For example, `GP_S_Gen5_2`,`HS_Gen4_1`,`BC_Gen5_2`, `ElasticPool`, `Basic`,`S0`, `P2` ,`DW100c`, `DS100`. Changing this from the HyperScale service tier to another service tier will create a new resource.
        ///
        /// > **NOTE:** The default `sku_name` value may differ between Azure locations depending on local availability of Gen4/Gen5 capacity. When databases are replicated using the `creation_source_database_id` property, the source (primary) database cannot have a higher SKU service tier than any secondary databases. When changing the `sku_name` of a database having one or more secondary databases, this resource will first update any secondary databases as necessary. In such cases it's recommended to use the same `sku_name` in your configuration for all related databases, as not doing so may cause an unresolvable diff during subsequent plans.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage account type used to store backups for this database. Possible values are `Geo`, `GeoZone`, `Local` and `Zone`. Defaults to `Geo`.
        pub storage_account_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Threat detection policy configuration. The `threat_detection_policy` block supports fields documented below.
        pub threat_detection_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::DatabaseThreatDetectionPolicy,
        >,
        /// If set to true, Transparent Data Encryption will be enabled on the database. Defaults to `true`.
        ///
        /// > **NOTE:** `transparent_data_encryption_enabled` can only be set to `false` on DW (e.g, DataWarehouse) server SKUs.
        pub transparent_data_encryption_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Boolean flag to specify whether TDE automatically rotates the encryption Key to latest version or not. Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** When the `sku_name` is `DW100c`, the `transparent_data_encryption_key_automatic_rotation_enabled` and the `transparent_data_encryption_key_vault_key_id` properties should not be specified, as database-level CMK is not supported for Data Warehouse SKUs.
        pub transparent_data_encryption_key_automatic_rotation_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The fully versioned `Key Vault` `Key` URL (e.g. `'https://<YourVaultName>.vault.azure.net/keys/<YourKeyName>/<YourKeyVersion>`) to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        ///
        /// > **NOTE:** To successfully deploy a `Microsoft SQL Database` in CMK/BYOK TDE the `Key Vault` must have `Soft-delete` and `purge protection` enabled to protect from data loss due to accidental key and/or key vault deletion. The `Key Vault` and the `Microsoft SQL Server` `User Managed Identity Instance` must belong to the same `Azure Active Directory` `tenant`.
        pub transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Whether or not this database is zone redundant, which means the replicas of this database will be spread across multiple availability zones. This property is only settable for Premium and Business Critical databases.
        pub zone_redundant: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_pause_delay_in_minutes_binding = args
            .auto_pause_delay_in_minutes
            .get_output(context)
            .get_inner();
        let collation_binding = args.collation.get_output(context).get_inner();
        let create_mode_binding = args.create_mode.get_output(context).get_inner();
        let creation_source_database_id_binding = args
            .creation_source_database_id
            .get_output(context)
            .get_inner();
        let elastic_pool_id_binding = args
            .elastic_pool_id
            .get_output(context)
            .get_inner();
        let enclave_type_binding = args.enclave_type.get_output(context).get_inner();
        let geo_backup_enabled_binding = args
            .geo_backup_enabled
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let import_binding = args.import.get_output(context).get_inner();
        let ledger_enabled_binding = args.ledger_enabled.get_output(context).get_inner();
        let license_type_binding = args.license_type.get_output(context).get_inner();
        let long_term_retention_policy_binding = args
            .long_term_retention_policy
            .get_output(context)
            .get_inner();
        let maintenance_configuration_name_binding = args
            .maintenance_configuration_name
            .get_output(context)
            .get_inner();
        let max_size_gb_binding = args.max_size_gb.get_output(context).get_inner();
        let min_capacity_binding = args.min_capacity.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let read_replica_count_binding = args
            .read_replica_count
            .get_output(context)
            .get_inner();
        let read_scale_binding = args.read_scale.get_output(context).get_inner();
        let recover_database_id_binding = args
            .recover_database_id
            .get_output(context)
            .get_inner();
        let recovery_point_id_binding = args
            .recovery_point_id
            .get_output(context)
            .get_inner();
        let restore_dropped_database_id_binding = args
            .restore_dropped_database_id
            .get_output(context)
            .get_inner();
        let restore_long_term_retention_backup_id_binding = args
            .restore_long_term_retention_backup_id
            .get_output(context)
            .get_inner();
        let restore_point_in_time_binding = args
            .restore_point_in_time
            .get_output(context)
            .get_inner();
        let sample_name_binding = args.sample_name.get_output(context).get_inner();
        let secondary_type_binding = args.secondary_type.get_output(context).get_inner();
        let server_id_binding = args.server_id.get_output(context).get_inner();
        let short_term_retention_policy_binding = args
            .short_term_retention_policy
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let storage_account_type_binding = args
            .storage_account_type
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let threat_detection_policy_binding = args
            .threat_detection_policy
            .get_output(context)
            .get_inner();
        let transparent_data_encryption_enabled_binding = args
            .transparent_data_encryption_enabled
            .get_output(context)
            .get_inner();
        let transparent_data_encryption_key_automatic_rotation_enabled_binding = args
            .transparent_data_encryption_key_automatic_rotation_enabled
            .get_output(context)
            .get_inner();
        let transparent_data_encryption_key_vault_key_id_binding = args
            .transparent_data_encryption_key_vault_key_id
            .get_output(context)
            .get_inner();
        let zone_redundant_binding = args.zone_redundant.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoPauseDelayInMinutes".into(),
                    value: &auto_pause_delay_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "collation".into(),
                    value: &collation_binding,
                },
                register_interface::ObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding,
                },
                register_interface::ObjectField {
                    name: "creationSourceDatabaseId".into(),
                    value: &creation_source_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "elasticPoolId".into(),
                    value: &elastic_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "enclaveType".into(),
                    value: &enclave_type_binding,
                },
                register_interface::ObjectField {
                    name: "geoBackupEnabled".into(),
                    value: &geo_backup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "import".into(),
                    value: &import_binding,
                },
                register_interface::ObjectField {
                    name: "ledgerEnabled".into(),
                    value: &ledger_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "longTermRetentionPolicy".into(),
                    value: &long_term_retention_policy_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationName".into(),
                    value: &maintenance_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "maxSizeGb".into(),
                    value: &max_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "minCapacity".into(),
                    value: &min_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "readReplicaCount".into(),
                    value: &read_replica_count_binding,
                },
                register_interface::ObjectField {
                    name: "readScale".into(),
                    value: &read_scale_binding,
                },
                register_interface::ObjectField {
                    name: "recoverDatabaseId".into(),
                    value: &recover_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryPointId".into(),
                    value: &recovery_point_id_binding,
                },
                register_interface::ObjectField {
                    name: "restoreDroppedDatabaseId".into(),
                    value: &restore_dropped_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "restoreLongTermRetentionBackupId".into(),
                    value: &restore_long_term_retention_backup_id_binding,
                },
                register_interface::ObjectField {
                    name: "restorePointInTime".into(),
                    value: &restore_point_in_time_binding,
                },
                register_interface::ObjectField {
                    name: "sampleName".into(),
                    value: &sample_name_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryType".into(),
                    value: &secondary_type_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "shortTermRetentionPolicy".into(),
                    value: &short_term_retention_policy_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountType".into(),
                    value: &storage_account_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threatDetectionPolicy".into(),
                    value: &threat_detection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "transparentDataEncryptionEnabled".into(),
                    value: &transparent_data_encryption_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "transparentDataEncryptionKeyAutomaticRotationEnabled".into(),
                    value: &transparent_data_encryption_key_automatic_rotation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "transparentDataEncryptionKeyVaultKeyId".into(),
                    value: &transparent_data_encryption_key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseResult {
            auto_pause_delay_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoPauseDelayInMinutes"),
            ),
            collation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collation"),
            ),
            create_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createMode"),
            ),
            creation_source_database_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationSourceDatabaseId"),
            ),
            elastic_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("elasticPoolId"),
            ),
            enclave_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enclaveType"),
            ),
            geo_backup_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("geoBackupEnabled"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            import: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("import"),
            ),
            ledger_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ledgerEnabled"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            long_term_retention_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("longTermRetentionPolicy"),
            ),
            maintenance_configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceConfigurationName"),
            ),
            max_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxSizeGb"),
            ),
            min_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minCapacity"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            read_replica_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readReplicaCount"),
            ),
            read_scale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readScale"),
            ),
            recover_database_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoverDatabaseId"),
            ),
            recovery_point_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryPointId"),
            ),
            restore_dropped_database_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restoreDroppedDatabaseId"),
            ),
            restore_long_term_retention_backup_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restoreLongTermRetentionBackupId"),
            ),
            restore_point_in_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restorePointInTime"),
            ),
            sample_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sampleName"),
            ),
            secondary_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryType"),
            ),
            server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
            short_term_retention_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortTermRetentionPolicy"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            storage_account_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            threat_detection_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("threatDetectionPolicy"),
            ),
            transparent_data_encryption_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionEnabled"),
            ),
            transparent_data_encryption_key_automatic_rotation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionKeyAutomaticRotationEnabled"),
            ),
            transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transparentDataEncryptionKeyVaultKeyId"),
            ),
            zone_redundant: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneRedundant"),
            ),
        }
    }
}
