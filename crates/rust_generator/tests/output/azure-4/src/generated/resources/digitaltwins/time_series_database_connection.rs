/// Manages a Digital Twins Time Series Database Connection.
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
///   exampleInstance:
///     type: azure:digitaltwins:Instance
///     name: example
///     properties:
///       name: example-DT
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: exampleEventHubNamespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: exampleEventHub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 2
///       messageRetention: 7
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: example-consumergroup
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: examplekc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: example-kusto-database
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///   databaseContributor:
///     type: azure:authorization:Assignment
///     name: database_contributor
///     properties:
///       scope: ${exampleDatabase.id}
///       principalId: ${exampleInstance.identity.principalId}
///       roleDefinitionName: Contributor
///   eventhubDataOwner:
///     type: azure:authorization:Assignment
///     name: eventhub_data_owner
///     properties:
///       scope: ${exampleEventHub.id}
///       principalId: ${exampleInstance.identity.principalId}
///       roleDefinitionName: Azure Event Hubs Data Owner
///   exampleDatabasePrincipalAssignment:
///     type: azure:kusto:DatabasePrincipalAssignment
///     name: example
///     properties:
///       name: dataadmin
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       databaseName: ${exampleDatabase.name}
///       tenantId: ${exampleInstance.identity.tenantId}
///       principalId: ${exampleInstance.identity.principalId}
///       principalType: App
///       role: Admin
///   exampleTimeSeriesDatabaseConnection:
///     type: azure:digitaltwins:TimeSeriesDatabaseConnection
///     name: example
///     properties:
///       name: example-connection
///       digitalTwinsId: ${exampleInstance.id}
///       eventhubName: ${exampleEventHub.name}
///       eventhubNamespaceId: ${exampleEventHubNamespace.id}
///       eventhubNamespaceEndpointUri: sb://${exampleEventHubNamespace.name}.servicebus.windows.net
///       eventhubConsumerGroupName: ${exampleConsumerGroup.name}
///       kustoClusterId: ${exampleCluster.id}
///       kustoClusterUri: ${exampleCluster.uri}
///       kustoDatabaseName: ${exampleDatabase.name}
///       kustoTableName: exampleTable
///     options:
///       dependsOn:
///         - ${databaseContributor}
///         - ${eventhubDataOwner}
///         - ${exampleDatabasePrincipalAssignment}
/// ```
///
/// ## Import
///
/// Digital Twins Time Series Database Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:digitaltwins/timeSeriesDatabaseConnection:TimeSeriesDatabaseConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DigitalTwins/digitalTwinsInstances/dt1/timeSeriesDatabaseConnections/connection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod time_series_database_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TimeSeriesDatabaseConnectionArgs {
        /// The ID of the Digital Twins. Changing this forces a new resource to be created.
        #[builder(into)]
        pub digital_twins_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Event Hub Consumer Group. Changing this forces a new resource to be created. Defaults to `$Default`.
        #[builder(into, default)]
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the Event Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URI of the Event Hub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_namespace_endpoint_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Event Hub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Kusto Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kusto_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URI of the Kusto Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kusto_cluster_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Kusto Database. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kusto_database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Kusto Table. Defaults to `AdtPropertyEvents`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub kusto_table_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Digital Twins Time Series Database Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TimeSeriesDatabaseConnectionResult {
        /// The ID of the Digital Twins. Changing this forces a new resource to be created.
        pub digital_twins_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Event Hub Consumer Group. Changing this forces a new resource to be created. Defaults to `$Default`.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the Event Hub. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// URI of the Event Hub Namespace. Changing this forces a new resource to be created.
        pub eventhub_namespace_endpoint_uri: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Event Hub Namespace. Changing this forces a new resource to be created.
        pub eventhub_namespace_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Kusto Cluster. Changing this forces a new resource to be created.
        pub kusto_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// URI of the Kusto Cluster. Changing this forces a new resource to be created.
        pub kusto_cluster_uri: pulumi_gestalt_rust::Output<String>,
        /// Name of the Kusto Database. Changing this forces a new resource to be created.
        pub kusto_database_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the Kusto Table. Defaults to `AdtPropertyEvents`. Changing this forces a new resource to be created.
        pub kusto_table_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Digital Twins Time Series Database Connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TimeSeriesDatabaseConnectionArgs,
    ) -> TimeSeriesDatabaseConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let digital_twins_id_binding = args.digital_twins_id.get_output(context);
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context);
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let eventhub_namespace_endpoint_uri_binding = args
            .eventhub_namespace_endpoint_uri
            .get_output(context);
        let eventhub_namespace_id_binding = args
            .eventhub_namespace_id
            .get_output(context);
        let kusto_cluster_id_binding = args.kusto_cluster_id.get_output(context);
        let kusto_cluster_uri_binding = args.kusto_cluster_uri.get_output(context);
        let kusto_database_name_binding = args.kusto_database_name.get_output(context);
        let kusto_table_name_binding = args.kusto_table_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:digitaltwins/timeSeriesDatabaseConnection:TimeSeriesDatabaseConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "digitalTwinsId".into(),
                    value: &digital_twins_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: &eventhub_consumer_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubNamespaceEndpointUri".into(),
                    value: &eventhub_namespace_endpoint_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubNamespaceId".into(),
                    value: &eventhub_namespace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoClusterId".into(),
                    value: &kusto_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoClusterUri".into(),
                    value: &kusto_cluster_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoDatabaseName".into(),
                    value: &kusto_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kustoTableName".into(),
                    value: &kusto_table_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TimeSeriesDatabaseConnectionResult {
            digital_twins_id: o.get_field("digitalTwinsId"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            eventhub_name: o.get_field("eventhubName"),
            eventhub_namespace_endpoint_uri: o.get_field("eventhubNamespaceEndpointUri"),
            eventhub_namespace_id: o.get_field("eventhubNamespaceId"),
            kusto_cluster_id: o.get_field("kustoClusterId"),
            kusto_cluster_uri: o.get_field("kustoClusterUri"),
            kusto_database_name: o.get_field("kustoDatabaseName"),
            kusto_table_name: o.get_field("kustoTableName"),
            name: o.get_field("name"),
        }
    }
}
