/// Manages a Managed Private Endpoint for a Kusto Cluster.
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
///       name: examplekc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleClusterManagedPrivateEndpoint:
///     type: azure:kusto:ClusterManagedPrivateEndpoint
///     name: example
///     properties:
///       name: examplempe
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       privateLinkResourceId: ${exampleAccount.id}
///       privateLinkResourceRegion: ${exampleAccount.location}
///       groupId: blob
///       requestMessage: Please Approve
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Managed Private Endpoint for a Kusto Cluster can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/clusterManagedPrivateEndpoint:ClusterManagedPrivateEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/managedPrivateEndpoints/managedPrivateEndpoint1
/// ```
///
pub mod cluster_managed_private_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterManagedPrivateEndpointArgs {
        /// The name of the Kusto Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The group id in which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Managed Private Endpoints to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARM resource ID of the resource for which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub private_link_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The region of the resource to which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_link_resource_region: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The user request message.
        #[builder(into, default)]
        pub request_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterManagedPrivateEndpointResult {
        /// The name of the Kusto Cluster. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The group id in which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub group_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Managed Private Endpoints to create. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARM resource ID of the resource for which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub private_link_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The region of the resource to which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub private_link_resource_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user request message.
        pub request_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterManagedPrivateEndpointArgs,
    ) -> ClusterManagedPrivateEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let group_id_binding = args.group_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_link_resource_id_binding = args
            .private_link_resource_id
            .get_output(context)
            .get_inner();
        let private_link_resource_region_binding = args
            .private_link_resource_region
            .get_output(context)
            .get_inner();
        let request_message_binding = args
            .request_message
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/clusterManagedPrivateEndpoint:ClusterManagedPrivateEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkResourceId".into(),
                    value: &private_link_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkResourceRegion".into(),
                    value: &private_link_resource_region_binding,
                },
                register_interface::ObjectField {
                    name: "requestMessage".into(),
                    value: &request_message_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterManagedPrivateEndpointResult {
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_link_resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkResourceId"),
            ),
            private_link_resource_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkResourceRegion"),
            ),
            request_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestMessage"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
