#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hci_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHciClusterArgs {
        /// The name of the Azure Stack HCI Cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Azure Stack HCI Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetHciClusterResult {
        /// The ID of the Automanage Configuration assigned to the Azure Stack HCI Cluster.
        pub automanage_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The Client ID of the Azure Active Directory used by the Azure Stack HCI Cluster.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// An immutable UUID for the Azure Stack HCI Cluster.
        pub cloud_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::stack::GetHciClusterIdentity>,
        >,
        /// The Azure Region where the Azure Stack HCI Cluster exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The object ID of the Resource Provider Service Principal.
        pub resource_provider_object_id: pulumi_gestalt_rust::Output<String>,
        /// The region specific Data Path Endpoint of the Azure Stack HCI Cluster.
        pub service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Azure Stack HCI Cluster.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Tenant ID associated with this Managed Service Identity.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetHciClusterArgs,
    ) -> GetHciClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:stack/getHciCluster:getHciCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetHciClusterResult {
            automanage_configuration_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automanageConfigurationId"),
            ),
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            cloud_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            resource_provider_object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceProviderObjectId"),
            ),
            service_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceEndpoint"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
