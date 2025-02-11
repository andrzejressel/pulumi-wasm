/// Manages a Security Alert Policy for a Synapse SQL Pool.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       aadAdmin:
///         - login: AzureAD Admin
///           objectId: 00000000-0000-0000-0000-000000000000
///           tenantId: 00000000-0000-0000-0000-000000000000
///       identity:
///         type: SystemAssigned
///       tags:
///         Env: production
///   exampleSqlPool:
///     type: azure:synapse:SqlPool
///     name: example
///     properties:
///       name: examplesqlpool
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       skuName: DW100c
///       createMode: Default
///   auditLogs:
///     type: azure:storage:Account
///     name: audit_logs
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleSqlPoolSecurityAlertPolicy:
///     type: azure:synapse:SqlPoolSecurityAlertPolicy
///     name: example
///     properties:
///       sqlPoolId: ${exampleSqlPool.id}
///       policyState: Enabled
///       storageEndpoint: ${auditLogs.primaryBlobEndpoint}
///       storageAccountAccessKey: ${auditLogs.primaryAccessKey}
///       disabledAlerts:
///         - Sql_Injection
///         - Data_Exfiltration
///       retentionDays: 20
/// ```
///
/// ## Import
///
/// Synapse SQL Pool Security Alert Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sqlPoolSecurityAlertPolicy:SqlPoolSecurityAlertPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/sqlPools/sqlPool1/securityAlertPolicies/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_pool_security_alert_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolSecurityAlertPolicyArgs {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        #[builder(into, default)]
        pub disabled_alerts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        #[builder(into, default)]
        pub email_account_admins_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies an array of email addresses to which the alert is sent.
        #[builder(into, default)]
        pub email_addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific SQL pool. Possible values are `Disabled`, `Enabled` and `New`.
        #[builder(into)]
        pub policy_state: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        #[builder(into, default)]
        pub retention_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the ID of the Synapse SQL Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the identifier key of the Threat Detection audit storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SqlPoolSecurityAlertPolicyResult {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        pub disabled_alerts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        pub email_account_admins_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        pub email_addresses: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific SQL pool. Possible values are `Disabled`, `Enabled` and `New`.
        pub policy_state: pulumi_gestalt_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        pub retention_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the ID of the Synapse SQL Pool. Changing this forces a new resource to be created.
        pub sql_pool_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier key of the Threat Detection audit storage account.
        pub storage_account_access_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        pub storage_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlPoolSecurityAlertPolicyArgs,
    ) -> SqlPoolSecurityAlertPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disabled_alerts_binding = args.disabled_alerts.get_output(context);
        let email_account_admins_enabled_binding = args
            .email_account_admins_enabled
            .get_output(context);
        let email_addresses_binding = args.email_addresses.get_output(context);
        let policy_state_binding = args.policy_state.get_output(context);
        let retention_days_binding = args.retention_days.get_output(context);
        let sql_pool_id_binding = args.sql_pool_id.get_output(context);
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context);
        let storage_endpoint_binding = args.storage_endpoint.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolSecurityAlertPolicy:SqlPoolSecurityAlertPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabledAlerts".into(),
                    value: &disabled_alerts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAccountAdminsEnabled".into(),
                    value: &email_account_admins_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyState".into(),
                    value: &policy_state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlPoolId".into(),
                    value: &sql_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlPoolSecurityAlertPolicyResult {
            disabled_alerts: o.get_field("disabledAlerts"),
            email_account_admins_enabled: o.get_field("emailAccountAdminsEnabled"),
            email_addresses: o.get_field("emailAddresses"),
            policy_state: o.get_field("policyState"),
            retention_days: o.get_field("retentionDays"),
            sql_pool_id: o.get_field("sqlPoolId"),
            storage_account_access_key: o.get_field("storageAccountAccessKey"),
            storage_endpoint: o.get_field("storageEndpoint"),
        }
    }
}
