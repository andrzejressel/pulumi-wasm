/// Manages a service connector for app service.
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
///   exampleServicePlan:
///     type: azure:appservice:ServicePlan
///     name: example
///     properties:
///       location: ${example.location}
///       name: example-serviceplan
///       resourceGroupName: ${example.name}
///       skuName: P1v2
///       osType: Linux
///   exampleLinuxWebApp:
///     type: azure:appservice:LinuxWebApp
///     name: example
///     properties:
///       location: ${example.location}
///       name: example-linuxwebapp
///       resourceGroupName: ${example.name}
///       servicePlanId: ${exampleServicePlan.id}
///       siteConfig: {}
///   exampleConnection:
///     type: azure:appservice:Connection
///     name: example
///     properties:
///       name: example-serviceconnector
///       appServiceId: ${exampleLinuxWebApp.id}
///       targetResourceId: ${exampleSqlDatabase.id}
///       authentication:
///         type: systemAssignedIdentity
/// ```
///
/// ## Import
///
/// Service Connector for app service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/connection:Connection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Web/sites/webapp/providers/Microsoft.ServiceLinker/linkers/serviceconnector1
/// ```
///
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The ID of the data source web app. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_id: pulumi_wasm_rust::Output<String>,
        /// The authentication info. An `authentication` block as defined below.
        ///
        /// > **Note:** If a Managed Identity is used, this will need to be configured on the App Service.
        #[builder(into)]
        pub authentication: pulumi_wasm_rust::Output<
            super::super::types::appservice::ConnectionAuthentication,
        >,
        #[builder(into, default)]
        pub client_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub secret_store: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::ConnectionSecretStore>,
        >,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub vnet_solution: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The ID of the data source web app. Changing this forces a new resource to be created.
        pub app_service_id: pulumi_wasm_rust::Output<String>,
        /// The authentication info. An `authentication` block as defined below.
        ///
        /// > **Note:** If a Managed Identity is used, this will need to be configured on the App Service.
        pub authentication: pulumi_wasm_rust::Output<
            super::super::types::appservice::ConnectionAuthentication,
        >,
        pub client_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the service connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret_store: pulumi_wasm_rust::Output<
            Option<super::super::types::appservice::ConnectionSecretStore>,
        >,
        /// The ID of the target resource. Changing this forces a new resource to be created. Possible target resources are `Postgres`, `PostgresFlexible`, `Mysql`, `Sql`, `Redis`, `RedisEnterprise`, `CosmosCassandra`, `CosmosGremlin`, `CosmosMongo`, `CosmosSql`, `CosmosTable`, `StorageBlob`, `StorageQueue`, `StorageFile`, `StorageTable`, `AppConfig`, `EventHub`, `ServiceBus`, `SignalR`, `WebPubSub`, `ConfluentKafka`. The integration guide can be found [here](https://learn.microsoft.com/en-us/azure/service-connector/how-to-integrate-postgres).
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        pub vnet_solution: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_id_binding = args.app_service_id.get_inner();
        let authentication_binding = args.authentication.get_inner();
        let client_type_binding = args.client_type.get_inner();
        let name_binding = args.name.get_inner();
        let secret_store_binding = args.secret_store.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let vnet_solution_binding = args.vnet_solution.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/connection:Connection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceId".into(),
                    value: &app_service_id_binding,
                },
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
                    name: "appServiceId".into(),
                },
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
        ConnectionResult {
            app_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceId").unwrap(),
            ),
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
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
            vnet_solution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetSolution").unwrap(),
            ),
        }
    }
}