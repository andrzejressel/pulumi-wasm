/// ## Example Usage
///
/// ### Testing Glacier Vault Lock Policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["glacier:DeleteArchive",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("NumericLessThanEquals").values(vec!["365",])
///                     .variable("glacier:ArchiveAgeinDays").build_struct(),])
///                     .effect("Deny").resources(vec!["${exampleVault.arn}",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder().name("example").build_struct(),
///     );
///     let exampleVaultLock = vault_lock::create(
///         "exampleVaultLock",
///         VaultLockArgs::builder()
///             .complete_lock(false)
///             .policy("${example.json}")
///             .vault_name("${exampleVault.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Permanently Applying Glacier Vault Lock Policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vault_lock::create(
///         "example",
///         VaultLockArgs::builder()
///             .complete_lock(true)
///             .policy("${exampleAwsIamPolicyDocument.json}")
///             .vault_name("${exampleAwsGlacierVault.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glacier Vault Locks using the Glacier Vault name. For example:
///
/// ```sh
/// $ pulumi import aws:glacier/vaultLock:VaultLock example example-vault
/// ```
pub mod vault_lock {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultLockArgs {
        /// Boolean whether to permanently apply this Glacier Lock Policy. Once completed, this cannot be undone. If set to `false`, the Glacier Lock Policy remains in a testing mode for 24 hours. After that time, the Glacier Lock Policy is automatically removed by Glacier and the this provider resource will show as needing recreation. Changing this from `false` to `true` will show as resource recreation, which is expected. Changing this from `true` to `false` is not possible unless the Glacier Vault is recreated at the same time.
        #[builder(into)]
        pub complete_lock: pulumi_wasm_rust::Output<bool>,
        /// Allow this provider to ignore the error returned when attempting to delete the Glacier Lock Policy. This can be used to delete or recreate the Glacier Vault via this provider, for example, if the Glacier Vault Lock policy permits that action. This should only be used in conjunction with `complete_lock` being set to `true`.
        #[builder(into, default)]
        pub ignore_deletion_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON string containing the IAM policy to apply as the Glacier Vault Lock policy.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The name of the Glacier Vault.
        #[builder(into)]
        pub vault_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VaultLockResult {
        /// Boolean whether to permanently apply this Glacier Lock Policy. Once completed, this cannot be undone. If set to `false`, the Glacier Lock Policy remains in a testing mode for 24 hours. After that time, the Glacier Lock Policy is automatically removed by Glacier and the this provider resource will show as needing recreation. Changing this from `false` to `true` will show as resource recreation, which is expected. Changing this from `true` to `false` is not possible unless the Glacier Vault is recreated at the same time.
        pub complete_lock: pulumi_wasm_rust::Output<bool>,
        /// Allow this provider to ignore the error returned when attempting to delete the Glacier Lock Policy. This can be used to delete or recreate the Glacier Vault via this provider, for example, if the Glacier Vault Lock policy permits that action. This should only be used in conjunction with `complete_lock` being set to `true`.
        pub ignore_deletion_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// JSON string containing the IAM policy to apply as the Glacier Vault Lock policy.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The name of the Glacier Vault.
        pub vault_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VaultLockArgs) -> VaultLockResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let complete_lock_binding = args.complete_lock.get_inner();
        let ignore_deletion_error_binding = args.ignore_deletion_error.get_inner();
        let policy_binding = args.policy.get_inner();
        let vault_name_binding = args.vault_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glacier/vaultLock:VaultLock".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "completeLock".into(),
                    value: &complete_lock_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreDeletionError".into(),
                    value: &ignore_deletion_error_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "vaultName".into(),
                    value: &vault_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "completeLock".into(),
                },
                register_interface::ResultField {
                    name: "ignoreDeletionError".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "vaultName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VaultLockResult {
            complete_lock: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("completeLock").unwrap(),
            ),
            ignore_deletion_error: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreDeletionError").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultName").unwrap(),
            ),
        }
    }
}
