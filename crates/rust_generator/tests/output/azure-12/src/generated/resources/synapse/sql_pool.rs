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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolArgs {
        /// The name of the collation to use with this pool, only applicable when `create_mode` is set to `Default`. Azure default is `SQL_LATIN1_GENERAL_CP1_CI_AS`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub collation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how to create the SQL Pool. Valid values are: `Default`, `Recovery` or `PointInTimeRestore`. Must be `Default` to create a new database. Defaults to `Default`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub create_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is transparent data encryption enabled?
        #[builder(into, default)]
        pub data_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is geo-backup policy enabled? Possible values include `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub geo_backup_policy_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Synapse SQL Pool. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Synapse SQL Pool or SQL Database which is to back up, only applicable when `create_mode` is set to `Recovery`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub recovery_database_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `restore` block as defined below. Only applicable when `create_mode` is set to `PointInTimeRestore`. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into, default)]
        pub restore: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::SqlPoolRestore>,
        >,
        /// Specifies the SKU Name for this Synapse SQL Pool. Possible values are `DW100c`, `DW200c`, `DW300c`, `DW400c`, `DW500c`, `DW1000c`, `DW1500c`, `DW2000c`, `DW2500c`, `DW3000c`, `DW5000c`, `DW6000c`, `DW7500c`, `DW10000c`, `DW15000c` or `DW30000c`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The storage account type that will be used to store backups for this Synapse SQL Pool. Possible values are `LRS` or `GRS`. Changing this forces a new Synapse SQL Pool to be created. Defaults to `GRS`.
        #[builder(into)]
        pub storage_account_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of Synapse Workspace within which this SQL Pool should be created. Changing this forces a new Synapse SQL Pool to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Synapse SQL Pool.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlPoolResult {
        /// The name of the collation to use with this pool, only applicable when `create_mode` is set to `Default`. Azure default is `SQL_LATIN1_GENERAL_CP1_CI_AS`. Changing this forces a new Synapse SQL Pool to be created.
        pub collation: pulumi_gestalt_rust::Output<String>,
        /// Specifies how to create the SQL Pool. Valid values are: `Default`, `Recovery` or `PointInTimeRestore`. Must be `Default` to create a new database. Defaults to `Default`. Changing this forces a new Synapse SQL Pool to be created.
        pub create_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is transparent data encryption enabled?
        pub data_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is geo-backup policy enabled? Possible values include `true` or `false`. Defaults to `true`.
        pub geo_backup_policy_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Synapse SQL Pool. Changing this forces a new Synapse SQL Pool to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Synapse SQL Pool or SQL Database which is to back up, only applicable when `create_mode` is set to `Recovery`. Changing this forces a new Synapse SQL Pool to be created.
        pub recovery_database_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `restore` block as defined below. Only applicable when `create_mode` is set to `PointInTimeRestore`. Changing this forces a new Synapse SQL Pool to be created.
        pub restore: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::SqlPoolRestore>,
        >,
        /// Specifies the SKU Name for this Synapse SQL Pool. Possible values are `DW100c`, `DW200c`, `DW300c`, `DW400c`, `DW500c`, `DW1000c`, `DW1500c`, `DW2000c`, `DW2500c`, `DW3000c`, `DW5000c`, `DW6000c`, `DW7500c`, `DW10000c`, `DW15000c` or `DW30000c`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The storage account type that will be used to store backups for this Synapse SQL Pool. Possible values are `LRS` or `GRS`. Changing this forces a new Synapse SQL Pool to be created. Defaults to `GRS`.
        pub storage_account_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of Synapse Workspace within which this SQL Pool should be created. Changing this forces a new Synapse SQL Pool to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Synapse SQL Pool.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlPoolArgs,
    ) -> SqlPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collation_binding = args.collation.get_output(context);
        let create_mode_binding = args.create_mode.get_output(context);
        let data_encrypted_binding = args.data_encrypted.get_output(context);
        let geo_backup_policy_enabled_binding = args
            .geo_backup_policy_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recovery_database_id_binding = args.recovery_database_id.get_output(context);
        let restore_binding = args.restore.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let storage_account_type_binding = args.storage_account_type.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/sqlPool:SqlPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collation".into(),
                    value: collation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createMode".into(),
                    value: create_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataEncrypted".into(),
                    value: data_encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoBackupPolicyEnabled".into(),
                    value: geo_backup_policy_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryDatabaseId".into(),
                    value: recovery_database_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restore".into(),
                    value: restore_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountType".into(),
                    value: storage_account_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlPoolResult {
            collation: o.get_field("collation"),
            create_mode: o.get_field("createMode"),
            data_encrypted: o.get_field("dataEncrypted"),
            geo_backup_policy_enabled: o.get_field("geoBackupPolicyEnabled"),
            name: o.get_field("name"),
            recovery_database_id: o.get_field("recoveryDatabaseId"),
            restore: o.get_field("restore"),
            sku_name: o.get_field("skuName"),
            storage_account_type: o.get_field("storageAccountType"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
            tags: o.get_field("tags"),
        }
    }
}
