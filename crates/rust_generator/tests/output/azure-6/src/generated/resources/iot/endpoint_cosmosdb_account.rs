/// Manages an IotHub Cosmos DB Account Endpoint
///
/// > **NOTE:** Endpoints can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azurerm_iothub_endpoint_*` resources - but the two ways of defining the endpoints cannot be used together. If both are used against the same IoTHub, spurious changes will occur. Also, defining a `azurerm_iothub_endpoint_*` resource and another endpoint of a different type directly on the `azure.iot.IoTHub` resource is not supported.
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
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: exampleIothub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: B1
///         capacity: '1'
///       tags:
///         purpose: example
///   exampleAccount:
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: cosmosdb-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       offerType: Standard
///       kind: GlobalDocumentDB
///       consistencyPolicy:
///         consistencyLevel: Strong
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
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: example-container
///       resourceGroupName: ${exampleAccount.resourceGroupName}
///       accountName: ${exampleAccount.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPath: /definition/id
///   exampleEndpointCosmosdbAccount:
///     type: azure:iot:EndpointCosmosdbAccount
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iothubId: ${exampleIoTHub.id}
///       containerName: ${exampleSqlContainer.name}
///       databaseName: ${exampleSqlDatabase.name}
///       endpointUri: ${exampleAccount.endpoint}
///       primaryKey: ${exampleAccount.primaryKey}
///       secondaryKey: ${exampleAccount.secondaryKey}
/// ```
///
/// ## Import
///
/// IoTHub Cosmos DB Account Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/endpointCosmosdbAccount:EndpointCosmosdbAccount endpoint1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/endpoints/cosmosDBAccountEndpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_cosmosdb_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointCosmosdbAccountArgs {
        /// The type used to authenticate against the Cosmos DB Account endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        #[builder(into, default)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cosmos DB Container in the Cosmos DB Database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Cosmos DB Database in the Cosmos DB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI of the Cosmos DB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub endpoint_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the User Managed Identity used to authenticate against the Cosmos DB Account endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        #[builder(into, default)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the IoT Hub to create the endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the partition key associated with the Cosmos DB Container.
        #[builder(into, default)]
        pub partition_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The template for generating a synthetic partition key value for use within the Cosmos DB Container.
        #[builder(into, default)]
        pub partition_key_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The primary key of the Cosmos DB Account.
        ///
        /// > **NOTE:** `primary_key` must and can only be specified when `authentication_type` is `keyBased`.
        #[builder(into, default)]
        pub primary_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Cosmos DB Account has been created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The secondary key of the Cosmos DB Account.
        ///
        /// > **NOTE:** `secondary_key` must and can only be specified when `authentication_type` is `keyBased`.
        #[builder(into, default)]
        pub secondary_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EndpointCosmosdbAccountResult {
        /// The type used to authenticate against the Cosmos DB Account endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        pub authentication_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Cosmos DB Container in the Cosmos DB Database. Changing this forces a new resource to be created.
        pub container_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cosmos DB Database in the Cosmos DB Account. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The URI of the Cosmos DB Account. Changing this forces a new resource to be created.
        pub endpoint_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of the User Managed Identity used to authenticate against the Cosmos DB Account endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        pub identity_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the IoT Hub to create the endpoint. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the partition key associated with the Cosmos DB Container.
        pub partition_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The template for generating a synthetic partition key value for use within the Cosmos DB Container.
        pub partition_key_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// The primary key of the Cosmos DB Account.
        ///
        /// > **NOTE:** `primary_key` must and can only be specified when `authentication_type` is `keyBased`.
        pub primary_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group under which the Cosmos DB Account has been created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary key of the Cosmos DB Account.
        ///
        /// > **NOTE:** `secondary_key` must and can only be specified when `authentication_type` is `keyBased`.
        pub secondary_key: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointCosmosdbAccountArgs,
    ) -> EndpointCosmosdbAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let container_name_binding = args.container_name.get_output(context).get_inner();
        let database_name_binding = args.database_name.get_output(context).get_inner();
        let endpoint_uri_binding = args.endpoint_uri.get_output(context).get_inner();
        let identity_id_binding = args.identity_id.get_output(context).get_inner();
        let iothub_id_binding = args.iothub_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partition_key_name_binding = args
            .partition_key_name
            .get_output(context)
            .get_inner();
        let partition_key_template_binding = args
            .partition_key_template
            .get_output(context)
            .get_inner();
        let primary_key_binding = args.primary_key.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let secondary_key_binding = args.secondary_key.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/endpointCosmosdbAccount:EndpointCosmosdbAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "endpointUri".into(),
                    value: &endpoint_uri_binding,
                },
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyName".into(),
                    value: &partition_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKeyTemplate".into(),
                    value: &partition_key_template_binding,
                },
                register_interface::ObjectField {
                    name: "primaryKey".into(),
                    value: &primary_key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryKey".into(),
                    value: &secondary_key_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointCosmosdbAccountResult {
            authentication_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationType"),
            ),
            container_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerName"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            endpoint_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointUri"),
            ),
            identity_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityId"),
            ),
            iothub_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            partition_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKeyName"),
            ),
            partition_key_template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKeyTemplate"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
        }
    }
}
