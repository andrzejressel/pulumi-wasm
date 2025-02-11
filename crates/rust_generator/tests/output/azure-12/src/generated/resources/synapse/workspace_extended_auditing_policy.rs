/// Manages a Synapse Workspace Extended Auditing Policy.
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
///   auditLogs:
///     type: azure:storage:Account
///     name: audit_logs
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleWorkspaceExtendedAuditingPolicy:
///     type: azure:synapse:WorkspaceExtendedAuditingPolicy
///     name: example
///     properties:
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       storageEndpoint: ${auditLogs.primaryBlobEndpoint}
///       storageAccountAccessKey: ${auditLogs.primaryAccessKey}
///       storageAccountAccessKeyIsSecondary: false
///       retentionInDays: 6
/// ```
///
/// ## Import
///
/// Synapse Workspace Extended Auditing Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/workspaceExtendedAuditingPolicy:WorkspaceExtendedAuditingPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/extendedAuditingSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_extended_auditing_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceExtendedAuditingPolicyArgs {
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its master database audit events to Azure Monitor. Defaults to `true`.
        #[builder(into, default)]
        pub log_monitoring_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        #[builder(into, default)]
        pub retention_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The access key to use for the auditing storage account.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Is `storage_account_access_key` value the storage's secondary key?
        #[builder(into, default)]
        pub storage_account_access_key_is_secondary: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Synapse workspace to set the extended auditing policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceExtendedAuditingPolicyResult {
        /// Enable audit events to Azure Monitor? To enable server audit events to Azure Monitor, please enable its master database audit events to Azure Monitor. Defaults to `true`.
        pub log_monitoring_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of days to retain logs for in the storage account. Defaults to `0`.
        pub retention_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The access key to use for the auditing storage account.
        pub storage_account_access_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is `storage_account_access_key` value the storage's secondary key?
        pub storage_account_access_key_is_secondary: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all extended auditing logs.
        pub storage_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Synapse workspace to set the extended auditing policy. Changing this forces a new resource to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceExtendedAuditingPolicyArgs,
    ) -> WorkspaceExtendedAuditingPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_monitoring_enabled_binding = args
            .log_monitoring_enabled
            .get_output(context);
        let retention_in_days_binding = args.retention_in_days.get_output(context);
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_output(context);
        let storage_account_access_key_is_secondary_binding = args
            .storage_account_access_key_is_secondary
            .get_output(context);
        let storage_endpoint_binding = args.storage_endpoint.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/workspaceExtendedAuditingPolicy:WorkspaceExtendedAuditingPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logMonitoringEnabled".into(),
                    value: &log_monitoring_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionInDays".into(),
                    value: &retention_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountAccessKeyIsSecondary".into(),
                    value: &storage_account_access_key_is_secondary_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceExtendedAuditingPolicyResult {
            log_monitoring_enabled: o.get_field("logMonitoringEnabled"),
            retention_in_days: o.get_field("retentionInDays"),
            storage_account_access_key: o.get_field("storageAccountAccessKey"),
            storage_account_access_key_is_secondary: o
                .get_field("storageAccountAccessKeyIsSecondary"),
            storage_endpoint: o.get_field("storageEndpoint"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
        }
    }
}
