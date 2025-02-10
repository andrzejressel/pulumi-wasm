/// Manages a Backup Vault.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBackupVault = backup_vault::create(
///         "exampleBackupVault",
///         BackupVaultArgs::builder()
///             .datastore_type("VaultStore")
///             .location("${example.location}")
///             .name("example-backup-vault")
///             .redundancy("LocallyRedundant")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Backup Vaults can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupVault:BackupVault example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupVaultArgs {
        /// Whether to enable cross-region restore for the Backup Vault.
        ///
        /// > **Note:** The `cross_region_restore_enabled` can only be specified when `redundancy` is specified for `GeoRedundant`. Once `cross_region_restore_enabled` is enabled, it cannot be disabled.
        #[builder(into, default)]
        pub cross_region_restore_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the type of the data store. Possible values are `ArchiveStore`, `OperationalStore`, `SnapshotStore` and `VaultStore`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `SnapshotStore` will be removed in version 4.0 as it has been replaced by `OperationalStore`.
        #[builder(into)]
        pub datastore_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataprotection::BackupVaultIdentity>,
        >,
        /// The Azure Region where the Backup Vault should exist. Changing this forces a new Backup Vault to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Backup Vault. Changing this forces a new Backup Vault to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the backup storage redundancy. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Changing this forces a new Backup Vault to be created.
        #[builder(into)]
        pub redundancy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Backup Vault should exist. Changing this forces a new Backup Vault to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The soft delete retention duration for this Backup Vault. Possible values are between `14` and `180`. Defaults to `14`.
        ///
        /// > **Note:** The `retention_duration_in_days` is the number of days for which deleted data is retained before being permanently deleted. Retention period till 14 days are free of cost, however, retention beyond 14 days may incur additional charges. The `retention_duration_in_days` is required when the `soft_delete` is set to `On`.
        #[builder(into, default)]
        pub retention_duration_in_days: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The state of soft delete for this Backup Vault. Possible values are `AlwaysOn`, `Off` and `On`. Defaults to `On`.
        ///
        /// > **Note:** Once the `soft_delete` is set to `AlwaysOn`, the setting cannot be changed.
        #[builder(into, default)]
        pub soft_delete: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Backup Vault.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BackupVaultResult {
        /// Whether to enable cross-region restore for the Backup Vault.
        ///
        /// > **Note:** The `cross_region_restore_enabled` can only be specified when `redundancy` is specified for `GeoRedundant`. Once `cross_region_restore_enabled` is enabled, it cannot be disabled.
        pub cross_region_restore_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the type of the data store. Possible values are `ArchiveStore`, `OperationalStore`, `SnapshotStore` and `VaultStore`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `SnapshotStore` will be removed in version 4.0 as it has been replaced by `OperationalStore`.
        pub datastore_type: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataprotection::BackupVaultIdentity>,
        >,
        /// The Azure Region where the Backup Vault should exist. Changing this forces a new Backup Vault to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Backup Vault. Changing this forces a new Backup Vault to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the backup storage redundancy. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Changing this forces a new Backup Vault to be created.
        pub redundancy: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Backup Vault should exist. Changing this forces a new Backup Vault to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The soft delete retention duration for this Backup Vault. Possible values are between `14` and `180`. Defaults to `14`.
        ///
        /// > **Note:** The `retention_duration_in_days` is the number of days for which deleted data is retained before being permanently deleted. Retention period till 14 days are free of cost, however, retention beyond 14 days may incur additional charges. The `retention_duration_in_days` is required when the `soft_delete` is set to `On`.
        pub retention_duration_in_days: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The state of soft delete for this Backup Vault. Possible values are `AlwaysOn`, `Off` and `On`. Defaults to `On`.
        ///
        /// > **Note:** Once the `soft_delete` is set to `AlwaysOn`, the setting cannot be changed.
        pub soft_delete: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Backup Vault.
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
        args: BackupVaultArgs,
    ) -> BackupVaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cross_region_restore_enabled_binding = args
            .cross_region_restore_enabled
            .get_output(context);
        let datastore_type_binding = args.datastore_type.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let redundancy_binding = args.redundancy.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_duration_in_days_binding = args
            .retention_duration_in_days
            .get_output(context);
        let soft_delete_binding = args.soft_delete.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupVault:BackupVault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crossRegionRestoreEnabled".into(),
                    value: cross_region_restore_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datastoreType".into(),
                    value: datastore_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
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
                    name: "redundancy".into(),
                    value: redundancy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionDurationInDays".into(),
                    value: retention_duration_in_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softDelete".into(),
                    value: soft_delete_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupVaultResult {
            cross_region_restore_enabled: o.get_field("crossRegionRestoreEnabled"),
            datastore_type: o.get_field("datastoreType"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            redundancy: o.get_field("redundancy"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_duration_in_days: o.get_field("retentionDurationInDays"),
            soft_delete: o.get_field("softDelete"),
            tags: o.get_field("tags"),
        }
    }
}
