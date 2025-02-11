/// Manages a Custom Location within an Extended Location.
///
/// > **Note:** Installing and configuring the Azure Arc Agent on your Kubernetes Cluster to establish connectivity is outside the scope of this document. For more details refer to [Deploy agents to your cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/conceptual-agent-overview#deploy-agents-to-your-cluster) and [Connect an existing Kubernetes Cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/quickstart-connect-cluster?tabs=azure-cli#connect-an-existing-kubernetes-cluster). If you encounter issues connecting your Kubernetes Cluster to Azure Arc, we'd recommend opening a ticket with Microsoft Support.
///
/// ## Import
///
/// Custom Locations can be imported using the resource id, e.g.
///
/// ```sh
/// $ pulumi import azure:extendedlocation/customLocation:CustomLocation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.ExtendedLocation/customLocations/example-custom-location
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomLocationArgs {
        /// An `authentication` block as defined below.
        #[builder(into, default)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::extendedlocation::CustomLocationAuthentication>,
        >,
        /// Specifies the list of Cluster Extension IDs.
        #[builder(into)]
        pub cluster_extension_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the display name of the Custom Location.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the host resource ID.
        #[builder(into)]
        pub host_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
        #[builder(into, default)]
        pub host_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
        #[builder(into)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomLocationResult {
        /// An `authentication` block as defined below.
        pub authentication: pulumi_gestalt_rust::Output<
            Option<super::super::types::extendedlocation::CustomLocationAuthentication>,
        >,
        /// Specifies the list of Cluster Extension IDs.
        pub cluster_extension_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the display name of the Custom Location.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the host resource ID.
        pub host_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
        pub host_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomLocationArgs,
    ) -> CustomLocationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_binding = args.authentication.get_output(context);
        let cluster_extension_ids_binding = args
            .cluster_extension_ids
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let host_resource_id_binding = args.host_resource_id.get_output(context);
        let host_type_binding = args.host_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:extendedlocation/customLocation:CustomLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterExtensionIds".into(),
                    value: &cluster_extension_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostResourceId".into(),
                    value: &host_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostType".into(),
                    value: &host_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomLocationResult {
            authentication: o.get_field("authentication"),
            cluster_extension_ids: o.get_field("clusterExtensionIds"),
            display_name: o.get_field("displayName"),
            host_resource_id: o.get_field("hostResourceId"),
            host_type: o.get_field("hostType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
