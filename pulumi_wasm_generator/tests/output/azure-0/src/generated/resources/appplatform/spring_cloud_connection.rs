/// Manages a service connector for spring cloud app.
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
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: examplespringcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: examplespringcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///       identity:
///         type: SystemAssigned
///   exampleSpringCloudJavaDeployment:
///     type: azure:appplatform:SpringCloudJavaDeployment
///     name: example
///     properties:
///       name: exampledeployment
///       springCloudAppId: ${exampleSpringCloudApp.id}
///   exampleSpringCloudConnection:
///     type: azure:appplatform:SpringCloudConnection
///     name: example
///     properties:
///       name: example-serviceconnector
///       springCloudId: ${exampleSpringCloudJavaDeployment.id}
///       targetResourceId: ${exampleSqlDatabase.id}
///       authentication:
///         type: systemAssignedIdentity
/// ```
///
/// ## Import
///
/// Service Connector for spring cloud can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudConnection:SpringCloudConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AppPlatform/Spring/springcloud/apps/springcloudapp/deployments/deployment/providers/Microsoft.ServiceLinker/linkers/serviceconnector1
/// ```
///
pub mod spring_cloud_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudConnectionArgs {
        /// The authentication info. An `authentication` block as defined below.
        #[builder(into)]
        pub authentication: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudConnectionAuthentication,
        >,
        #[builder(into, default)]
        pub client_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub secret_store: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudConnectionSecretStore>,
        >,
        /// The ID of the data source spring cloud. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub vnet_solution: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudConnectionResult {
        /// The authentication info. An `authentication` block as defined below.
        pub authentication: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudConnectionAuthentication,
        >,
        pub client_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret_store: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudConnectionSecretStore>,
        >,
        /// The ID of the data source spring cloud. Changing this forces a new resource to be created.
        pub spring_cloud_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        pub vnet_solution: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudConnectionArgs,
    ) -> SpringCloudConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_binding = args.authentication.get_inner();
        let client_type_binding = args.client_type.get_inner();
        let name_binding = args.name.get_inner();
        let secret_store_binding = args.secret_store.get_inner();
        let spring_cloud_id_binding = args.spring_cloud_id.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let vnet_solution_binding = args.vnet_solution.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudConnection:SpringCloudConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "clientType".into(),
                    value: &client_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "secretStore".into(),
                    value: &secret_store_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudId".into(),
                    value: &spring_cloud_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "vnetSolution".into(),
                    value: &vnet_solution_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authentication".into(),
                },
                register_interface::ResultField {
                    name: "clientType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "secretStore".into(),
                },
                register_interface::ResultField {
                    name: "springCloudId".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
                register_interface::ResultField {
                    name: "vnetSolution".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudConnectionResult {
            authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authentication").unwrap(),
            ),
            client_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            secret_store: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretStore").unwrap(),
            ),
            spring_cloud_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudId").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
            vnet_solution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetSolution").unwrap(),
            ),
        }
    }
}
