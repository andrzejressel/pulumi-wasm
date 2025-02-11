/// Manages an Azure Stack HCI Logical Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleHciLogicalNetwork:
///     type: azure:stack:HciLogicalNetwork
///     name: example
///     properties:
///       name: example-hci-ln
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       virtualSwitchName: ConvergedSwitch(managementcompute)
///       dnsServers:
///         - 10.0.0.7
///         - 10.0.0.8
///       subnet:
///         ipAllocationMethod: Static
///         addressPrefix: 10.0.0.0/24
///         vlanId: 123
///         route:
///           addressPrefix: 0.0.0.0/0
///           nextHopIpAddress: 10.0.0.1
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Azure Stack HCI Logical Networks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciLogicalNetwork:HciLogicalNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AzureStackHCI/logicalNetworks/ln1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hci_logical_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciLogicalNetworkArgs {
        /// The ID of Custom Location where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Logical Networks. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Azure Region where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Logical Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `subnet` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::stack::HciLogicalNetworkSubnet,
        >,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Logical Network.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual switch on the cluster used to associate with the Azure Stack HCI Logical Network. Possible switch names can be retrieved by following this [Azure guide](https://learn.microsoft.com/azure-stack/hci/manage/create-logical-networks?tabs=azurecli#prerequisites). Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_switch_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HciLogicalNetworkResult {
        /// The ID of Custom Location where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_gestalt_rust::Output<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Logical Networks. Changing this forces a new resource to be created.
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Azure Region where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure Stack HCI Logical Network. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `subnet` block as defined below. Changing this forces a new resource to be created.
        pub subnet: pulumi_gestalt_rust::Output<
            super::super::types::stack::HciLogicalNetworkSubnet,
        >,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Logical Network.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual switch on the cluster used to associate with the Azure Stack HCI Logical Network. Possible switch names can be retrieved by following this [Azure guide](https://learn.microsoft.com/azure-stack/hci/manage/create-logical-networks?tabs=azurecli#prerequisites). Changing this forces a new resource to be created.
        pub virtual_switch_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciLogicalNetworkArgs,
    ) -> HciLogicalNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_location_id_binding = args.custom_location_id.get_output(context);
        let dns_servers_binding = args.dns_servers.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subnet_binding = args.subnet.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_switch_name_binding = args.virtual_switch_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:stack/hciLogicalNetwork:HciLogicalNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding.drop_type(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnet".into(),
                    value: &subnet_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualSwitchName".into(),
                    value: &virtual_switch_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HciLogicalNetworkResult {
            custom_location_id: o.get_field("customLocationId"),
            dns_servers: o.get_field("dnsServers"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnet: o.get_field("subnet"),
            tags: o.get_field("tags"),
            virtual_switch_name: o.get_field("virtualSwitchName"),
        }
    }
}
