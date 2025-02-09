/// Manages an Azure Stack HCI Cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleHciCluster:
///     type: azure:stack:HciCluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       clientId: ${example.applicationId}
///       tenantId: ${current.tenantId}
///       identity:
///         type: SystemAssigned
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getApplication
///       arguments:
///         displayName: Allowed resource types
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Azure Stack HCI Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciCluster:HciCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AzureStackHCI/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hci_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciClusterArgs {
        /// The ID of the Automanage Configuration assigned to the Azure Stack HCI Cluster.
        #[builder(into, default)]
        pub automanage_configuration_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Client ID of the Azure Active Directory Application which is used by the Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::stack::HciClusterIdentity>,
        >,
        /// The Azure Region where the Azure Stack HCI Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Stack HCI Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Tenant ID of the Azure Active Directory which is used by the Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** If unspecified the Tenant ID of the Provider will be used.
        #[builder(into, default)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HciClusterResult {
        /// The ID of the Automanage Configuration assigned to the Azure Stack HCI Cluster.
        pub automanage_configuration_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Client ID of the Azure Active Directory Application which is used by the Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        pub client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An immutable UUID for the Azure Stack HCI Cluster.
        pub cloud_id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::stack::HciClusterIdentity>,
        >,
        /// The Azure Region where the Azure Stack HCI Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The object ID of the Resource Provider Service Principal.
        pub resource_provider_object_id: pulumi_gestalt_rust::Output<String>,
        /// The region specific Data Path Endpoint of the Azure Stack HCI Cluster.
        pub service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Tenant ID of the Azure Active Directory which is used by the Azure Stack HCI Cluster. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** If unspecified the Tenant ID of the Provider will be used.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HciClusterArgs,
    ) -> HciClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automanage_configuration_id_binding_1 = args
            .automanage_configuration_id
            .get_output(context);
        let automanage_configuration_id_binding = automanage_configuration_id_binding_1
            .get_inner();
        let client_id_binding_1 = args.client_id.get_output(context);
        let client_id_binding = client_id_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let tenant_id_binding_1 = args.tenant_id.get_output(context);
        let tenant_id_binding = tenant_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciCluster:HciCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automanageConfigurationId".into(),
                    value: &automanage_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HciClusterResult {
            automanage_configuration_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automanageConfigurationId"),
            ),
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            cloud_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudId"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
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
