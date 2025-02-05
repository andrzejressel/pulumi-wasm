/// Provides an AWS Backup vault notifications resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testTopic:
///     type: aws:sns:Topic
///     name: test
///     properties:
///       name: backup-vault-events
///   testTopicPolicy:
///     type: aws:sns:TopicPolicy
///     name: test
///     properties:
///       arn: ${testTopic.arn}
///       policy: ${test.json}
///   testVaultNotifications:
///     type: aws:backup:VaultNotifications
///     name: test
///     properties:
///       backupVaultName: example_backup_vault
///       snsTopicArn: ${testTopic.arn}
///       backupVaultEvents:
///         - BACKUP_JOB_STARTED
///         - RESTORE_JOB_COMPLETED
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: __default_policy_ID
///         statements:
///           - actions:
///               - SNS:Publish
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - backup.amazonaws.com
///             resources:
///               - ${testTopic.arn}
///             sid: __default_statement_ID
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup vault notifications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/vaultNotifications:VaultNotifications test TestVault
/// ```
pub mod vault_notifications {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultNotificationsArgs {
        /// An array of events that indicate the status of jobs to back up resources to the backup vault.
        #[builder(into)]
        pub backup_vault_events: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Name of the backup vault to add notifications for.
        #[builder(into)]
        pub backup_vault_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events
        #[builder(into)]
        pub sns_topic_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VaultNotificationsResult {
        /// The ARN of the vault.
        pub backup_vault_arn: pulumi_wasm_rust::Output<String>,
        /// An array of events that indicate the status of jobs to back up resources to the backup vault.
        pub backup_vault_events: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the backup vault to add notifications for.
        pub backup_vault_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) that specifies the topic for a backup vault’s events
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VaultNotificationsArgs,
    ) -> VaultNotificationsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_events_binding = args
            .backup_vault_events
            .get_output(context)
            .get_inner();
        let backup_vault_name_binding = args
            .backup_vault_name
            .get_output(context)
            .get_inner();
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/vaultNotifications:VaultNotifications".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultEvents".into(),
                    value: &backup_vault_events_binding,
                },
                register_interface::ObjectField {
                    name: "backupVaultName".into(),
                    value: &backup_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VaultNotificationsResult {
            backup_vault_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupVaultArn"),
            ),
            backup_vault_events: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupVaultEvents"),
            ),
            backup_vault_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupVaultName"),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snsTopicArn"),
            ),
        }
    }
}
