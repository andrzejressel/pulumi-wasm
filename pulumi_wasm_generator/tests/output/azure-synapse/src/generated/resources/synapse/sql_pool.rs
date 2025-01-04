/// Manages a Synapse SQL Pool.
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
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: BlobStorage
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleSqlPool:
///     type: azure:synapse:SqlPool
///     name: example
///     properties:
///       name: examplesqlpool
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       skuName: DW100c
///       createMode: Default
///       storageAccountType: GRS
/// ```
///
/// ## Import
///
/// Synapse SQL Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sqlPool:SqlPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/sqlPools/sqlPool1
/// ```
///
pub mod sql_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolArgs {
        /// The name of the collation to use with this pool, only applicable when `create_mode` is set to `Default`. Azure default is `SQL_LATIN1_GENERAL_CP1_CI_AS`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub collation: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how to create the SQL Pool. Valid values are: `Default`, `Recovery` or `PointInTimeRestore`. Must be `Default` to create a new database. Defaults to `Default`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Is transparent data encryption enabled?
        #[builder(into, default)]
        pub data_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is geo-backup policy enabled? Possible values include `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub geo_backup_policy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Synapse SQL Pool. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Synapse SQL Pool or SQL Database which is to back up, only applicable when `create_mode` is set to `Recovery`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub recovery_database_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `restore` block as defined below. Only applicable when `create_mode` is set to `PointInTimeRestore`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub restore: pulumi_wasm_rust::Output<
            Option<super::super::types::synapse::SqlPoolRestore>,
        >,
        /// Specifies the SKU Name for this Synapse SQL Pool. Possible values are `DW100c`, `DW200c`, `DW300c`, `DW400c`, `DW500c`, `DW1000c`, `DW1500c`, `DW2000c`, `DW2500c`, `DW3000c`, `DW5000c`, `DW6000c`, `DW7500c`, `DW10000c`, `DW15000c` or `DW30000c`.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The storage account type that will be used to store backups for this Synapse SQL Pool. Possible values are `LRS` or `GRS`. Changing this forces a new Synapse SQL Pool to be created. Defaults to `GRS`.
        #[builder(into)]
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// The ID of Synapse Workspace within which this SQL Pool should be created. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Synapse SQL Pool.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlPoolResult {
        /// The name of the collation to use with this pool, only applicable when `create_mode` is set to `Default`. Azure default is `SQL_LATIN1_GENERAL_CP1_CI_AS`. Changing this forces a new Synapse SQL Pool to be created.
        pub collation: pulumi_wasm_rust::Output<String>,
        /// Specifies how to create the SQL Pool. Valid values are: `Default`, `Recovery` or `PointInTimeRestore`. Must be `Default` to create a new database. Defaults to `Default`. Changing this forces a new Synapse SQL Pool to be created.
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Is transparent data encryption enabled?
        pub data_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is geo-backup policy enabled? Possible values include `true` or `false`. Defaults to `true`.
        pub geo_backup_policy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Synapse SQL Pool. Changing this forces a new Synapse SQL Pool to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Synapse SQL Pool or SQL Database which is to back up, only applicable when `create_mode` is set to `Recovery`. Changing this forces a new Synapse SQL Pool to be created.
        pub recovery_database_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `restore` block as defined below. Only applicable when `create_mode` is set to `PointInTimeRestore`. Changing this forces a new Synapse SQL Pool to be created.
        pub restore: pulumi_wasm_rust::Output<
            Option<super::super::types::synapse::SqlPoolRestore>,
        >,
        /// Specifies the SKU Name for this Synapse SQL Pool. Possible values are `DW100c`, `DW200c`, `DW300c`, `DW400c`, `DW500c`, `DW1000c`, `DW1500c`, `DW2000c`, `DW2500c`, `DW3000c`, `DW5000c`, `DW6000c`, `DW7500c`, `DW10000c`, `DW15000c` or `DW30000c`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The storage account type that will be used to store backups for this Synapse SQL Pool. Possible values are `LRS` or `GRS`. Changing this forces a new Synapse SQL Pool to be created. Defaults to `GRS`.
        pub storage_account_type: pulumi_wasm_rust::Output<String>,
        /// The ID of Synapse Workspace within which this SQL Pool should be created. Changing this forces a new Synapse SQL Pool to be created.
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Synapse SQL Pool.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SqlPoolArgs) -> SqlPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let collation_binding = args.collation.get_inner();
        let create_mode_binding = args.create_mode.get_inner();
        let data_encrypted_binding = args.data_encrypted.get_inner();
        let geo_backup_policy_enabled_binding = args
            .geo_backup_policy_enabled
            .get_inner();
        let name_binding = args.name.get_inner();
        let recovery_database_id_binding = args.recovery_database_id.get_inner();
        let restore_binding = args.restore.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let storage_account_type_binding = args.storage_account_type.get_inner();
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/sqlPool:SqlPool".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collation".into(),
                    value: &collation_binding,
                },
                register_interface::ObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding,
                },
                register_interface::ObjectField {
                    name: "dataEncrypted".into(),
                    value: &data_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "geoBackupPolicyEnabled".into(),
                    value: &geo_backup_policy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryDatabaseId".into(),
                    value: &recovery_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "restore".into(),
                    value: &restore_binding,
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
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "createMode".into(),
                },
                register_interface::ResultField {
                    name: "dataEncrypted".into(),
                },
                register_interface::ResultField {
                    name: "geoBackupPolicyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryDatabaseId".into(),
                },
                register_interface::ResultField {
                    name: "restore".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountType".into(),
                },
                register_interface::ResultField {
                    name: "synapseWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SqlPoolResult {
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            create_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createMode").unwrap(),
            ),
            data_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataEncrypted").unwrap(),
            ),
            geo_backup_policy_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoBackupPolicyEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryDatabaseId").unwrap(),
            ),
            restore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restore").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_account_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountType").unwrap(),
            ),
            synapse_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synapseWorkspaceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
