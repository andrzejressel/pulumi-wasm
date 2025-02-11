/// Manages a Synapse SQL Pool Workload Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: west europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountKind: BlobStorage
///       accountTier: Standard
///       accountReplicationType: LRS
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
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       skuName: DW100c
///       createMode: Default
///   exampleSqlPoolWorkloadGroup:
///     type: azure:synapse:SqlPoolWorkloadGroup
///     name: example
///     properties:
///       name: example
///       sqlPoolId: ${exampleSqlPool.id}
///       importance: normal
///       maxResourcePercent: 100
///       minResourcePercent: 0
///       maxResourcePercentPerRequest: 3
///       minResourcePercentPerRequest: 3
///       queryExecutionTimeoutInSeconds: 0
/// ```
///
/// ## Import
///
/// Synapse SQL Pool Workload Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sqlPoolWorkloadGroup:SqlPoolWorkloadGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Synapse/workspaces/workspace1/sqlPools/sqlPool1/workloadGroups/workloadGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_pool_workload_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadGroupArgs {
        /// The workload group importance level. Defaults to `normal`.
        #[builder(into, default)]
        pub importance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload group cap percentage resource.
        #[builder(into)]
        pub max_resource_percent: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The workload group request maximum grant percentage. Defaults to `3`.
        #[builder(into, default)]
        pub max_resource_percent_per_request: pulumi_gestalt_rust::InputOrOutput<
            Option<f64>,
        >,
        /// The workload group minimum percentage resource.
        #[builder(into)]
        pub min_resource_percent: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The workload group request minimum grant percentage.
        #[builder(into, default)]
        pub min_resource_percent_per_request: pulumi_gestalt_rust::InputOrOutput<
            Option<f64>,
        >,
        /// The name which should be used for this Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload group query execution timeout.
        #[builder(into, default)]
        pub query_execution_timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of the Synapse SQL Pool. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        #[builder(into)]
        pub sql_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadGroupResult {
        /// The workload group importance level. Defaults to `normal`.
        pub importance: pulumi_gestalt_rust::Output<Option<String>>,
        /// The workload group cap percentage resource.
        pub max_resource_percent: pulumi_gestalt_rust::Output<i32>,
        /// The workload group request maximum grant percentage. Defaults to `3`.
        pub max_resource_percent_per_request: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The workload group minimum percentage resource.
        pub min_resource_percent: pulumi_gestalt_rust::Output<i32>,
        /// The workload group request minimum grant percentage.
        pub min_resource_percent_per_request: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The name which should be used for this Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The workload group query execution timeout.
        pub query_execution_timeout_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Synapse SQL Pool. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        pub sql_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlPoolWorkloadGroupArgs,
    ) -> SqlPoolWorkloadGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let importance_binding = args.importance.get_output(context);
        let max_resource_percent_binding = args.max_resource_percent.get_output(context);
        let max_resource_percent_per_request_binding = args
            .max_resource_percent_per_request
            .get_output(context);
        let min_resource_percent_binding = args.min_resource_percent.get_output(context);
        let min_resource_percent_per_request_binding = args
            .min_resource_percent_per_request
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let query_execution_timeout_in_seconds_binding = args
            .query_execution_timeout_in_seconds
            .get_output(context);
        let sql_pool_id_binding = args.sql_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolWorkloadGroup:SqlPoolWorkloadGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importance".into(),
                    value: &importance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxResourcePercent".into(),
                    value: &max_resource_percent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxResourcePercentPerRequest".into(),
                    value: &max_resource_percent_per_request_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minResourcePercent".into(),
                    value: &min_resource_percent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minResourcePercentPerRequest".into(),
                    value: &min_resource_percent_per_request_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryExecutionTimeoutInSeconds".into(),
                    value: &query_execution_timeout_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlPoolId".into(),
                    value: &sql_pool_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlPoolWorkloadGroupResult {
            importance: o.get_field("importance"),
            max_resource_percent: o.get_field("maxResourcePercent"),
            max_resource_percent_per_request: o
                .get_field("maxResourcePercentPerRequest"),
            min_resource_percent: o.get_field("minResourcePercent"),
            min_resource_percent_per_request: o
                .get_field("minResourcePercentPerRequest"),
            name: o.get_field("name"),
            query_execution_timeout_in_seconds: o
                .get_field("queryExecutionTimeoutInSeconds"),
            sql_pool_id: o.get_field("sqlPoolId"),
        }
    }
}
