/// Manages a Dev Center Network Connection.
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
///     let exampleNetworkConnection = network_connection::create(
///         "exampleNetworkConnection",
///         NetworkConnectionArgs::builder()
///             .domain_join_type("AzureADJoin")
///             .location("${example.location}")
///             .name("example-dcnc")
///             .resource_group_name("${example.name}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Dev Center Network Connection can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/networkConnection:NetworkConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/networkConnections/networkConnection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkConnectionArgs {
        /// The Azure Active Directory Join type. Possible values are `AzureADJoin` and `HybridAzureADJoin`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_join_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Azure Active Directory domain.
        #[builder(into, default)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password for the account used to join domain.
        #[builder(into, default)]
        pub domain_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The username of the Azure Active Directory account (user or service account) that has permissions to create computer objects in Active Directory.
        #[builder(into, default)]
        pub domain_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Dev Center Network Connection should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dev Center Network Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Active Directory domain Organization Unit (OU).
        #[builder(into, default)]
        pub organization_unit: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group within which this Dev Center Network Connection should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet that is used to attach Virtual Machines.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Dev Center Network Connection.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkConnectionResult {
        /// The Azure Active Directory Join type. Possible values are `AzureADJoin` and `HybridAzureADJoin`. Changing this forces a new resource to be created.
        pub domain_join_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Active Directory domain.
        pub domain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The password for the account used to join domain.
        pub domain_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The username of the Azure Active Directory account (user or service account) that has permissions to create computer objects in Active Directory.
        pub domain_username: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Dev Center Network Connection should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Network Connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Active Directory domain Organization Unit (OU).
        pub organization_unit: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group within which this Dev Center Network Connection should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet that is used to attach Virtual Machines.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Dev Center Network Connection.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkConnectionArgs,
    ) -> NetworkConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_join_type_binding = args.domain_join_type.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let domain_password_binding = args.domain_password.get_output(context);
        let domain_username_binding = args.domain_username.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let organization_unit_binding = args.organization_unit.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/networkConnection:NetworkConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainJoinType".into(),
                    value: domain_join_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainPassword".into(),
                    value: domain_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainUsername".into(),
                    value: domain_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationUnit".into(),
                    value: organization_unit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkConnectionResult {
            domain_join_type: o.get_field("domainJoinType"),
            domain_name: o.get_field("domainName"),
            domain_password: o.get_field("domainPassword"),
            domain_username: o.get_field("domainUsername"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            organization_unit: o.get_field("organizationUnit"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
        }
    }
}
