/// Provides an AWS Backup vault lock configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = vault_lock_configuration::create(
///         "test",
///         VaultLockConfigurationArgs::builder()
///             .backup_vault_name("example_backup_vault")
///             .changeable_for_days(3)
///             .max_retention_days(1200)
///             .min_retention_days(7)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup vault lock configuration using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/vaultLockConfiguration:VaultLockConfiguration test TestVault
/// ```
pub mod vault_lock_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultLockConfigurationArgs {
        /// Name of the backup vault to add a lock configuration for.
        #[builder(into)]
        pub backup_vault_name: pulumi_wasm_rust::Output<String>,
        /// The number of days before the lock date. If omitted creates a vault lock in `governance` mode, otherwise it will create a vault lock in `compliance` mode.
        #[builder(into, default)]
        pub changeable_for_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum retention period that the vault retains its recovery points.
        #[builder(into, default)]
        pub max_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum retention period that the vault retains its recovery points.
        #[builder(into, default)]
        pub min_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VaultLockConfigurationResult {
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the backup vault to add a lock configuration for.
        pub backup_vault_name: pulumi_wasm_rust::Output<String>,
        /// The number of days before the lock date. If omitted creates a vault lock in `governance` mode, otherwise it will create a vault lock in `compliance` mode.
        pub changeable_for_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum retention period that the vault retains its recovery points.
        pub max_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum retention period that the vault retains its recovery points.
        pub min_retention_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VaultLockConfigurationArgs,
    ) -> VaultLockConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_name_binding = args.backup_vault_name.get_inner();
        let changeable_for_days_binding = args.changeable_for_days.get_inner();
        let max_retention_days_binding = args.max_retention_days.get_inner();
        let min_retention_days_binding = args.min_retention_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/vaultLockConfiguration:VaultLockConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultName".into(),
                    value: &backup_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "changeableForDays".into(),
                    value: &changeable_for_days_binding,
                },
                register_interface::ObjectField {
                    name: "maxRetentionDays".into(),
                    value: &max_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "minRetentionDays".into(),
                    value: &min_retention_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupVaultArn".into(),
                },
                register_interface::ResultField {
                    name: "backupVaultName".into(),
                },
                register_interface::ResultField {
                    name: "changeableForDays".into(),
                },
                register_interface::ResultField {
                    name: "maxRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "minRetentionDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VaultLockConfigurationResult {
            backup_vault_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultArn").unwrap(),
            ),
            backup_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultName").unwrap(),
            ),
            changeable_for_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("changeableForDays").unwrap(),
            ),
            max_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRetentionDays").unwrap(),
            ),
            min_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minRetentionDays").unwrap(),
            ),
        }
    }
}