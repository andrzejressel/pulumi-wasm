/// Manages a Private Link Service.
///
/// > **NOTE** Private Link is now in [GA](https://docs.microsoft.com/en-gb/azure/private-link/).
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.5.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.5.1.0/24
///       enforcePrivateLinkServiceNetworkPolicies: true
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-api
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
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       autoApprovalSubscriptionIds:
///         - 00000000-0000-0000-0000-000000000000
///       visibilitySubscriptionIds:
///         - 00000000-0000-0000-0000-000000000000
///       loadBalancerFrontendIpConfigurationIds:
///         - ${exampleLoadBalancer.frontendIpConfigurations[0].id}
///       natIpConfigurations:
///         - name: primary
///           privateIpAddress: 10.5.1.17
///           privateIpAddressVersion: IPv4
///           subnetId: ${exampleSubnet.id}
///           primary: true
///         - name: secondary
///           privateIpAddress: 10.5.1.18
///           privateIpAddressVersion: IPv4
///           subnetId: ${exampleSubnet.id}
///           primary: false
/// ```
///
/// ## Import
///
/// Private Link Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/linkService:LinkService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/privateLinkServices/service1
/// ```
///
pub mod link_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkServiceArgs {
        /// A list of Subscription UUID/GUID's that will be automatically be able to use this Private Link Service.
        #[builder(into, default)]
        pub auto_approval_subscription_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Should the Private Link Service support the Proxy Protocol?
        #[builder(into, default)]
        pub enable_proxy_protocol: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of FQDNs allowed for the Private Link Service.
        #[builder(into, default)]
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of Frontend IP Configuration IDs from a Standard Load Balancer, where traffic from the Private Link Service should be routed. You can use Load Balancer Rules to direct this traffic to appropriate backend pools where your applications are running. Changing this forces a new resource to be created.
        #[builder(into)]
        pub load_balancer_frontend_ip_configuration_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of this Private Link Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more (up to 8) `nat_ip_configuration` block as defined below.
        #[builder(into)]
        pub nat_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatedns::LinkServiceNatIpConfiguration>,
        >,
        /// The name of the Resource Group where the Private Link Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Subscription UUID/GUID's that will be able to see this Private Link Service.
        ///
        /// > **NOTE:** If no Subscription IDs are specified then Azure allows every Subscription to see this Private Link Service.
        #[builder(into, default)]
        pub visibility_subscription_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct LinkServiceResult {
        /// A globally unique DNS Name for your Private Link Service. You can use this alias to request a connection to your Private Link Service.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// A list of Subscription UUID/GUID's that will be automatically be able to use this Private Link Service.
        pub auto_approval_subscription_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Should the Private Link Service support the Proxy Protocol?
        pub enable_proxy_protocol: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of FQDNs allowed for the Private Link Service.
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of Frontend IP Configuration IDs from a Standard Load Balancer, where traffic from the Private Link Service should be routed. You can use Load Balancer Rules to direct this traffic to appropriate backend pools where your applications are running. Changing this forces a new resource to be created.
        pub load_balancer_frontend_ip_configuration_ids: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Private Link Service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more (up to 8) `nat_ip_configuration` block as defined below.
        pub nat_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatedns::LinkServiceNatIpConfiguration>,
        >,
        /// The name of the Resource Group where the Private Link Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Subscription UUID/GUID's that will be able to see this Private Link Service.
        ///
        /// > **NOTE:** If no Subscription IDs are specified then Azure allows every Subscription to see this Private Link Service.
        pub visibility_subscription_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkServiceArgs) -> LinkServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_approval_subscription_ids_binding = args
            .auto_approval_subscription_ids
            .get_inner();
        let enable_proxy_protocol_binding = args.enable_proxy_protocol.get_inner();
        let fqdns_binding = args.fqdns.get_inner();
        let load_balancer_frontend_ip_configuration_ids_binding = args
            .load_balancer_frontend_ip_configuration_ids
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let nat_ip_configurations_binding = args.nat_ip_configurations.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let visibility_subscription_ids_binding = args
            .visibility_subscription_ids
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/linkService:LinkService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoApprovalSubscriptionIds".into(),
                    value: &auto_approval_subscription_ids_binding,
                },
                register_interface::ObjectField {
                    name: "enableProxyProtocol".into(),
                    value: &enable_proxy_protocol_binding,
                },
                register_interface::ObjectField {
                    name: "fqdns".into(),
                    value: &fqdns_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerFrontendIpConfigurationIds".into(),
                    value: &load_balancer_frontend_ip_configuration_ids_binding,
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
                    name: "natIpConfigurations".into(),
                    value: &nat_ip_configurations_binding,
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
                    name: "visibilitySubscriptionIds".into(),
                    value: &visibility_subscription_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "autoApprovalSubscriptionIds".into(),
                },
                register_interface::ResultField {
                    name: "enableProxyProtocol".into(),
                },
                register_interface::ResultField {
                    name: "fqdns".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerFrontendIpConfigurationIds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "natIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "visibilitySubscriptionIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkServiceResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            auto_approval_subscription_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoApprovalSubscriptionIds").unwrap(),
            ),
            enable_proxy_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableProxyProtocol").unwrap(),
            ),
            fqdns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdns").unwrap(),
            ),
            load_balancer_frontend_ip_configuration_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerFrontendIpConfigurationIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nat_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natIpConfigurations").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            visibility_subscription_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibilitySubscriptionIds").unwrap(),
            ),
        }
    }
}
