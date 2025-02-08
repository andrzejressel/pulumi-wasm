/// Provides an AWS Backup vault policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleVault:
///     type: aws:backup:Vault
///     name: example
///     properties:
///       name: example
///   exampleVaultPolicy:
///     type: aws:backup:VaultPolicy
///     name: example
///     properties:
///       backupVaultName: ${exampleVault.name}
///       policy: ${example.json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - ${current.accountId}
///             actions:
///               - backup:DescribeBackupVault
///               - backup:DeleteBackupVault
///               - backup:PutBackupVaultAccessPolicy
///               - backup:DeleteBackupVaultAccessPolicy
///               - backup:GetBackupVaultAccessPolicy
///               - backup:StartBackupJob
///               - backup:GetBackupVaultNotifications
///               - backup:PutBackupVaultNotifications
///             resources:
///               - ${exampleVault.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup vault policy using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/vaultPolicy:VaultPolicy test TestVault
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod vault_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultPolicyArgs {
        /// Name of the backup vault to add policy for.
        #[builder(into)]
        pub backup_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The backup vault access policy document in JSON format.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VaultPolicyResult {
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the backup vault to add policy for.
        pub backup_vault_name: pulumi_gestalt_rust::Output<String>,
        /// The backup vault access policy document in JSON format.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VaultPolicyArgs,
    ) -> VaultPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_vault_name_binding = args
            .backup_vault_name
            .get_output(context)
            .get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/vaultPolicy:VaultPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultName".into(),
                    value: &backup_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VaultPolicyResult {
            backup_vault_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupVaultArn"),
            ),
            backup_vault_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupVaultName"),
            ),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
        }
    }
}
