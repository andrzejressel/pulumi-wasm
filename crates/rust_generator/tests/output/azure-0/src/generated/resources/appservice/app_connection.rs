/// Manages a service connector for function app.
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
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: example-cosmosdb-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       offerType: Standard
///       kind: GlobalDocumentDB
///       consistencyPolicy:
///         consistencyLevel: BoundedStaleness
///         maxIntervalInSeconds: 10
///         maxStalenessPrefix: 200
///       geoLocations:
///         - location: ${example.location}
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
///       name: example-container
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPath: /definition
///   exampleAccount2:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       location: ${example.location}
///       name: example-serviceplan
///       resourceGroupName: ${example.name}
///       skuName: P1v2
///       osType: Linux
///   test:
///     type: azure:appservice:FunctionApp
///     properties:
///       name: example-function-app
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       appServicePlanId: ${testAzurermAppServicePlan.id}
///       storageAccountName: ${testAzurermStorageAccount.name}
///       storageAccountAccessKey: ${testAzurermStorageAccount.primaryAccessKey}
///   exampleAppConnection:
///     type: azure:appservice:AppConnection
///     name: example
///     properties:
///       name: example-serviceconnector
///       functionAppId: ${exampleAzurermFunctionApp.id}
///       targetResourceId: ${testAzurermCosmosdbAccount.id}
///       authentication:
///         type: systemAssignedIdentity
/// ```
///
/// ## Import
///
/// Service Connector for app service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/appConnection:AppConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Web/sites/webapp/providers/Microsoft.ServiceLinker/linkers/serviceconnector1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppConnectionArgs {
        /// The authentication info. An `authentication` block as defined below.
        ///
        /// > **Note:** If a Managed Identity is used, this will need to be configured on the App Service.
        #[builder(into)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appservice::AppConnectionAuthentication,
        >,
        #[builder(into, default)]
        pub client_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the data source function app. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub secret_store: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::AppConnectionSecretStore>,
        >,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub vnet_solution: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppConnectionResult {
        /// The authentication info. An `authentication` block as defined below.
        ///
        /// > **Note:** If a Managed Identity is used, this will need to be configured on the App Service.
        pub authentication: pulumi_gestalt_rust::Output<
            super::super::types::appservice::AppConnectionAuthentication,
        >,
        pub client_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the data source function app. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub secret_store: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::AppConnectionSecretStore>,
        >,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
        pub vnet_solution: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppConnectionArgs,
    ) -> AppConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_binding = args.authentication.get_output(context);
        let client_type_binding = args.client_type.get_output(context);
        let function_app_id_binding = args.function_app_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let secret_store_binding = args.secret_store.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let vnet_solution_binding = args.vnet_solution.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/appConnection:AppConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: authentication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientType".into(),
                    value: client_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAppId".into(),
                    value: function_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretStore".into(),
                    value: secret_store_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: target_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vnetSolution".into(),
                    value: vnet_solution_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppConnectionResult {
            authentication: o.get_field("authentication"),
            client_type: o.get_field("clientType"),
            function_app_id: o.get_field("functionAppId"),
            name: o.get_field("name"),
            secret_store: o.get_field("secretStore"),
            target_resource_id: o.get_field("targetResourceId"),
            vnet_solution: o.get_field("vnetSolution"),
        }
    }
}
