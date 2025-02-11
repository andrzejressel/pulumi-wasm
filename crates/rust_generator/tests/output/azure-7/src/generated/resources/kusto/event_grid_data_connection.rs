/// Manages a Kusto (also known as Azure Data Explorer) Event Grid Data Connection
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("storageaccountname")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("examplekustocluster")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ClusterSku::builder().capacity(2).name("Standard_D13_v2").build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleConsumerGroup = consumer_group::create(
///         "exampleConsumerGroup",
///         ConsumerGroupArgs::builder()
///             .eventhub_name("${exampleEventHub.name}")
///             .name("consumergroup-example")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .cluster_name("${exampleCluster.name}")
///             .hot_cache_period("P7D")
///             .location("${example.location}")
///             .name("example-kusto-database")
///             .resource_group_name("${example.name}")
///             .soft_delete_period("P31D")
///             .build_struct(),
///     );
///     let exampleEventGridDataConnection = event_grid_data_connection::create(
///         "exampleEventGridDataConnection",
///         EventGridDataConnectionArgs::builder()
///             .cluster_name("${exampleCluster.name}")
///             .data_format("JSON")
///             .database_name("${exampleDatabase.name}")
///             .eventhub_consumer_group_name("${exampleConsumerGroup.name}")
///             .eventhub_id("${exampleEventHub.id}")
///             .location("${example.location}")
///             .mapping_rule_name("my-table-mapping")
///             .name("my-kusto-eventgrid-data-connection")
///             .resource_group_name("${example.name}")
///             .storage_account_id("${exampleAccount.id}")
///             .table_name("my-table")
///             .build_struct(),
///     );
///     let exampleEventHub = event_hub::create(
///         "exampleEventHub",
///         EventHubArgs::builder()
///             .message_retention(1)
///             .name("eventhub-example")
///             .namespace_name("${exampleEventHubNamespace.name}")
///             .partition_count(1)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEventHubNamespace = event_hub_namespace::create(
///         "exampleEventHubNamespace",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("eventhubnamespace-example")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleEventSubscription = event_subscription::create(
///         "exampleEventSubscription",
///         EventSubscriptionArgs::builder()
///             .event_delivery_schema("EventGridSchema")
///             .eventhub_endpoint_id("${exampleEventHub.id}")
///             .included_event_types(
///                 vec!["Microsoft.Storage.BlobCreated", "Microsoft.Storage.BlobRenamed",],
///             )
///             .name("eventgrid-example")
///             .retry_policy(
///                 EventSubscriptionRetryPolicy::builder()
///                     .eventTimeToLive(144)
///                     .maxDeliveryAttempts(10)
///                     .build_struct(),
///             )
///             .scope("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_grid_data_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventGridDataConnectionArgs {
        /// Specifies the blob storage event type that needs to be processed. Possible Values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobRenamed`. Defaults to `Microsoft.Storage.BlobCreated`.
        #[builder(into, default)]
        pub blob_storage_event_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`.
        #[builder(into, default)]
        pub data_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        #[builder(into, default)]
        pub database_routing_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the event grid that is subscribed to the storage account events.
        #[builder(into, default)]
        pub eventgrid_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Event Hub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the resource id of the Event Hub this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Empty for non-managed identity based data connection. For system assigned identity, provide cluster resource Id. For user assigned identity (UAI) provide the UAI resource Id.
        #[builder(into, default)]
        pub managed_identity_resource_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto Event Grid Data Connection to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// is the first record of every file ignored? Defaults to `false`.
        #[builder(into, default)]
        pub skip_first_record: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the resource id of the Storage Account this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created.
        #[builder(into, default)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventGridDataConnectionResult {
        /// Specifies the blob storage event type that needs to be processed. Possible Values are `Microsoft.Storage.BlobCreated` and `Microsoft.Storage.BlobRenamed`. Defaults to `Microsoft.Storage.BlobCreated`.
        pub blob_storage_event_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`.
        pub data_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        pub database_routing_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the event grid that is subscribed to the storage account events.
        pub eventgrid_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Event Hub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the resource id of the Event Hub this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub eventhub_id: pulumi_gestalt_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Empty for non-managed identity based data connection. For system assigned identity, provide cluster resource Id. For user assigned identity (UAI) provide the UAI resource Id.
        pub managed_identity_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        pub mapping_rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Kusto Event Grid Data Connection to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// is the first record of every file ignored? Defaults to `false`.
        pub skip_first_record: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the resource id of the Storage Account this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created.
        pub table_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventGridDataConnectionArgs,
    ) -> EventGridDataConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blob_storage_event_type_binding = args
            .blob_storage_event_type
            .get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let data_format_binding = args.data_format.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let database_routing_type_binding = args
            .database_routing_type
            .get_output(context);
        let eventgrid_resource_id_binding = args
            .eventgrid_resource_id
            .get_output(context);
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context);
        let eventhub_id_binding = args.eventhub_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_identity_resource_id_binding = args
            .managed_identity_resource_id
            .get_output(context);
        let mapping_rule_name_binding = args.mapping_rule_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let skip_first_record_binding = args.skip_first_record.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/eventGridDataConnection:EventGridDataConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobStorageEventType".into(),
                    value: &blob_storage_event_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFormat".into(),
                    value: &data_format_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseRoutingType".into(),
                    value: &database_routing_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventgridResourceId".into(),
                    value: &eventgrid_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: &eventhub_consumer_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubId".into(),
                    value: &eventhub_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedIdentityResourceId".into(),
                    value: &managed_identity_resource_id_binding.drop_type(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFirstRecord".into(),
                    value: &skip_first_record_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventGridDataConnectionResult {
            blob_storage_event_type: o.get_field("blobStorageEventType"),
            cluster_name: o.get_field("clusterName"),
            data_format: o.get_field("dataFormat"),
            database_name: o.get_field("databaseName"),
            database_routing_type: o.get_field("databaseRoutingType"),
            eventgrid_resource_id: o.get_field("eventgridResourceId"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            eventhub_id: o.get_field("eventhubId"),
            location: o.get_field("location"),
            managed_identity_resource_id: o.get_field("managedIdentityResourceId"),
            mapping_rule_name: o.get_field("mappingRuleName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            skip_first_record: o.get_field("skipFirstRecord"),
            storage_account_id: o.get_field("storageAccountId"),
            table_name: o.get_field("tableName"),
        }
    }
}
