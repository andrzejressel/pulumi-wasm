/// Provides an AWS Backup vault lock configuration resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vault_lock_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultLockConfigurationArgs {
        /// Name of the backup vault to add a lock configuration for.
        #[builder(into)]
        pub backup_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of days before the lock date. If omitted creates a vault lock in `governance` mode, otherwise it will create a vault lock in `compliance` mode.
        #[builder(into, default)]
        pub changeable_for_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum retention period that the vault retains its recovery points.
        #[builder(into, default)]
        pub max_retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The minimum retention period that the vault retains its recovery points.
        #[builder(into, default)]
        pub min_retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VaultLockConfigurationResult {
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the backup vault to add a lock configuration for.
        pub backup_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The number of days before the lock date. If omitted creates a vault lock in `governance` mode, otherwise it will create a vault lock in `compliance` mode.
        pub changeable_for_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum retention period that the vault retains its recovery points.
        pub max_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The minimum retention period that the vault retains its recovery points.
        pub min_retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultLockConfigurationArgs,
    ) -> VaultLockConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_vault_name_binding = args.backup_vault_name.get_output(context);
        let changeable_for_days_binding = args.changeable_for_days.get_output(context);
        let max_retention_days_binding = args.max_retention_days.get_output(context);
        let min_retention_days_binding = args.min_retention_days.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/vaultLockConfiguration:VaultLockConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVaultName".into(),
                    value: backup_vault_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "changeableForDays".into(),
                    value: changeable_for_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxRetentionDays".into(),
                    value: max_retention_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minRetentionDays".into(),
                    value: min_retention_days_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VaultLockConfigurationResult {
            backup_vault_arn: o.get_field("backupVaultArn"),
            backup_vault_name: o.get_field("backupVaultName"),
            changeable_for_days: o.get_field("changeableForDays"),
            max_retention_days: o.get_field("maxRetentionDays"),
            min_retention_days: o.get_field("minRetentionDays"),
        }
    }
}
