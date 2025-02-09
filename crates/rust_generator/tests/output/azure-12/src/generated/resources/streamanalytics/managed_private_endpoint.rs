/// Manages a Stream Analytics Managed Private Endpoint.
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
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleCluster:
///     type: azure:streamanalytics:Cluster
///     name: example
///     properties:
///       name: examplestreamanalyticscluster
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       streamingCapacity: 36
///   exampleManagedPrivateEndpoint:
///     type: azure:streamanalytics:ManagedPrivateEndpoint
///     name: example
///     properties:
///       name: exampleprivateendpoint
///       resourceGroupName: ${example.name}
///       streamAnalyticsClusterName: ${exampleCluster.name}
///       targetResourceId: ${exampleAccount.id}
///       subresourceName: blob
/// ```
///
/// ## Import
///
/// Stream Analytics Private Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/managedPrivateEndpoint:ManagedPrivateEndpoint example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.StreamAnalytics/clusters/cluster1/privateEndpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_private_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointArgs {
        /// The name which should be used for this Stream Analytics Managed Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Managed Private Endpoint should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Cluster where the Managed Private Endpoint should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the sub resource name which the Stream Analytics Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subresource_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Private Link Enabled Remote Resource which this Stream Analytics Private endpoint should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointResult {
        /// The name which should be used for this Stream Analytics Managed Private Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Managed Private Endpoint should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Cluster where the Managed Private Endpoint should be created. Changing this forces a new resource to be created.
        pub stream_analytics_cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub resource name which the Stream Analytics Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        pub subresource_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Private Link Enabled Remote Resource which this Stream Analytics Private endpoint should be connected to. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagedPrivateEndpointArgs,
    ) -> ManagedPrivateEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let stream_analytics_cluster_name_binding_1 = args
            .stream_analytics_cluster_name
            .get_output(context);
        let stream_analytics_cluster_name_binding = stream_analytics_cluster_name_binding_1
            .get_inner();
        let subresource_name_binding_1 = args.subresource_name.get_output(context);
        let subresource_name_binding = subresource_name_binding_1.get_inner();
        let target_resource_id_binding_1 = args.target_resource_id.get_output(context);
        let target_resource_id_binding = target_resource_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/managedPrivateEndpoint:ManagedPrivateEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsClusterName".into(),
                    value: &stream_analytics_cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "subresourceName".into(),
                    value: &subresource_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedPrivateEndpointResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            stream_analytics_cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsClusterName"),
            ),
            subresource_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subresourceName"),
            ),
            target_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
        }
    }
}
