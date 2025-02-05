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
pub mod custom_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomLocationArgs {
        /// An `authentication` block as defined below.
        #[builder(into, default)]
        pub authentication: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::extendedlocation::CustomLocationAuthentication>,
        >,
        /// Specifies the list of Cluster Extension IDs.
        #[builder(into)]
        pub cluster_extension_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Specifies the display name of the Custom Location.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the host resource ID.
        #[builder(into)]
        pub host_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
        #[builder(into, default)]
        pub host_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
        #[builder(into)]
        pub namespace: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomLocationResult {
        /// An `authentication` block as defined below.
        pub authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::extendedlocation::CustomLocationAuthentication>,
        >,
        /// Specifies the list of Cluster Extension IDs.
        pub cluster_extension_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the display name of the Custom Location.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the host resource ID.
        pub host_resource_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
        pub host_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomLocationArgs,
    ) -> CustomLocationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_binding = args.authentication.get_output(context).get_inner();
        let cluster_extension_ids_binding = args
            .cluster_extension_ids
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let host_resource_id_binding = args
            .host_resource_id
            .get_output(context)
            .get_inner();
        let host_type_binding = args.host_type.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:extendedlocation/customLocation:CustomLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "clusterExtensionIds".into(),
                    value: &cluster_extension_ids_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "hostResourceId".into(),
                    value: &host_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostType".into(),
                    value: &host_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomLocationResult {
            authentication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            cluster_extension_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterExtensionIds"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            host_resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostResourceId"),
            ),
            host_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostType"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespace"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
