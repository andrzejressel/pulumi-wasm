/// Manages a Kusto (also known as Azure Data Explorer) IotHub Data Connection
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
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: examplekustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: example-kusto-database
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///       hotCachePeriod: P7D
///       softDeletePeriod: P31D
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: exampleIoTHub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: B1
///         capacity: '1'
///   exampleSharedAccessPolicy:
///     type: azure:iot:SharedAccessPolicy
///     name: example
///     properties:
///       name: example-shared-access-policy
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       registryRead: true
///   exampleConsumerGroup:
///     type: azure:iot:ConsumerGroup
///     name: example
///     properties:
///       name: example-consumer-group
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       eventhubEndpointName: events
///   exampleIotHubDataConnection:
///     type: azure:kusto:IotHubDataConnection
///     name: example
///     properties:
///       name: my-kusto-iothub-data-connection
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///       databaseName: ${exampleDatabase.name}
///       iothubId: ${exampleIoTHub.id}
///       consumerGroup: ${exampleConsumerGroup.name}
///       sharedAccessPolicyName: ${exampleSharedAccessPolicy.name}
///       eventSystemProperties:
///         - message-id
///         - sequence-number
///         - to
///       tableName: my-table
///       mappingRuleName: my-table-mapping
///       dataFormat: JSON
/// ```
///
/// ## Import
///
/// Kusto IotHub Data Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/iotHubDataConnection:IotHubDataConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/dataConnections/dataConnection1
/// ```
///
pub mod iot_hub_data_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDataConnectionArgs {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the IotHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub consumer_group: pulumi_wasm_rust::Output<String>,
        /// Specifies the data format of the IoTHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_format: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        #[builder(into, default)]
        pub database_routing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the System Properties that each IoT Hub message should contain. Changing this forces a new resource to be created. Possible values are `message-id`, `sequence-number`, `to`, `absolute-expiry-time`, `iothub-enqueuedtime`, `correlation-id`, `user-id`, `iothub-ack`, `iothub-connection-device-id`, `iothub-connection-auth-generation-id` and `iothub-connection-auth-method`.
        #[builder(into, default)]
        pub event_system_properties: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource id of the IotHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Kusto IotHub Data Connection to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the IotHub Shared Access Policy this data connection will use for ingestion, which must have read permission. Changing this forces a new resource to be created.
        #[builder(into)]
        pub shared_access_policy_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IotHubDataConnectionResult {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the IotHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub consumer_group: pulumi_wasm_rust::Output<String>,
        /// Specifies the data format of the IoTHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`. Changing this forces a new resource to be created.
        pub data_format: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        pub database_routing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the System Properties that each IoT Hub message should contain. Changing this forces a new resource to be created. Possible values are `message-id`, `sequence-number`, `to`, `absolute-expiry-time`, `iothub-enqueuedtime`, `correlation-id`, `user-id`, `iothub-ack`, `iothub-connection-device-id`, `iothub-connection-auth-generation-id` and `iothub-connection-auth-method`.
        pub event_system_properties: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource id of the IotHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created. Changing this forces a new resource to be created.
        pub mapping_rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Kusto IotHub Data Connection to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the IotHub Shared Access Policy this data connection will use for ingestion, which must have read permission. Changing this forces a new resource to be created.
        pub shared_access_policy_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created. Changing this forces a new resource to be created.
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IotHubDataConnectionArgs,
    ) -> IotHubDataConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let consumer_group_binding = args.consumer_group.get_inner();
        let data_format_binding = args.data_format.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let database_routing_type_binding = args.database_routing_type.get_inner();
        let event_system_properties_binding = args.event_system_properties.get_inner();
        let iothub_id_binding = args.iothub_id.get_inner();
        let location_binding = args.location.get_inner();
        let mapping_rule_name_binding = args.mapping_rule_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let shared_access_policy_name_binding = args
            .shared_access_policy_name
            .get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/iotHubDataConnection:IotHubDataConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "consumerGroup".into(),
                    value: &consumer_group_binding,
                },
                register_interface::ObjectField {
                    name: "dataFormat".into(),
                    value: &data_format_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "databaseRoutingType".into(),
                    value: &database_routing_type_binding,
                },
                register_interface::ObjectField {
                    name: "eventSystemProperties".into(),
                    value: &event_system_properties_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mappingRuleName".into(),
                    value: &mapping_rule_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessPolicyName".into(),
                    value: &shared_access_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "consumerGroup".into(),
                },
                register_interface::ResultField {
                    name: "dataFormat".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "databaseRoutingType".into(),
                },
                register_interface::ResultField {
                    name: "eventSystemProperties".into(),
                },
                register_interface::ResultField {
                    name: "iothubId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mappingRuleName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IotHubDataConnectionResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            consumer_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerGroup").unwrap(),
            ),
            data_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFormat").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            database_routing_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseRoutingType").unwrap(),
            ),
            event_system_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSystemProperties").unwrap(),
            ),
            iothub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mapping_rule_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mappingRuleName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            shared_access_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessPolicyName").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}