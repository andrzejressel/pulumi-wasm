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
pub mod sql_pool_workload_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadGroupArgs {
        /// The workload group importance level. Defaults to `normal`.
        #[builder(into, default)]
        pub importance: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The workload group cap percentage resource.
        #[builder(into)]
        pub max_resource_percent: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The workload group request maximum grant percentage. Defaults to `3`.
        #[builder(into, default)]
        pub max_resource_percent_per_request: pulumi_wasm_rust::InputOrOutput<
            Option<f64>,
        >,
        /// The workload group minimum percentage resource.
        #[builder(into)]
        pub min_resource_percent: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The workload group request minimum grant percentage.
        #[builder(into, default)]
        pub min_resource_percent_per_request: pulumi_wasm_rust::InputOrOutput<
            Option<f64>,
        >,
        /// The name which should be used for this Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The workload group query execution timeout.
        #[builder(into, default)]
        pub query_execution_timeout_in_seconds: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The ID of the Synapse SQL Pool. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        #[builder(into)]
        pub sql_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadGroupResult {
        /// The workload group importance level. Defaults to `normal`.
        pub importance: pulumi_wasm_rust::Output<Option<String>>,
        /// The workload group cap percentage resource.
        pub max_resource_percent: pulumi_wasm_rust::Output<i32>,
        /// The workload group request maximum grant percentage. Defaults to `3`.
        pub max_resource_percent_per_request: pulumi_wasm_rust::Output<Option<f64>>,
        /// The workload group minimum percentage resource.
        pub min_resource_percent: pulumi_wasm_rust::Output<i32>,
        /// The workload group request minimum grant percentage.
        pub min_resource_percent_per_request: pulumi_wasm_rust::Output<Option<f64>>,
        /// The name which should be used for this Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The workload group query execution timeout.
        pub query_execution_timeout_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Synapse SQL Pool. Changing this forces a new Synapse SQL Pool Workload Group to be created.
        pub sql_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SqlPoolWorkloadGroupArgs,
    ) -> SqlPoolWorkloadGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let importance_binding = args.importance.get_output(context).get_inner();
        let max_resource_percent_binding = args
            .max_resource_percent
            .get_output(context)
            .get_inner();
        let max_resource_percent_per_request_binding = args
            .max_resource_percent_per_request
            .get_output(context)
            .get_inner();
        let min_resource_percent_binding = args
            .min_resource_percent
            .get_output(context)
            .get_inner();
        let min_resource_percent_per_request_binding = args
            .min_resource_percent_per_request
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_execution_timeout_in_seconds_binding = args
            .query_execution_timeout_in_seconds
            .get_output(context)
            .get_inner();
        let sql_pool_id_binding = args.sql_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolWorkloadGroup:SqlPoolWorkloadGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "importance".into(),
                    value: &importance_binding,
                },
                register_interface::ObjectField {
                    name: "maxResourcePercent".into(),
                    value: &max_resource_percent_binding,
                },
                register_interface::ObjectField {
                    name: "maxResourcePercentPerRequest".into(),
                    value: &max_resource_percent_per_request_binding,
                },
                register_interface::ObjectField {
                    name: "minResourcePercent".into(),
                    value: &min_resource_percent_binding,
                },
                register_interface::ObjectField {
                    name: "minResourcePercentPerRequest".into(),
                    value: &min_resource_percent_per_request_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryExecutionTimeoutInSeconds".into(),
                    value: &query_execution_timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "sqlPoolId".into(),
                    value: &sql_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "importance".into(),
                },
                register_interface::ResultField {
                    name: "maxResourcePercent".into(),
                },
                register_interface::ResultField {
                    name: "maxResourcePercentPerRequest".into(),
                },
                register_interface::ResultField {
                    name: "minResourcePercent".into(),
                },
                register_interface::ResultField {
                    name: "minResourcePercentPerRequest".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queryExecutionTimeoutInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "sqlPoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SqlPoolWorkloadGroupResult {
            importance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importance").unwrap(),
            ),
            max_resource_percent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxResourcePercent").unwrap(),
            ),
            max_resource_percent_per_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxResourcePercentPerRequest").unwrap(),
            ),
            min_resource_percent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minResourcePercent").unwrap(),
            ),
            min_resource_percent_per_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minResourcePercentPerRequest").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_execution_timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryExecutionTimeoutInSeconds").unwrap(),
            ),
            sql_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlPoolId").unwrap(),
            ),
        }
    }
}
