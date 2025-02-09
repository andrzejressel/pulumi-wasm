/// Manages a VMWare replicated VM using Azure Site Recovery (VMWare to Azure only). A replicated VM keeps a copiously updated image of the VM in Azure in order to be able to start the VM in Azure in case of a disaster.
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
///             .location("West US")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_kind("StorageV2")
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorageacc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["192.168.2.0/24",])
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVMWareReplicationPolicy = vm_ware_replication_policy::create(
///         "exampleVMWareReplicationPolicy",
///         VmWareReplicationPolicyArgs::builder()
///             .application_consistent_snapshot_frequency_in_minutes(240)
///             .name("example-policy")
///             .recovery_point_retention_in_minutes(1440)
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["192.168.2.0/24",])
///             .location("${example.location}")
///             .name("example-net")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVmwareReplicatedVm = vmware_replicated_vm::create(
///         "exampleVmwareReplicatedVm",
///         VmwareReplicatedVmArgs::builder()
///             .appliance_name("example-appliance")
///             .default_log_storage_account_id("${exampleAccount.id}")
///             .default_recovery_disk_type("Standard_LRS")
///             .license_type("NotSpecified")
///             .name("example-vmware-vm")
///             .network_interfaces(
///                 vec![
///                     VmwareReplicatedVmNetworkInterface::builder().isPrimary(true)
///                     .sourceMacAddress("00:00:00:00:00:00")
///                     .targetSubnetName("${exampleSubnet.name}").build_struct(),
///                 ],
///             )
///             .physical_server_credential_name("example-creds")
///             .recovery_replication_policy_id(
///                 "${exampleAzurermSiteRecoveryVmwareReplicationPolicyAssociation.policyId}",
///             )
///             .recovery_vault_id("${exampleVault.id}")
///             .source_vm_name("example-vm")
///             .target_boot_diagnostics_storage_account_id("${exampleAccount.id}")
///             .target_network_id("${exampleVirtualNetwork.id}")
///             .target_resource_group_id("${example.id}")
///             .target_vm_name("example_replicated_vm")
///             .build_struct(),
///     );
///     let test = vmware_replication_policy_association::create(
///         "test",
///         VmwareReplicationPolicyAssociationArgs::builder()
///             .name("example-association")
///             .policy_id("${exampleVMWareReplicationPolicy.id}")
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Site Recovery VMWare Replicated VM's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/vmwareReplicatedVm:VmwareReplicatedVm vmreplication /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/fabric-name/replicationProtectionContainers/protection-container-name/replicationProtectedItems/vm-replication-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vmware_replicated_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VmwareReplicatedVmArgs {
        /// The name of VMWare appliance which handles the replication. Changing this forces a new resource to be created.
        #[builder(into)]
        pub appliance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the stroage account that should be used for logging during replication.
        ///
        /// > **Note:** Only standard types of storage accounts are allowed.
        ///
        /// > **Note:** Only one of `default_log_storage_account_id` or `managed_disk` must be specified.
        ///
        /// > **Note:** Changing `default_log_storage_account_id` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_log_storage_account_id` co-exist with `managed_disk`, the value of `default_log_storage_account_id` must be as same as `log_storage_account_id` of every `managed_disk` or it forces a new resource to be created.
        #[builder(into, default)]
        pub default_log_storage_account_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The type of storage account that should be used for recovery disks when a failover is done. Possible values are `Premium_LRS`, `Standard_LRS` and `StandardSSD_LRS`.
        ///
        /// > **Note:** Only one of `default_recovery_disk_type` or `managed_disk` must be specified.
        ///
        /// > **Note:** Changing `default_recovery_disk_type` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_recovery_disk_type` co-exist with `managed_disk`, the value of `default_recovery_disk_type` must be as same as `target_disk_type` of every `managed_disk` or it forces a new resource to be created.
        #[builder(into, default)]
        pub default_recovery_disk_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the default Disk Encryption Set that should be used for the disks when a failover is done.
        ///
        /// > **Note:** Changing `default_target_disk_encryption_set_id` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_target_disk_encryption_set_id` co-exist with `managed_disk`, the value of `default_target_disk_encryption_set_id` must be as same as `target_disk_encryption_set_id` of every `managed_disk` or it forces a new resource to be created.
        #[builder(into, default)]
        pub default_target_disk_encryption_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The license type of the VM. Possible values are `NoLicenseType`, `NotSpecified` and `WindowsServer`. Defaults to `NotSpecified`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `managed_disk` block as defined below. It's available only if mobility service is already installed on the source VM.
        ///
        /// > **Note:** A replicated VM could be created without `managed_disk` block, once the block has been specified, changing it expect removing it forces a new resource to be created.
        #[builder(into, default)]
        pub managed_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::siterecovery::VmwareReplicatedVmManagedDisk>>,
        >,
        /// Name of group in which all machines will replicate together and have shared crash consistent and app-consistent recovery points when failed over.
        #[builder(into, default)]
        pub multi_vm_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the replicated VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_interface` block as defined below.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::siterecovery::VmwareReplicatedVmNetworkInterface,
                >,
            >,
        >,
        /// The name of the credential to access the source VM. Changing this forces a new resource to be created. More information about the credentials could be found [here](https://learn.microsoft.com/en-us/azure/site-recovery/deploy-vmware-azure-replication-appliance-modernized).
        #[builder(into)]
        pub physical_server_credential_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the policy to use for this replicated VM.
        #[builder(into)]
        pub recovery_replication_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Recovery Services Vault where the replicated VM is created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the source VM in VMWare. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_vm_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of availability set that the new VM should belong to when a failover is done.
        #[builder(into, default)]
        pub target_availability_set_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the storage account that should be used for boot diagnostics when a failover is done.
        #[builder(into, default)]
        pub target_boot_diagnostics_storage_account_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of network to use when a failover is done.
        ///
        /// > **Note:** `target_network_id` is required when `network_interface` is specified.
        #[builder(into, default)]
        pub target_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of Proximity Placement Group the new VM should belong to when a failover is done.
        ///
        /// > **Note:** Only one of `target_availability_set_id` or `target_zone` can be specified.
        #[builder(into, default)]
        pub target_proximity_placement_group_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of resource group where the VM should be created when a failover is done.
        #[builder(into)]
        pub target_resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the VM that should be created when a failover is done. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_vm_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Size of the VM that should be created when a failover is done, such as `Standard_F2`. If it's not specified, it will automatically be set by detecting the source VM size.
        #[builder(into, default)]
        pub target_vm_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Availability Zone where the Failover VM should exist.
        #[builder(into, default)]
        pub target_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of network to use when a test failover is done.
        #[builder(into, default)]
        pub test_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VmwareReplicatedVmResult {
        /// The name of VMWare appliance which handles the replication. Changing this forces a new resource to be created.
        pub appliance_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the stroage account that should be used for logging during replication.
        ///
        /// > **Note:** Only standard types of storage accounts are allowed.
        ///
        /// > **Note:** Only one of `default_log_storage_account_id` or `managed_disk` must be specified.
        ///
        /// > **Note:** Changing `default_log_storage_account_id` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_log_storage_account_id` co-exist with `managed_disk`, the value of `default_log_storage_account_id` must be as same as `log_storage_account_id` of every `managed_disk` or it forces a new resource to be created.
        pub default_log_storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of storage account that should be used for recovery disks when a failover is done. Possible values are `Premium_LRS`, `Standard_LRS` and `StandardSSD_LRS`.
        ///
        /// > **Note:** Only one of `default_recovery_disk_type` or `managed_disk` must be specified.
        ///
        /// > **Note:** Changing `default_recovery_disk_type` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_recovery_disk_type` co-exist with `managed_disk`, the value of `default_recovery_disk_type` must be as same as `target_disk_type` of every `managed_disk` or it forces a new resource to be created.
        pub default_recovery_disk_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the default Disk Encryption Set that should be used for the disks when a failover is done.
        ///
        /// > **Note:** Changing `default_target_disk_encryption_set_id` forces a new resource to be created. But removing it does not.
        ///
        /// > **Note:** When `default_target_disk_encryption_set_id` co-exist with `managed_disk`, the value of `default_target_disk_encryption_set_id` must be as same as `target_disk_encryption_set_id` of every `managed_disk` or it forces a new resource to be created.
        pub default_target_disk_encryption_set_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The license type of the VM. Possible values are `NoLicenseType`, `NotSpecified` and `WindowsServer`. Defaults to `NotSpecified`.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `managed_disk` block as defined below. It's available only if mobility service is already installed on the source VM.
        ///
        /// > **Note:** A replicated VM could be created without `managed_disk` block, once the block has been specified, changing it expect removing it forces a new resource to be created.
        pub managed_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::siterecovery::VmwareReplicatedVmManagedDisk>>,
        >,
        /// Name of group in which all machines will replicate together and have shared crash consistent and app-consistent recovery points when failed over.
        pub multi_vm_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the replicated VM. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_interface` block as defined below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::siterecovery::VmwareReplicatedVmNetworkInterface,
                >,
            >,
        >,
        /// The name of the credential to access the source VM. Changing this forces a new resource to be created. More information about the credentials could be found [here](https://learn.microsoft.com/en-us/azure/site-recovery/deploy-vmware-azure-replication-appliance-modernized).
        pub physical_server_credential_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the policy to use for this replicated VM.
        pub recovery_replication_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Recovery Services Vault where the replicated VM is created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the source VM in VMWare. Changing this forces a new resource to be created.
        pub source_vm_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of availability set that the new VM should belong to when a failover is done.
        pub target_availability_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the storage account that should be used for boot diagnostics when a failover is done.
        pub target_boot_diagnostics_storage_account_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of network to use when a failover is done.
        ///
        /// > **Note:** `target_network_id` is required when `network_interface` is specified.
        pub target_network_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of Proximity Placement Group the new VM should belong to when a failover is done.
        ///
        /// > **Note:** Only one of `target_availability_set_id` or `target_zone` can be specified.
        pub target_proximity_placement_group_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of resource group where the VM should be created when a failover is done.
        pub target_resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the VM that should be created when a failover is done. Changing this forces a new resource to be created.
        pub target_vm_name: pulumi_gestalt_rust::Output<String>,
        /// Size of the VM that should be created when a failover is done, such as `Standard_F2`. If it's not specified, it will automatically be set by detecting the source VM size.
        pub target_vm_size: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Availability Zone where the Failover VM should exist.
        pub target_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of network to use when a test failover is done.
        pub test_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VmwareReplicatedVmArgs,
    ) -> VmwareReplicatedVmResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let appliance_name_binding = args.appliance_name.get_output(context);
        let default_log_storage_account_id_binding = args
            .default_log_storage_account_id
            .get_output(context);
        let default_recovery_disk_type_binding = args
            .default_recovery_disk_type
            .get_output(context);
        let default_target_disk_encryption_set_id_binding = args
            .default_target_disk_encryption_set_id
            .get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let managed_disks_binding = args.managed_disks.get_output(context);
        let multi_vm_group_name_binding = args.multi_vm_group_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let physical_server_credential_name_binding = args
            .physical_server_credential_name
            .get_output(context);
        let recovery_replication_policy_id_binding = args
            .recovery_replication_policy_id
            .get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let source_vm_name_binding = args.source_vm_name.get_output(context);
        let target_availability_set_id_binding = args
            .target_availability_set_id
            .get_output(context);
        let target_boot_diagnostics_storage_account_id_binding = args
            .target_boot_diagnostics_storage_account_id
            .get_output(context);
        let target_network_id_binding = args.target_network_id.get_output(context);
        let target_proximity_placement_group_id_binding = args
            .target_proximity_placement_group_id
            .get_output(context);
        let target_resource_group_id_binding = args
            .target_resource_group_id
            .get_output(context);
        let target_vm_name_binding = args.target_vm_name.get_output(context);
        let target_vm_size_binding = args.target_vm_size.get_output(context);
        let target_zone_binding = args.target_zone.get_output(context);
        let test_network_id_binding = args.test_network_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/vmwareReplicatedVm:VmwareReplicatedVm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applianceName".into(),
                    value: appliance_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultLogStorageAccountId".into(),
                    value: default_log_storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRecoveryDiskType".into(),
                    value: default_recovery_disk_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultTargetDiskEncryptionSetId".into(),
                    value: default_target_disk_encryption_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseType".into(),
                    value: license_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedDisks".into(),
                    value: managed_disks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiVmGroupName".into(),
                    value: multi_vm_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: network_interfaces_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "physicalServerCredentialName".into(),
                    value: physical_server_credential_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryReplicationPolicyId".into(),
                    value: recovery_replication_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: recovery_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVmName".into(),
                    value: source_vm_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetAvailabilitySetId".into(),
                    value: target_availability_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetBootDiagnosticsStorageAccountId".into(),
                    value: target_boot_diagnostics_storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNetworkId".into(),
                    value: target_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetProximityPlacementGroupId".into(),
                    value: target_proximity_placement_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceGroupId".into(),
                    value: target_resource_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVmName".into(),
                    value: target_vm_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVmSize".into(),
                    value: target_vm_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetZone".into(),
                    value: target_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "testNetworkId".into(),
                    value: test_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VmwareReplicatedVmResult {
            appliance_name: o.get_field("applianceName"),
            default_log_storage_account_id: o.get_field("defaultLogStorageAccountId"),
            default_recovery_disk_type: o.get_field("defaultRecoveryDiskType"),
            default_target_disk_encryption_set_id: o
                .get_field("defaultTargetDiskEncryptionSetId"),
            license_type: o.get_field("licenseType"),
            managed_disks: o.get_field("managedDisks"),
            multi_vm_group_name: o.get_field("multiVmGroupName"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            physical_server_credential_name: o.get_field("physicalServerCredentialName"),
            recovery_replication_policy_id: o.get_field("recoveryReplicationPolicyId"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
            source_vm_name: o.get_field("sourceVmName"),
            target_availability_set_id: o.get_field("targetAvailabilitySetId"),
            target_boot_diagnostics_storage_account_id: o
                .get_field("targetBootDiagnosticsStorageAccountId"),
            target_network_id: o.get_field("targetNetworkId"),
            target_proximity_placement_group_id: o
                .get_field("targetProximityPlacementGroupId"),
            target_resource_group_id: o.get_field("targetResourceGroupId"),
            target_vm_name: o.get_field("targetVmName"),
            target_vm_size: o.get_field("targetVmSize"),
            target_zone: o.get_field("targetZone"),
            test_network_id: o.get_field("testNetworkId"),
        }
    }
}
