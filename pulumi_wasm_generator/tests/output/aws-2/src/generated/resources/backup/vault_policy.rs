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
pub mod vault_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultPolicyArgs {
        /// Name of the backup vault to add policy for.
        #[builder(into)]
        pub backup_vault_name: pulumi_wasm_rust::Output<String>,
        /// The backup vault access policy document in JSON format.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VaultPolicyResult {
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the backup vault to add policy for.
        pub backup_vault_name: pulumi_wasm_rust::Output<String>,
        /// The backup vault access policy document in JSON format.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VaultPolicyArgs) -> VaultPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_name_binding = args.backup_vault_name.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/vaultPolicy:VaultPolicy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupVaultArn".into(),
                },
                register_interface::ResultField {
                    name: "backupVaultName".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VaultPolicyResult {
            backup_vault_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultArn").unwrap(),
            ),
            backup_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultName").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
