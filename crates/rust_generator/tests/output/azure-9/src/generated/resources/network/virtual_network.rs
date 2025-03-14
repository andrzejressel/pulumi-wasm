/// Manages a virtual network including any configured subnets. Each subnet can
/// optionally be configured with a security group to be associated with the subnet.
///
/// > **NOTE on Virtual Networks and Subnet's:** This provider currently
/// provides both a standalone Subnet resource, and allows for Subnets to be defined in-line within the Virtual Network resource.
/// At this time you cannot use a Virtual Network with in-line Subnets in conjunction with any Subnet resources. Doing so will cause a conflict of Subnet configurations and will overwrite Subnet's.
/// > **NOTE on Virtual Networks and DNS Servers:** This provider currently provides both a standalone virtual network DNS Servers resource, and allows for DNS servers to be defined in-line within the Virtual Network resource.
/// At this time you cannot use a Virtual Network with in-line DNS servers in conjunction with any Virtual Network DNS Servers resources. Doing so will cause a conflict of Virtual Network DNS Servers configurations and will overwrite virtual networks DNS servers.
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
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: example-security-group
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///       dnsServers:
///         - 10.0.0.4
///         - 10.0.0.5
///       subnets:
///         - name: subnet1
///           addressPrefixes:
///             - 10.0.1.0/24
///         - name: subnet2
///           addressPrefixes:
///             - 10.0.2.0/24
///           securityGroup: ${exampleNetworkSecurityGroup.id}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Virtual Networks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetwork:VirtualNetwork exampleNetwork /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkArgs {
        /// The address space that is used the virtual network. You can supply more than one address space.
        #[builder(into)]
        pub address_spaces: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The BGP community attribute in format `<as-number>:<community-value>`.
        ///
        /// > **NOTE** The `as-number` segment is the Microsoft ASN, which is always `12076` for now.
        #[builder(into, default)]
        pub bgp_community: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `ddos_protection_plan` block as documented below.
        #[builder(into, default)]
        pub ddos_protection_plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VirtualNetworkDdosProtectionPlan>,
        >,
        /// List of IP addresses of DNS servers
        ///
        /// > **NOTE** Since `dns_servers` can be configured both inline and via the separate `azure.network.VirtualNetworkDnsServers` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network should exist. Changing this forces a new Virtual Network to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `encryption` block as defined below.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VirtualNetworkEncryption>,
        >,
        /// The flow timeout in minutes for the Virtual Network, which is used to enable connection tracking for intra-VM flows. Possible values are between `4` and `30` minutes.
        #[builder(into, default)]
        pub flow_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The location/region where the virtual network is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the virtual network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Can be specified multiple times to define multiple subnets. Each `subnet` block supports fields documented below.
        ///
        /// > **NOTE** Since `subnet` can be configured both inline and via the separate `azure.network.Subnet` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub subnets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::VirtualNetworkSubnet>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkResult {
        /// The address space that is used the virtual network. You can supply more than one address space.
        pub address_spaces: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The BGP community attribute in format `<as-number>:<community-value>`.
        ///
        /// > **NOTE** The `as-number` segment is the Microsoft ASN, which is always `12076` for now.
        pub bgp_community: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `ddos_protection_plan` block as documented below.
        pub ddos_protection_plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::VirtualNetworkDdosProtectionPlan>,
        >,
        /// List of IP addresses of DNS servers
        ///
        /// > **NOTE** Since `dns_servers` can be configured both inline and via the separate `azure.network.VirtualNetworkDnsServers` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network should exist. Changing this forces a new Virtual Network to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `encryption` block as defined below.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::VirtualNetworkEncryption>,
        >,
        /// The flow timeout in minutes for the Virtual Network, which is used to enable connection tracking for intra-VM flows. Possible values are between `4` and `30` minutes.
        pub flow_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The GUID of the virtual network.
        pub guid: pulumi_gestalt_rust::Output<String>,
        /// The location/region where the virtual network is created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the virtual network. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Can be specified multiple times to define multiple subnets. Each `subnet` block supports fields documented below.
        ///
        /// > **NOTE** Since `subnet` can be configured both inline and via the separate `azure.network.Subnet` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub subnets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::VirtualNetworkSubnet>,
        >,
        /// A mapping of tags to assign to the resource.
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
        args: VirtualNetworkArgs,
    ) -> VirtualNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_spaces_binding = args.address_spaces.get_output(context);
        let bgp_community_binding = args.bgp_community.get_output(context);
        let ddos_protection_plan_binding = args.ddos_protection_plan.get_output(context);
        let dns_servers_binding = args.dns_servers.get_output(context);
        let edge_zone_binding = args.edge_zone.get_output(context);
        let encryption_binding = args.encryption.get_output(context);
        let flow_timeout_in_minutes_binding = args
            .flow_timeout_in_minutes
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subnets_binding = args.subnets.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualNetwork:VirtualNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressSpaces".into(),
                    value: &address_spaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpCommunity".into(),
                    value: &bgp_community_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ddosProtectionPlan".into(),
                    value: &ddos_protection_plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flowTimeoutInMinutes".into(),
                    value: &flow_timeout_in_minutes_binding.drop_type(),
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
                    name: "subnets".into(),
                    value: &subnets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkResult {
            address_spaces: o.get_field("addressSpaces"),
            bgp_community: o.get_field("bgpCommunity"),
            ddos_protection_plan: o.get_field("ddosProtectionPlan"),
            dns_servers: o.get_field("dnsServers"),
            edge_zone: o.get_field("edgeZone"),
            encryption: o.get_field("encryption"),
            flow_timeout_in_minutes: o.get_field("flowTimeoutInMinutes"),
            guid: o.get_field("guid"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            subnets: o.get_field("subnets"),
            tags: o.get_field("tags"),
        }
    }
}
