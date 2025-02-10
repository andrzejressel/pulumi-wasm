/// Manages a Kusto (also known as Azure Data Explorer) EventHub Data Connection
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .location("${example.location}")
///             .name("kustocluster")
///             .resource_group_name("${example.name}")
///             .sku(
///                 ClusterSku::builder().capacity(2).name("Standard_D13_v2").build_struct(),
///             )
///             .build_struct(),
///     );
///     let consumerGroup = consumer_group::create(
///         "consumerGroup",
///         ConsumerGroupArgs::builder()
///             .eventhub_name("${eventhub.name}")
///             .name("my-eventhub-consumergroup")
///             .namespace_name("${eventhubNs.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .cluster_name("${cluster.name}")
///             .hot_cache_period("P7D")
///             .location("${example.location}")
///             .name("my-kusto-database")
///             .resource_group_name("${example.name}")
///             .soft_delete_period("P31D")
///             .build_struct(),
///     );
///     let eventhub = event_hub::create(
///         "eventhub",
///         EventHubArgs::builder()
///             .message_retention(1)
///             .name("my-eventhub")
///             .namespace_name("${eventhubNs.name}")
///             .partition_count(1)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let eventhubConnection = eventhub_data_connection::create(
///         "eventhubConnection",
///         EventhubDataConnectionArgs::builder()
///             .cluster_name("${cluster.name}")
///             .consumer_group("${consumerGroup.name}")
///             .data_format("JSON")
///             .database_name("${database.name}")
///             .eventhub_id("${eventhub.id}")
///             .location("${example.location}")
///             .mapping_rule_name("my-table-mapping")
///             .name("my-kusto-eventhub-data-connection")
///             .resource_group_name("${example.name}")
///             .table_name("my-table")
///             .build_struct(),
///     );
///     let eventhubNs = event_hub_namespace::create(
///         "eventhubNs",
///         EventHubNamespaceArgs::builder()
///             .location("${example.location}")
///             .name("my-eventhub-ns")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("my-kusto-rg")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Kusto EventHub Data Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/eventhubDataConnection:EventhubDataConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/databases/database1/dataConnections/eventHubConnection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod eventhub_data_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventhubDataConnectionArgs {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies compression type for the connection. Allowed values: `GZip` and `None`. Defaults to `None`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub compression: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the EventHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub consumer_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSVE`, `TSV`, `TXT`, and `W3CLOGFILE`.
        #[builder(into, default)]
        pub data_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        #[builder(into, default)]
        pub database_routing_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of system properties for the Event Hub.
        #[builder(into, default)]
        pub event_system_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the resource id of the EventHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of a managed identity (system or user assigned) to be used to authenticate with event hub.
        #[builder(into, default)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto EventHub Data Connection to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created.
        #[builder(into, default)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventhubDataConnectionResult {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies compression type for the connection. Allowed values: `GZip` and `None`. Defaults to `None`. Changing this forces a new resource to be created.
        pub compression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the EventHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub consumer_group: pulumi_gestalt_rust::Output<String>,
        /// Specifies the data format of the EventHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSVE`, `TSV`, `TXT`, and `W3CLOGFILE`.
        pub data_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        pub database_routing_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of system properties for the Event Hub.
        pub event_system_properties: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the resource id of the EventHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub eventhub_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of a managed identity (system or user assigned) to be used to authenticate with event hub.
        pub identity_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created.
        pub mapping_rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Kusto EventHub Data Connection to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
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
        args: EventhubDataConnectionArgs,
    ) -> EventhubDataConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let compression_binding = args.compression.get_output(context);
        let consumer_group_binding = args.consumer_group.get_output(context);
        let data_format_binding = args.data_format.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let database_routing_type_binding = args
            .database_routing_type
            .get_output(context);
        let event_system_properties_binding = args
            .event_system_properties
            .get_output(context);
        let eventhub_id_binding = args.eventhub_id.get_output(context);
        let identity_id_binding = args.identity_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let mapping_rule_name_binding = args.mapping_rule_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/eventhubDataConnection:EventhubDataConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compression".into(),
                    value: compression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerGroup".into(),
                    value: consumer_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFormat".into(),
                    value: data_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseRoutingType".into(),
                    value: database_routing_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventSystemProperties".into(),
                    value: event_system_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubId".into(),
                    value: eventhub_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: identity_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mappingRuleName".into(),
                    value: mapping_rule_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: table_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventhubDataConnectionResult {
            cluster_name: o.get_field("clusterName"),
            compression: o.get_field("compression"),
            consumer_group: o.get_field("consumerGroup"),
            data_format: o.get_field("dataFormat"),
            database_name: o.get_field("databaseName"),
            database_routing_type: o.get_field("databaseRoutingType"),
            event_system_properties: o.get_field("eventSystemProperties"),
            eventhub_id: o.get_field("eventhubId"),
            identity_id: o.get_field("identityId"),
            location: o.get_field("location"),
            mapping_rule_name: o.get_field("mappingRuleName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            table_name: o.get_field("tableName"),
        }
    }
}
