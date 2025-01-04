/// Manages a Security Alert Policy for a Synapse Workspace.
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
///   auditLogs:
///     type: azure:storage:Account
///     name: audit_logs
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleWorkspaceSecurityAlertPolicy:
///     type: azure:synapse:WorkspaceSecurityAlertPolicy
///     name: example
///     properties:
///       synapseWorkspaceId: ${exampleWorkspace.id}
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
/// Synapse Workspace Security Alert Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/workspaceSecurityAlertPolicy:WorkspaceSecurityAlertPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/securityAlertPolicies/Default
/// ```
///
pub mod workspace_security_alert_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceSecurityAlertPolicyArgs {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        #[builder(into, default)]
        pub disabled_alerts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        #[builder(into, default)]
        pub email_account_admins_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        #[builder(into, default)]
        pub email_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific workspace. Possible values are `Disabled`, `Enabled` and `New`.
        #[builder(into)]
        pub policy_state: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the identifier key of the Threat Detection audit storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Synapse Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceSecurityAlertPolicyResult {
        /// Specifies an array of alerts that are disabled. Allowed values are: `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action`.
        pub disabled_alerts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        pub email_account_admins_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        pub email_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the state of the policy, whether it is enabled or disabled or a policy has not been applied yet on the specific workspace. Possible values are `Disabled`, `Enabled` and `New`.
        pub policy_state: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the identifier key of the Threat Detection audit storage account.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs.
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Synapse Workspace. Changing this forces a new resource to be created.
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WorkspaceSecurityAlertPolicyArgs,
    ) -> WorkspaceSecurityAlertPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_alerts_binding = args.disabled_alerts.get_inner();
        let email_account_admins_enabled_binding = args
            .email_account_admins_enabled
            .get_inner();
        let email_addresses_binding = args.email_addresses.get_inner();
        let policy_state_binding = args.policy_state.get_inner();
        let retention_days_binding = args.retention_days.get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_inner();
        let storage_endpoint_binding = args.storage_endpoint.get_inner();
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/workspaceSecurityAlertPolicy:WorkspaceSecurityAlertPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabledAlerts".into(),
                    value: &disabled_alerts_binding,
                },
                register_interface::ObjectField {
                    name: "emailAccountAdminsEnabled".into(),
                    value: &email_account_admins_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "policyState".into(),
                    value: &policy_state_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disabledAlerts".into(),
                },
                register_interface::ResultField {
                    name: "emailAccountAdminsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "emailAddresses".into(),
                },
                register_interface::ResultField {
                    name: "policyState".into(),
                },
                register_interface::ResultField {
                    name: "retentionDays".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "synapseWorkspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceSecurityAlertPolicyResult {
            disabled_alerts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabledAlerts").unwrap(),
            ),
            email_account_admins_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAccountAdminsEnabled").unwrap(),
            ),
            email_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddresses").unwrap(),
            ),
            policy_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyState").unwrap(),
            ),
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEndpoint").unwrap(),
            ),
            synapse_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synapseWorkspaceId").unwrap(),
            ),
        }
    }
}
