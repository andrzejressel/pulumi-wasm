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
pub mod virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkArgs {
        /// The address space that is used the virtual network. You can supply more than one address space.
        #[builder(into)]
        pub address_spaces: pulumi_wasm_rust::Output<Vec<String>>,
        /// The BGP community attribute in format `<as-number>:<community-value>`.
        ///
        /// > **NOTE** The `as-number` segment is the Microsoft ASN, which is always `12076` for now.
        #[builder(into, default)]
        pub bgp_community: pulumi_wasm_rust::Output<Option<String>>,
        /// A `ddos_protection_plan` block as documented below.
        #[builder(into, default)]
        pub ddos_protection_plan: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkDdosProtectionPlan>,
        >,
        /// List of IP addresses of DNS servers
        ///
        /// > **NOTE** Since `dns_servers` can be configured both inline and via the separate `azure.network.VirtualNetworkDnsServers` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network should exist. Changing this forces a new Virtual Network to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// A `encryption` block as defined below.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkEncryption>,
        >,
        /// The flow timeout in minutes for the Virtual Network, which is used to enable connection tracking for intra-VM flows. Possible values are between `4` and `30` minutes.
        #[builder(into, default)]
        pub flow_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The location/region where the virtual network is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the virtual network. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Can be specified multiple times to define multiple subnets. Each `subnet` block supports fields documented below.
        ///
        /// > **NOTE** Since `subnet` can be configured both inline and via the separate `azure.network.Subnet` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub subnets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VirtualNetworkSubnet>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkResult {
        /// The address space that is used the virtual network. You can supply more than one address space.
        pub address_spaces: pulumi_wasm_rust::Output<Vec<String>>,
        /// The BGP community attribute in format `<as-number>:<community-value>`.
        ///
        /// > **NOTE** The `as-number` segment is the Microsoft ASN, which is always `12076` for now.
        pub bgp_community: pulumi_wasm_rust::Output<Option<String>>,
        /// A `ddos_protection_plan` block as documented below.
        pub ddos_protection_plan: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkDdosProtectionPlan>,
        >,
        /// List of IP addresses of DNS servers
        ///
        /// > **NOTE** Since `dns_servers` can be configured both inline and via the separate `azure.network.VirtualNetworkDnsServers` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Edge Zone within the Azure Region where this Virtual Network should exist. Changing this forces a new Virtual Network to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// A `encryption` block as defined below.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::network::VirtualNetworkEncryption>,
        >,
        /// The flow timeout in minutes for the Virtual Network, which is used to enable connection tracking for intra-VM flows. Possible values are between `4` and `30` minutes.
        pub flow_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The GUID of the virtual network.
        pub guid: pulumi_wasm_rust::Output<String>,
        /// The location/region where the virtual network is created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the virtual network. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the virtual network. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Can be specified multiple times to define multiple subnets. Each `subnet` block supports fields documented below.
        ///
        /// > **NOTE** Since `subnet` can be configured both inline and via the separate `azure.network.Subnet` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub subnets: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VirtualNetworkSubnet>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualNetworkArgs) -> VirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_spaces_binding = args.address_spaces.get_inner();
        let bgp_community_binding = args.bgp_community.get_inner();
        let ddos_protection_plan_binding = args.ddos_protection_plan.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let encryption_binding = args.encryption.get_inner();
        let flow_timeout_in_minutes_binding = args.flow_timeout_in_minutes.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subnets_binding = args.subnets.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualNetwork:VirtualNetwork".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressSpaces".into(),
                    value: &address_spaces_binding,
                },
                register_interface::ObjectField {
                    name: "bgpCommunity".into(),
                    value: &bgp_community_binding,
                },
                register_interface::ObjectField {
                    name: "ddosProtectionPlan".into(),
                    value: &ddos_protection_plan_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "flowTimeoutInMinutes".into(),
                    value: &flow_timeout_in_minutes_binding,
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
                    name: "subnets".into(),
                    value: &subnets_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressSpaces".into(),
                },
                register_interface::ResultField {
                    name: "bgpCommunity".into(),
                },
                register_interface::ResultField {
                    name: "ddosProtectionPlan".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "flowTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "guid".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkResult {
            address_spaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressSpaces").unwrap(),
            ),
            bgp_community: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpCommunity").unwrap(),
            ),
            ddos_protection_plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ddosProtectionPlan").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            flow_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flowTimeoutInMinutes").unwrap(),
            ),
            guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guid").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
