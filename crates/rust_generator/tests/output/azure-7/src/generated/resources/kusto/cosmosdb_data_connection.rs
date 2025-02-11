/// Manages a Kusto / Cosmos Database Data Connection.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: exampleRG
///       location: West Europe
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleResourceGroup.id}
///       roleDefinitionName: ${builtin.name}
///       principalId: ${exampleCluster.identity.principalId}
///   exampleAccount:
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: example-ca
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       offerType: Standard
///       kind: GlobalDocumentDB
///       consistencyPolicy:
///         consistencyLevel: Session
///         maxIntervalInSeconds: 5
///         maxStalenessPrefix: 100
///       geoLocations:
///         - location: ${exampleResourceGroup.location}
///           failoverPriority: 0
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: examplecosmosdbsqldb
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: examplecosmosdbsqlcon
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPath: /part
///       throughput: 400
///   exampleSqlRoleAssignment:
///     type: azure:cosmosdb:SqlRoleAssignment
///     name: example
///     properties:
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountName: ${exampleAccount.name}
///       roleDefinitionId: ${example.id}
///       principalId: ${exampleCluster.identity.principalId}
///       scope: ${exampleAccount.id}
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: examplekc
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///       identity:
///         type: SystemAssigned
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: examplekd
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       clusterName: ${exampleCluster.name}
///   exampleScript:
///     type: azure:kusto:Script
///     name: example
///     properties:
///       name: create-table-script
///       databaseId: ${exampleDatabase.id}
///       scriptContent: |
///         .create table TestTable(Id:string, Name:string, _ts:long, _timestamp:datetime)
///         .create table TestTable ingestion json mapping "TestMapping"
///         '['
///         '    {"column":"Id","path":"$.id"},'
///         '    {"column":"Name","path":"$.name"},'
///         '    {"column":"_ts","path":"$._ts"},'
///         '    {"column":"_timestamp","path":"$._ts", "transform":"DateTimeFromUnixSeconds"}'
///         ']'
///         .alter table TestTable policy ingestionbatching "{'MaximumBatchingTimeSpan': '0:0:10', 'MaximumNumberOfItems': 10000}"
///   exampleCosmosdbDataConnection:
///     type: azure:kusto:CosmosdbDataConnection
///     name: example
///     properties:
///       name: examplekcdcd
///       location: ${exampleResourceGroup.location}
///       cosmosdbContainerId: ${exampleSqlContainer.id}
///       kustoDatabaseId: ${exampleDatabase.id}
///       managedIdentityId: ${exampleCluster.id}
///       tableName: TestTable
///       mappingRuleName: TestMapping
///       retrievalStartDate: 2023-06-26T12:00:00.6554616Z
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   builtin:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         roleDefinitionId: fbdf93bf-df7d-467e-a4d2-9458aa1360c8
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getSqlRoleDefinition
///       arguments:
///         roleDefinitionId: 00000000-0000-0000-0000-000000000001
///         resourceGroupName: ${exampleResourceGroup.name}
///         accountName: ${exampleAccount.name}
/// ```
///
/// ## Import
///
/// Kusto / Cosmos Database Data Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/cosmosdbDataConnection:CosmosdbDataConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/dataConnections/dataConnection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cosmosdb_data_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CosmosdbDataConnectionArgs {
        /// The name of an existing container in the Cosmos DB database. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into)]
        pub cosmosdb_container_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database in the Kusto cluster. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into)]
        pub kusto_database_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Data Explorer should exist. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of a managed system or user-assigned identity. The identity is used to authenticate with Cosmos DB. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into)]
        pub managed_identity_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of an existing mapping rule to use when ingesting the retrieved data. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the data connection. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If defined, the data connection retrieves Cosmos DB documents created or updated after the specified retrieval start date. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into, default)]
        pub retrieval_start_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The case-sensitive name of the existing target table in your cluster. Retrieved data is ingested into this table. Changing this forces a new Kusto Cosmos DB Connection to be created.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CosmosdbDataConnectionResult {
        /// The name of an existing container in the Cosmos DB database. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub cosmosdb_container_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the database in the Kusto cluster. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub kusto_database_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Data Explorer should exist. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of a managed system or user-assigned identity. The identity is used to authenticate with Cosmos DB. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub managed_identity_id: pulumi_gestalt_rust::Output<String>,
        /// The name of an existing mapping rule to use when ingesting the retrieved data. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub mapping_rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the data connection. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If defined, the data connection retrieves Cosmos DB documents created or updated after the specified retrieval start date. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub retrieval_start_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The case-sensitive name of the existing target table in your cluster. Retrieved data is ingested into this table. Changing this forces a new Kusto Cosmos DB Connection to be created.
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CosmosdbDataConnectionArgs,
    ) -> CosmosdbDataConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cosmosdb_container_id_binding = args
            .cosmosdb_container_id
            .get_output(context);
        let kusto_database_id_binding = args.kusto_database_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_identity_id_binding = args.managed_identity_id.get_output(context);
        let mapping_rule_name_binding = args.mapping_rule_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let retrieval_start_date_binding = args.retrieval_start_date.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/cosmosdbDataConnection:CosmosdbDataConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cosmosdbContainerId".into(),
                    value: &cosmosdb_container_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoDatabaseId".into(),
                    value: &kusto_database_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedIdentityId".into(),
                    value: &managed_identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mappingRuleName".into(),
                    value: &mapping_rule_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retrievalStartDate".into(),
                    value: &retrieval_start_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CosmosdbDataConnectionResult {
            cosmosdb_container_id: o.get_field("cosmosdbContainerId"),
            kusto_database_id: o.get_field("kustoDatabaseId"),
            location: o.get_field("location"),
            managed_identity_id: o.get_field("managedIdentityId"),
            mapping_rule_name: o.get_field("mappingRuleName"),
            name: o.get_field("name"),
            retrieval_start_date: o.get_field("retrievalStartDate"),
            table_name: o.get_field("tableName"),
        }
    }
}
