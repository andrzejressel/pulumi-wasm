/// Manages a Stream Analytics Output to CosmosDB.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleAccount:
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: exampledb
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       offerType: Standard
///       kind: GlobalDocumentDB
///       consistencyPolicy:
///         consistencyLevel: BoundedStaleness
///         maxIntervalInSeconds: 10
///         maxStalenessPrefix: 200
///       geoLocations:
///         - location: ${exampleResourceGroup.location}
///           failoverPriority: 0
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: cosmos-sql-db
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///       throughput: 400
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: examplecontainer
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPath: foo
///   exampleOutputCosmosdb:
///     type: azure:streamanalytics:OutputCosmosdb
///     name: example
///     properties:
///       name: output-to-cosmosdb
///       streamAnalyticsJobId: ${example.id}
///       cosmosdbAccountKey: ${exampleAccount.primaryKey}
///       cosmosdbSqlDatabaseId: ${exampleSqlDatabase.id}
///       containerName: ${exampleSqlContainer.name}
///       documentId: exampledocumentid
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Outputs for CosmosDB can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputCosmosdb:OutputCosmosdb example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_cosmosdb {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputCosmosdbArgs {
        /// The name of the CosmosDB container.
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The account key for the CosmosDB database.
        #[builder(into)]
        pub cosmosdb_account_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the CosmosDB database.
        #[builder(into)]
        pub cosmosdb_sql_database_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the field in output events used to specify the primary key which insert or update operations are based on.
        #[builder(into, default)]
        pub document_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Stream Analytics Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the field in output events used to specify the key for partitioning output across collections. If `container_name` contains `{partition}` token, this property is required to be specified.
        #[builder(into, default)]
        pub partition_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputCosmosdbResult {
        /// The name of the CosmosDB container.
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The account key for the CosmosDB database.
        pub cosmosdb_account_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the CosmosDB database.
        pub cosmosdb_sql_database_id: pulumi_wasm_rust::Output<String>,
        /// The name of the field in output events used to specify the primary key which insert or update operations are based on.
        pub document_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the field in output events used to specify the key for partitioning output across collections. If `container_name` contains `{partition}` token, this property is required to be specified.
        pub partition_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OutputCosmosdbArgs,
    ) -> OutputCosmosdbResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_name_binding = args.container_name.get_output(context).get_inner();
        let cosmosdb_account_key_binding = args
            .cosmosdb_account_key
            .get_output(context)
            .get_inner();
        let cosmosdb_sql_database_id_binding = args
            .cosmosdb_sql_database_id
            .get_output(context)
            .get_inner();
        let document_id_binding = args.document_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partition_key_binding = args.partition_key.get_output(context).get_inner();
        let stream_analytics_job_id_binding = args
            .stream_analytics_job_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputCosmosdb:OutputCosmosdb".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "cosmosdbAccountKey".into(),
                    value: &cosmosdb_account_key_binding,
                },
                register_interface::ObjectField {
                    name: "cosmosdbSqlDatabaseId".into(),
                    value: &cosmosdb_sql_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "documentId".into(),
                    value: &document_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobId".into(),
                    value: &stream_analytics_job_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OutputCosmosdbResult {
            container_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerName"),
            ),
            cosmosdb_account_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cosmosdbAccountKey"),
            ),
            cosmosdb_sql_database_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cosmosdbSqlDatabaseId"),
            ),
            document_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("documentId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            partition_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("partitionKey"),
            ),
            stream_analytics_job_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobId"),
            ),
        }
    }
}
