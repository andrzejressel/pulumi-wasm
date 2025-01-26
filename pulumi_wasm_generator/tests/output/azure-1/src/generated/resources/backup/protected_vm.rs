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
pub mod protected_vm {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectedVMArgs {
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        #[builder(into, default)]
        pub backup_policy_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        #[builder(into, default)]
        pub exclude_disk_luns: pulumi_wasm_rust::InputOrOutput<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        #[builder(into, default)]
        pub include_disk_luns: pulumi_wasm_rust::InputOrOutput<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        #[builder(into, default)]
        pub protection_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        #[builder(into, default)]
        pub source_vm_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProtectedVMResult {
        /// Specifies the id of the backup policy to use. Required in creation or when `protection_stopped` is not specified.
        pub backup_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be excluded for VM Protection.
        pub exclude_disk_luns: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// A list of Disks' Logical Unit Numbers(LUN) to be included for VM Protection.
        pub include_disk_luns: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        /// Specifies Protection state of the backup. Possible values are `Invalid`, `IRPending`, `Protected`, `ProtectionStopped`, `ProtectionError` and `ProtectionPaused`.
        pub protection_state: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group **associated with** the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the VM to backup. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** After creation, the `source_vm_id` property can be removed without forcing a new resource to be created; however, setting it to a different ID will create a new resource.
        /// This allows the source vm to be deleted without having to remove the backup.
        pub source_vm_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProtectedVMArgs,
    ) -> ProtectedVMResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_id_binding = args
            .backup_policy_id
            .get_output(context)
            .get_inner();
        let exclude_disk_luns_binding = args
            .exclude_disk_luns
            .get_output(context)
            .get_inner();
        let include_disk_luns_binding = args
            .include_disk_luns
            .get_output(context)
            .get_inner();
        let protection_state_binding = args
            .protection_state
            .get_output(context)
            .get_inner();
        let recovery_vault_name_binding = args
            .recovery_vault_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let source_vm_id_binding = args.source_vm_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:backup/protectedVM:ProtectedVM".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPolicyId".into(),
                    value: &backup_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "excludeDiskLuns".into(),
                    value: &exclude_disk_luns_binding,
                },
                register_interface::ObjectField {
                    name: "includeDiskLuns".into(),
                    value: &include_disk_luns_binding,
                },
                register_interface::ObjectField {
                    name: "protectionState".into(),
                    value: &protection_state_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceVmId".into(),
                    value: &source_vm_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "excludeDiskLuns".into(),
                },
                register_interface::ResultField {
                    name: "includeDiskLuns".into(),
                },
                register_interface::ResultField {
                    name: "protectionState".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceVmId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProtectedVMResult {
            backup_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPolicyId").unwrap(),
            ),
            exclude_disk_luns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeDiskLuns").unwrap(),
            ),
            include_disk_luns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeDiskLuns").unwrap(),
            ),
            protection_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectionState").unwrap(),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_vm_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceVmId").unwrap(),
            ),
        }
    }
}
