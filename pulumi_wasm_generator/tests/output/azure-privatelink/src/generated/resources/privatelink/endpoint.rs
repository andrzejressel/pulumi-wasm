/// Manages a Private Endpoint.
///
/// Azure Private Endpoint is a network interface that connects you privately and securely to a service powered by Azure Private Link. Private Endpoint uses a private IP address from your VNet, effectively bringing the service into your VNet. The service could be an Azure service such as Azure Storage, SQL, etc. or your own Private Link Service.
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   service:
///     type: azure:network:Subnet
///     properties:
///       name: service
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       enforcePrivateLinkServiceNetworkPolicies: true
///   endpoint:
///     type: azure:network:Subnet
///     properties:
///       name: endpoint
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       enforcePrivateLinkEndpointNetworkPolicies: true
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       allocationMethod: Static
///   exampleLoadBalancer:
///     type: azure:lb:LoadBalancer
///     name: example
///     properties:
///       name: example-lb
///       sku: Standard
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       frontendIpConfigurations:
///         - name: ${examplePublicIp.name}
///           publicIpAddressId: ${examplePublicIp.id}
///   exampleLinkService:
///     type: azure:privatedns:LinkService
///     name: example
///     properties:
///       name: example-privatelink
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       natIpConfigurations:
///         - name: ${examplePublicIp.name}
///           primary: true
///           subnetId: ${service.id}
///       loadBalancerFrontendIpConfigurationIds:
///         - ${exampleLoadBalancer.frontendIpConfigurations[0].id}
///   exampleEndpoint:
///     type: azure:privatelink:Endpoint
///     name: example
///     properties:
///       name: example-endpoint
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       subnetId: ${endpoint.id}
///       privateServiceConnection:
///         name: example-privateserviceconnection
///         privateConnectionResourceId: ${exampleLinkService.id}
///         isManualConnection: false
/// ```
///
/// Using a Private Link Service Alias with existing resources:
///
/// ```yaml
/// resources:
///   exampleEndpoint:
///     type: azure:privatelink:Endpoint
///     name: example
///     properties:
///       name: example-endpoint
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       subnetId: ${subnet.id}
///       privateServiceConnection:
///         name: example-privateserviceconnection
///         privateConnectionResourceAlias: example-privatelinkservice.d20286c8-4ea5-11eb-9584-8f53157226c6.centralus.azure.privatelinkservice
///         isManualConnection: true
///         requestMessage: PL
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getResourceGroup
///       arguments:
///         name: example-resources
///   vnet:
///     fn::invoke:
///       function: azure:network:getVirtualNetwork
///       arguments:
///         name: example-network
///         resourceGroupName: ${example.name}
///   subnet:
///     fn::invoke:
///       function: azure:network:getSubnet
///       arguments:
///         name: default
///         virtualNetworkName: ${vnet.name}
///         resourceGroupName: ${example.name}
/// ```
///
/// Using a Private Endpoint pointing to an *owned* Azure service, with proper DNS configuration:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("exampleaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEndpoint = endpoint::create(
///         "exampleEndpoint",
///         EndpointArgs::builder()
///             .location("${example.location}")
///             .name("example-endpoint")
///             .private_dns_zone_group(
///                 EndpointPrivateDnsZoneGroup::builder()
///                     .name("example-dns-zone-group")
///                     .privateDnsZoneIds(vec!["${exampleZone.id}",])
///                     .build_struct(),
///             )
///             .private_service_connection(
///                 EndpointPrivateServiceConnection::builder()
///                     .isManualConnection(false)
///                     .name("example-privateserviceconnection")
///                     .privateConnectionResourceId("${exampleAccount.id}")
///                     .subresourceNames(vec!["blob",])
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("subnetname")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("virtnetname")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("privatelink.blob.core.windows.net")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleZoneVirtualNetworkLink = zone_virtual_network_link::create(
///         "exampleZoneVirtualNetworkLink",
///         ZoneVirtualNetworkLinkArgs::builder()
///             .name("example-link")
///             .private_dns_zone_name("${exampleZone.name}")
///             .resource_group_name("${example.name}")
///             .virtual_network_id("${exampleVirtualNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example HCL Configurations
///
/// * How to conneca `Private Endpoint` to a Application Gateway
/// * How to connect a `Private Endpoint` to a Cosmos MongoDB
/// * How to connect a `Private Endpoint` to a Cosmos PostgreSQL
/// * How to connect a `Private Endpoint` to a PostgreSQL Server
/// * How to connect a `Private Endpoint` to a Private Link Service
/// * How to connect a `Private Endpoint` to a Private DNS Group
/// * How to connect a `Private Endpoint` to a Databricks Workspace
///
/// ## Import
///
/// Private Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatelink/endpoint:Endpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/privateEndpoints/endpoint1
/// ```
///
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// The custom name of the network interface attached to the private endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub custom_network_interface_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ip_configuration` blocks as defined below. This allows a static IP address to be set for this Private Endpoint, otherwise an address is dynamically allocated from the Subnet.
        #[builder(into, default)]
        pub ip_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::privatelink::EndpointIpConfiguration>>,
        >,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Name of the Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `private_dns_zone_group` block as defined below.
        #[builder(into, default)]
        pub private_dns_zone_group: pulumi_wasm_rust::Output<
            Option<super::super::types::privatelink::EndpointPrivateDnsZoneGroup>,
        >,
        /// A `private_service_connection` block as defined below.
        #[builder(into)]
        pub private_service_connection: pulumi_wasm_rust::Output<
            super::super::types::privatelink::EndpointPrivateServiceConnection,
        >,
        /// Specifies the Name of the Resource Group within which the Private Endpoint should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet from which Private IP Addresses will be allocated for this Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// A `custom_dns_configs` block as defined below.
        pub custom_dns_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatelink::EndpointCustomDnsConfig>,
        >,
        /// The custom name of the network interface attached to the private endpoint. Changing this forces a new resource to be created.
        pub custom_network_interface_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ip_configuration` blocks as defined below. This allows a static IP address to be set for this Private Endpoint, otherwise an address is dynamically allocated from the Subnet.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::privatelink::EndpointIpConfiguration>>,
        >,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the Name of the Private Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_interface` block as defined below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatelink::EndpointNetworkInterface>,
        >,
        /// A `private_dns_zone_configs` block as defined below.
        pub private_dns_zone_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatelink::EndpointPrivateDnsZoneConfig>,
        >,
        /// A `private_dns_zone_group` block as defined below.
        pub private_dns_zone_group: pulumi_wasm_rust::Output<
            Option<super::super::types::privatelink::EndpointPrivateDnsZoneGroup>,
        >,
        /// A `private_service_connection` block as defined below.
        pub private_service_connection: pulumi_wasm_rust::Output<
            super::super::types::privatelink::EndpointPrivateServiceConnection,
        >,
        /// Specifies the Name of the Resource Group within which the Private Endpoint should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet from which Private IP Addresses will be allocated for this Private Endpoint. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointArgs) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_network_interface_name_binding = args
            .custom_network_interface_name
            .get_inner();
        let ip_configurations_binding = args.ip_configurations.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let private_dns_zone_group_binding = args.private_dns_zone_group.get_inner();
        let private_service_connection_binding = args
            .private_service_connection
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatelink/endpoint:Endpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customNetworkInterfaceName".into(),
                    value: &custom_network_interface_name_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurations".into(),
                    value: &ip_configurations_binding,
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
                    name: "privateDnsZoneGroup".into(),
                    value: &private_dns_zone_group_binding,
                },
                register_interface::ObjectField {
                    name: "privateServiceConnection".into(),
                    value: &private_service_connection_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customDnsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "customNetworkInterfaceName".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsZoneConfigs".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsZoneGroup".into(),
                },
                register_interface::ResultField {
                    name: "privateServiceConnection".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
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
        EndpointResult {
            custom_dns_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDnsConfigs").unwrap(),
            ),
            custom_network_interface_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customNetworkInterfaceName").unwrap(),
            ),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            private_dns_zone_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsZoneConfigs").unwrap(),
            ),
            private_dns_zone_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsZoneGroup").unwrap(),
            ),
            private_service_connection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateServiceConnection").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}