/// Manages a Kusto (also known as Azure Data Explorer) Event Grid Data Connection
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: eventhubnamespace-example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: eventhub-example
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 1
///       messageRetention: 1
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: consumergroup-example
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///   exampleEventSubscription:
///     type: azure:eventgrid:EventSubscription
///     name: example
///     properties:
///       name: eventgrid-example
///       scope: ${exampleAccount.id}
///       eventhubEndpointId: ${exampleEventHub.id}
///       eventDeliverySchema: EventGridSchema
///       includedEventTypes:
///         - Microsoft.Storage.BlobCreated
///         - Microsoft.Storage.BlobRenamed
///       retryPolicy:
///         eventTimeToLive: 144
///         maxDeliveryAttempts: 10
///   exampleEventGridDataConnection:
///     type: azure:kusto:EventGridDataConnection
///     name: example
///     properties:
///       name: my-kusto-eventgrid-data-connection
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///       databaseName: ${exampleDatabase.name}
///       storageAccountId: ${exampleAccount.id}
///       eventhubId: ${exampleEventHub.id}
///       eventhubConsumerGroupName: ${exampleConsumerGroup.name}
///       tableName: my-table
///       mappingRuleName: my-table-mapping
///       dataFormat: JSON
///     options:
///       dependsOn:
///         - ${exampleEventSubscription}
/// ```
///
/// ## Import
///
/// Kusto Event Grid Data Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/eventGridDataConnection:EventGridDataConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/dataConnections/dataConnection1
/// ```
///
pub mod event_grid_data_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventGridDataConnectionArgs {
        /// Specifies the blob storage event type that needs to be processed. Possible Values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobRenamed`. Defaults to `Microsoft.Storage.BlobCreated`.
        #[builder(into, default)]
        pub blob_storage_event_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`.
        #[builder(into, default)]
        pub data_format: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        #[builder(into, default)]
        pub database_routing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the event grid that is subscribed to the storage account events.
        #[builder(into, default)]
        pub eventgrid_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Event Hub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_consumer_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource id of the Event Hub this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_id: pulumi_wasm_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Empty for non-managed identity based data connection. For system assigned identity, provide cluster resource Id. For user assigned identity (UAI) provide the UAI resource Id.
        #[builder(into, default)]
        pub managed_identity_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Kusto Event Grid Data Connection to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// is the first record of every file ignored? Defaults to `false`.
        #[builder(into, default)]
        pub skip_first_record: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the resource id of the Storage Account this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created.
        #[builder(into, default)]
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventGridDataConnectionResult {
        /// Specifies the blob storage event type that needs to be processed. Possible Values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobRenamed`. Defaults to `Microsoft.Storage.BlobCreated`.
        pub blob_storage_event_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`.
        pub data_format: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        pub database_routing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the event grid that is subscribed to the storage account events.
        pub eventgrid_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Event Hub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub eventhub_consumer_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource id of the Event Hub this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub eventhub_id: pulumi_wasm_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Empty for non-managed identity based data connection. For system assigned identity, provide cluster resource Id. For user assigned identity (UAI) provide the UAI resource Id.
        pub managed_identity_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        pub mapping_rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Kusto Event Grid Data Connection to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// is the first record of every file ignored? Defaults to `false`.
        pub skip_first_record: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the resource id of the Storage Account this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created.
        pub table_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EventGridDataConnectionArgs,
    ) -> EventGridDataConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blob_storage_event_type_binding = args.blob_storage_event_type.get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let data_format_binding = args.data_format.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let database_routing_type_binding = args.database_routing_type.get_inner();
        let eventgrid_resource_id_binding = args.eventgrid_resource_id.get_inner();
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_inner();
        let eventhub_id_binding = args.eventhub_id.get_inner();
        let location_binding = args.location.get_inner();
        let managed_identity_resource_id_binding = args
            .managed_identity_resource_id
            .get_inner();
        let mapping_rule_name_binding = args.mapping_rule_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let skip_first_record_binding = args.skip_first_record.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/eventGridDataConnection:EventGridDataConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blobStorageEventType".into(),
                    value: &blob_storage_event_type_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
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
                    name: "eventgridResourceId".into(),
                    value: &eventgrid_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: &eventhub_consumer_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubId".into(),
                    value: &eventhub_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedIdentityResourceId".into(),
                    value: &managed_identity_resource_id_binding,
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
                    name: "skipFirstRecord".into(),
                    value: &skip_first_record_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blobStorageEventType".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
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
                    name: "eventgridResourceId".into(),
                },
                register_interface::ResultField {
                    name: "eventhubConsumerGroupName".into(),
                },
                register_interface::ResultField {
                    name: "eventhubId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedIdentityResourceId".into(),
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
                    name: "skipFirstRecord".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
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
        EventGridDataConnectionResult {
            blob_storage_event_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobStorageEventType").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
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
            eventgrid_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventgridResourceId").unwrap(),
            ),
            eventhub_consumer_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubConsumerGroupName").unwrap(),
            ),
            eventhub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_identity_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedIdentityResourceId").unwrap(),
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
            skip_first_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipFirstRecord").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
