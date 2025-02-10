/// Manages a Recovery Services Vault.
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
///             .name("tfex-recovery_vault")
///             .build_struct(),
///     );
///     let vault = vault::create(
///         "vault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .soft_delete_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Recovery Services Vaults can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:recoveryservices/vault:Vault vault1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/vault1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultArgs {
        /// Whether to enable the Classic experience for VMware replication. If set to `false` VMware machines will be protected using the new stateless ASR replication appliance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub classic_vmware_replication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Is cross region restore enabled for this Vault? Only can be `true`, when `storage_mode_type` is `GeoRedundant`. Defaults to `false`.
        ///
        /// > **Note:** Once `cross_region_restore_enabled` is set to `true`, changing it back to `false` forces a new Recovery Service Vault to be created.
        #[builder(into, default)]
        pub cross_region_restore_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `encryption` block as defined below. Required with `identity`.
        ///
        /// !> **Note:** Once Encryption with your own key has been Enabled it's not possible to Disable it.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recoveryservices::VaultEncryption>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recoveryservices::VaultIdentity>,
        >,
        /// Immutability Settings of vault, possible values include: `Locked`, `Unlocked` and `Disabled`.
        ///
        /// > **Note:** Once `immutability` is set to `Locked`, changing it to other values forces a new Recovery Services Vault to be created.
        #[builder(into, default)]
        pub immutability: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `monitoring` block as defined below.
        #[builder(into, default)]
        pub monitoring: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::recoveryservices::VaultMonitoring>,
        >,
        /// Specifies the name of the Recovery Services Vault. Recovery Service Vault name must be 2 - 50 characters long, start with a letter, contain only letters, numbers and hyphens. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is it enabled to access the vault from public networks. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Recovery Services Vault. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Sets the vault's SKU. Possible values include: `Standard`, `RS0`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is soft delete enable for this Vault? Defaults to `true`.
        #[builder(into, default)]
        pub soft_delete_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The storage type of the Recovery Services Vault. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Defaults to `GeoRedundant`.
        #[builder(into, default)]
        pub storage_mode_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VaultResult {
        /// Whether to enable the Classic experience for VMware replication. If set to `false` VMware machines will be protected using the new stateless ASR replication appliance. Changing this forces a new resource to be created.
        pub classic_vmware_replication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Is cross region restore enabled for this Vault? Only can be `true`, when `storage_mode_type` is `GeoRedundant`. Defaults to `false`.
        ///
        /// > **Note:** Once `cross_region_restore_enabled` is set to `true`, changing it back to `false` forces a new Recovery Service Vault to be created.
        pub cross_region_restore_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `encryption` block as defined below. Required with `identity`.
        ///
        /// !> **Note:** Once Encryption with your own key has been Enabled it's not possible to Disable it.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultEncryption>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultIdentity>,
        >,
        /// Immutability Settings of vault, possible values include: `Locked`, `Unlocked` and `Disabled`.
        ///
        /// > **Note:** Once `immutability` is set to `Locked`, changing it to other values forces a new Recovery Services Vault to be created.
        pub immutability: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `monitoring` block as defined below.
        pub monitoring: pulumi_gestalt_rust::Output<
            Option<super::super::types::recoveryservices::VaultMonitoring>,
        >,
        /// Specifies the name of the Recovery Services Vault. Recovery Service Vault name must be 2 - 50 characters long, start with a letter, contain only letters, numbers and hyphens. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is it enabled to access the vault from public networks. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Recovery Services Vault. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Sets the vault's SKU. Possible values include: `Standard`, `RS0`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Is soft delete enable for this Vault? Defaults to `true`.
        pub soft_delete_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The storage type of the Recovery Services Vault. Possible values are `GeoRedundant`, `LocallyRedundant` and `ZoneRedundant`. Defaults to `GeoRedundant`.
        pub storage_mode_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
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
        args: VaultArgs,
    ) -> VaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let classic_vmware_replication_enabled_binding = args
            .classic_vmware_replication_enabled
            .get_output(context);
        let cross_region_restore_enabled_binding = args
            .cross_region_restore_enabled
            .get_output(context);
        let encryption_binding = args.encryption.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let immutability_binding = args.immutability.get_output(context);
        let location_binding = args.location.get_output(context);
        let monitoring_binding = args.monitoring.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let soft_delete_enabled_binding = args.soft_delete_enabled.get_output(context);
        let storage_mode_type_binding = args.storage_mode_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:recoveryservices/vault:Vault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "classicVmwareReplicationEnabled".into(),
                    value: classic_vmware_replication_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crossRegionRestoreEnabled".into(),
                    value: cross_region_restore_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "immutability".into(),
                    value: immutability_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoring".into(),
                    value: monitoring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softDeleteEnabled".into(),
                    value: soft_delete_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageModeType".into(),
                    value: storage_mode_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VaultResult {
            classic_vmware_replication_enabled: o
                .get_field("classicVmwareReplicationEnabled"),
            cross_region_restore_enabled: o.get_field("crossRegionRestoreEnabled"),
            encryption: o.get_field("encryption"),
            identity: o.get_field("identity"),
            immutability: o.get_field("immutability"),
            location: o.get_field("location"),
            monitoring: o.get_field("monitoring"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            soft_delete_enabled: o.get_field("softDeleteEnabled"),
            storage_mode_type: o.get_field("storageModeType"),
            tags: o.get_field("tags"),
        }
    }
}
