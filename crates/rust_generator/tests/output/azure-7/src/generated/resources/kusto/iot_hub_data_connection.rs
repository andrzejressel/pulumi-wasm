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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iot_hub_data_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDataConnectionArgs {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the IotHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub consumer_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the data format of the IoTHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub data_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        #[builder(into, default)]
        pub database_routing_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the System Properties that each IoT Hub message should contain. Changing this forces a new resource to be created. Possible values are `message-id`, `sequence-number`, `to`, `absolute-expiry-time`, `iothub-enqueuedtime`, `correlation-id`, `user-id`, `iothub-ack`, `iothub-connection-device-id`, `iothub-connection-auth-generation-id` and `iothub-connection-auth-method`.
        #[builder(into, default)]
        pub event_system_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the resource id of the IotHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub mapping_rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto IotHub Data Connection to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the IotHub Shared Access Policy this data connection will use for ingestion, which must have read permission. Changing this forces a new resource to be created.
        #[builder(into)]
        pub shared_access_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IotHubDataConnectionResult {
        /// Specifies the name of the Kusto Cluster this data connection will be added to. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IotHub consumer group this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub consumer_group: pulumi_gestalt_rust::Output<String>,
        /// Specifies the data format of the IoTHub messages. Allowed values: `APACHEAVRO`, `AVRO`, `CSV`, `JSON`, `MULTIJSON`, `ORC`, `PARQUET`, `PSV`, `RAW`, `SCSV`, `SINGLEJSON`, `SOHSV`, `TSV`, `TSVE`, `TXT` and `W3CLOGFILE`. Changing this forces a new resource to be created.
        pub data_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Kusto Database this data connection will be added to. Changing this forces a new resource to be created.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Indication for database routing information from the data connection, by default only database routing information is allowed. Allowed values: `Single`, `Multi`. Changing this forces a new resource to be created. Defaults to `Single`.
        pub database_routing_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the System Properties that each IoT Hub message should contain. Changing this forces a new resource to be created. Possible values are `message-id`, `sequence-number`, `to`, `absolute-expiry-time`, `iothub-enqueuedtime`, `correlation-id`, `user-id`, `iothub-ack`, `iothub-connection-device-id`, `iothub-connection-auth-generation-id` and `iothub-connection-auth-method`.
        pub event_system_properties: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the resource id of the IotHub this data connection will use for ingestion. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// The location where the Kusto Database should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the mapping rule used for the message ingestion. Mapping rule must exist before resource is created. Changing this forces a new resource to be created.
        pub mapping_rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Kusto IotHub Data Connection to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Resource Group where the Kusto Database should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IotHub Shared Access Policy this data connection will use for ingestion, which must have read permission. Changing this forces a new resource to be created.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the target table name used for the message ingestion. Table must exist before resource is created. Changing this forces a new resource to be created.
        pub table_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IotHubDataConnectionArgs,
    ) -> IotHubDataConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding_1 = args.cluster_name.get_output(context);
        let cluster_name_binding = cluster_name_binding_1.get_inner();
        let consumer_group_binding_1 = args.consumer_group.get_output(context);
        let consumer_group_binding = consumer_group_binding_1.get_inner();
        let data_format_binding_1 = args.data_format.get_output(context);
        let data_format_binding = data_format_binding_1.get_inner();
        let database_name_binding_1 = args.database_name.get_output(context);
        let database_name_binding = database_name_binding_1.get_inner();
        let database_routing_type_binding_1 = args
            .database_routing_type
            .get_output(context);
        let database_routing_type_binding = database_routing_type_binding_1.get_inner();
        let event_system_properties_binding_1 = args
            .event_system_properties
            .get_output(context);
        let event_system_properties_binding = event_system_properties_binding_1
            .get_inner();
        let iothub_id_binding_1 = args.iothub_id.get_output(context);
        let iothub_id_binding = iothub_id_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let mapping_rule_name_binding_1 = args.mapping_rule_name.get_output(context);
        let mapping_rule_name_binding = mapping_rule_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let shared_access_policy_name_binding_1 = args
            .shared_access_policy_name
            .get_output(context);
        let shared_access_policy_name_binding = shared_access_policy_name_binding_1
            .get_inner();
        let table_name_binding_1 = args.table_name.get_output(context);
        let table_name_binding = table_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/iotHubDataConnection:IotHubDataConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        IotHubDataConnectionResult {
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            consumer_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("consumerGroup"),
            ),
            data_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFormat"),
            ),
            database_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseName"),
            ),
            database_routing_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseRoutingType"),
            ),
            event_system_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventSystemProperties"),
            ),
            iothub_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mapping_rule_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mappingRuleName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            shared_access_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedAccessPolicyName"),
            ),
            table_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableName"),
            ),
        }
    }
}
