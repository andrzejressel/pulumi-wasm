/// Manages a Synapse SQL Pool Extended Auditing Policy.
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
///       accountKind: BlobStorage
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
///       identity:
///         type: SystemAssigned
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
///   exampleSqlPoolExtendedAuditingPolicy:
///     type: azure:synapse:SqlPoolExtendedAuditingPolicy
///     name: example
///     properties:
///       sqlPoolId: ${exampleSqlPool.id}
///       storageEndpoint: ${auditLogs.primaryBlobEndpoint}
///       storageAccountAccessKey: ${auditLogs.primaryAccessKey}
///       storageAccountAccessKeyIsSecondary: false
///       retentionInDays: 6
/// ```
///
/// ## Import
///
/// Synapse SQL Pool Extended Auditing Policys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sqlPoolExtendedAuditingPolicy:SqlPoolExtendedAuditingPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/sqlPools/sqlPool1/extendedAuditingSettings/default
/// ```
///
pub mod sql_pool_extended_auditing_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolExtendedAuditingPolicyArgs {
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its master database audit events to Azure Monitor. Defaults to `true`.
        #[builder(into, default)]
        pub log_monitoring_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Synapse SQL pool to set the extended auditing policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The access key to use for the auditing storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        #[builder(into, default)]
        pub storage_account_access_key_is_secondary: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SqlPoolExtendedAuditingPolicyResult {
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its master database audit events to Azure Monitor. Defaults to `true`.
        pub log_monitoring_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        pub retention_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Synapse SQL pool to set the extended auditing policy. Changing this forces a new resource to be created.
        pub sql_pool_id: pulumi_wasm_rust::Output<String>,
        /// The access key to use for the auditing storage account.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        pub storage_account_access_key_is_secondary: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SqlPoolExtendedAuditingPolicyArgs,
    ) -> SqlPoolExtendedAuditingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_monitoring_enabled_binding = args
            .log_monitoring_enabled
            .get_output(context)
            .get_inner();
        let retention_in_days_binding = args
            .retention_in_days
            .get_output(context)
            .get_inner();
        let sql_pool_id_binding = args.sql_pool_id.get_output(context).get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context)
            .get_inner();
        let storage_account_access_key_is_secondary_binding = args
            .storage_account_access_key_is_secondary
            .get_output(context)
            .get_inner();
        let storage_endpoint_binding = args
            .storage_endpoint
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolExtendedAuditingPolicy:SqlPoolExtendedAuditingPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logMonitoringEnabled".into(),
                    value: &log_monitoring_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "sqlPoolId".into(),
                    value: &sql_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKeyIsSecondary".into(),
                    value: &storage_account_access_key_is_secondary_binding,
                },
                register_interface::ObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlPoolExtendedAuditingPolicyResult {
            log_monitoring_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logMonitoringEnabled"),
            ),
            retention_in_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentionInDays"),
            ),
            sql_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqlPoolId"),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKey"),
            ),
            storage_account_access_key_is_secondary: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountAccessKeyIsSecondary"),
            ),
            storage_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageEndpoint"),
            ),
        }
    }
}
