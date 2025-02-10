/// Manages Azure Backup for an Azure VM
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: tfex-recovery_vault
///       location: West Europe
///   exampleVault:
///     type: azure:recoveryservices:Vault
///     name: example
///     properties:
///       name: tfex-recovery-vault
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard
///   examplePolicyVM:
///     type: azure:backup:PolicyVM
///     name: example
///     properties:
///       name: tfex-recovery-vault-policy
///       resourceGroupName: ${exampleResourceGroup.name}
///       recoveryVaultName: ${exampleVault.name}
///       backup:
///         frequency: Daily
///         time: 23:00
///       retentionDaily:
///         count: 10
///   vm1:
///     type: azure:backup:ProtectedVM
///     properties:
///       resourceGroupName: ${exampleResourceGroup.name}
///       recoveryVaultName: ${exampleVault.name}
///       sourceVmId: ${example.id}
///       backupPolicyId: ${examplePolicyVM.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:compute:getVirtualMachine
///       arguments:
///         name: example-vm
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Recovery Services Protected VMs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:backup/protectedVM:ProtectedVM item1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/example-recovery-vault/backupFabrics/Azure/protectionContainers/iaasvmcontainer;iaasvmcontainerv2;group1;vm1/protectedItems/vm;iaasvmcontainerv2;group1;vm1"
/// ```
///
/// Note the ID requires quoting as there are semicolons
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protected_vm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectedVMArgs {
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        #[builder(into, default)]
        pub backup_policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        #[builder(into, default)]
        pub exclude_disk_luns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        #[builder(into, default)]
        pub include_disk_luns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        #[builder(into, default)]
        pub protection_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        #[builder(into, default)]
        pub source_vm_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProtectedVMResult {
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        pub backup_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        pub exclude_disk_luns: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        pub include_disk_luns: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        pub protection_state: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        pub source_vm_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectedVMArgs,
    ) -> ProtectedVMResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_policy_id_binding = args.backup_policy_id.get_output(context);
        let exclude_disk_luns_binding = args.exclude_disk_luns.get_output(context);
        let include_disk_luns_binding = args.include_disk_luns.get_output(context);
        let protection_state_binding = args.protection_state.get_output(context);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let source_vm_id_binding = args.source_vm_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:backup/protectedVM:ProtectedVM".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicyId".into(),
                    value: backup_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeDiskLuns".into(),
                    value: exclude_disk_luns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeDiskLuns".into(),
                    value: include_disk_luns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectionState".into(),
                    value: protection_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultName".into(),
                    value: recovery_vault_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVmId".into(),
                    value: source_vm_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProtectedVMResult {
            backup_policy_id: o.get_field("backupPolicyId"),
            exclude_disk_luns: o.get_field("excludeDiskLuns"),
            include_disk_luns: o.get_field("includeDiskLuns"),
            protection_state: o.get_field("protectionState"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_vm_id: o.get_field("sourceVmId"),
        }
    }
}
