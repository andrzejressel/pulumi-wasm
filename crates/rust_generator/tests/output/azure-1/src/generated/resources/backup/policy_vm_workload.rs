/// Manages an Azure VM Workload Backup Policy within a Recovery Services vault.
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
///             .name("example-bpvmw")
///             .build_struct(),
///     );
///     let examplePolicyVMWorkload = policy_vm_workload::create(
///         "examplePolicyVMWorkload",
///         PolicyVmWorkloadArgs::builder()
///             .name("example-bpvmw")
///             .protection_policies(
///                 vec![
///                     PolicyVmWorkloadProtectionPolicy::builder()
///                     .backup(PolicyVmWorkloadProtectionPolicyBackup::builder()
///                     .frequency("Daily").time("15:00").build_struct()).policyType("Full")
///                     .retentionDaily(PolicyVmWorkloadProtectionPolicyRetentionDaily::builder()
///                     .count(8).build_struct()).build_struct(),
///                     PolicyVmWorkloadProtectionPolicy::builder()
///                     .backup(PolicyVmWorkloadProtectionPolicyBackup::builder()
///                     .frequencyInMinutes(15).build_struct()).policyType("Log")
///                     .simpleRetention(PolicyVmWorkloadProtectionPolicySimpleRetention::builder()
///                     .count(8).build_struct()).build_struct(),
///                 ],
///             )
///             .recovery_vault_name("${exampleVault.name}")
///             .resource_group_name("${example.name}")
///             .settings(
///                 PolicyVmWorkloadSettings::builder()
///                     .compressionEnabled(false)
///                     .timeZone("UTC")
///                     .build_struct(),
///             )
///             .workload_type("SQLDataBase")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-rsv")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .soft_delete_enabled(false)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure VM Workload Backup Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:backup/policyVMWorkload:PolicyVMWorkload policy1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/vault1/backupPolicies/policy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_vm_workload {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyVMWorkloadArgs {
        /// The name of the VM Workload Backup Policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `protection_policy` blocks as defined below.
        #[builder(into)]
        pub protection_policies: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::backup::PolicyVmWorkloadProtectionPolicy>,
        >,
        /// The name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the VM Workload Backup Policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `settings` block as defined below.
        #[builder(into)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::backup::PolicyVmWorkloadSettings,
        >,
        /// The VM Workload type for the Backup Policy. Possible values are `SQLDataBase` and `SAPHanaDatabase`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workload_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyVMWorkloadResult {
        /// The name of the VM Workload Backup Policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `protection_policy` blocks as defined below.
        pub protection_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::backup::PolicyVmWorkloadProtectionPolicy>,
        >,
        /// The name of the Recovery Services Vault to use. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the VM Workload Backup Policy. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `settings` block as defined below.
        pub settings: pulumi_gestalt_rust::Output<
            super::super::types::backup::PolicyVmWorkloadSettings,
        >,
        /// The VM Workload type for the Backup Policy. Possible values are `SQLDataBase` and `SAPHanaDatabase`. Changing this forces a new resource to be created.
        pub workload_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PolicyVMWorkloadArgs,
    ) -> PolicyVMWorkloadResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let protection_policies_binding_1 = args.protection_policies.get_output(context);
        let protection_policies_binding = protection_policies_binding_1.get_inner();
        let recovery_vault_name_binding_1 = args.recovery_vault_name.get_output(context);
        let recovery_vault_name_binding = recovery_vault_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let settings_binding_1 = args.settings.get_output(context);
        let settings_binding = settings_binding_1.get_inner();
        let workload_type_binding_1 = args.workload_type.get_output(context);
        let workload_type_binding = workload_type_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:backup/policyVMWorkload:PolicyVMWorkload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protectionPolicies".into(),
                    value: &protection_policies_binding,
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
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "workloadType".into(),
                    value: &workload_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyVMWorkloadResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protection_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectionPolicies"),
            ),
            recovery_vault_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
            workload_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadType"),
            ),
        }
    }
}
