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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod link_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkServiceArgs {
        /// A list of Subscription UUID/GUID's that will be automatically be able to use this Private Link Service.
        #[builder(into, default)]
        pub auto_approval_subscription_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Should the Private Link Service support the Proxy Protocol?
        #[builder(into, default)]
        pub enable_proxy_protocol: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of FQDNs allowed for the Private Link Service.
        #[builder(into, default)]
        pub fqdns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of Frontend IP Configuration IDs from a Standard Load Balancer, where traffic from the Private Link Service should be routed. You can use Load Balancer Rules to direct this traffic to appropriate backend pools where your applications are running. Changing this forces a new resource to be created.
        #[builder(into)]
        pub load_balancer_frontend_ip_configuration_ids: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Private Link Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more (up to 8) `nat_ip_configuration` block as defined below.
        #[builder(into)]
        pub nat_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::privatedns::LinkServiceNatIpConfiguration>,
        >,
        /// The name of the Resource Group where the Private Link Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Subscription UUID/GUID's that will be able to see this Private Link Service.
        ///
        /// > **NOTE:** If no Subscription IDs are specified then Azure allows every Subscription to see this Private Link Service.
        #[builder(into, default)]
        pub visibility_subscription_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkServiceResult {
        /// A globally unique DNS Name for your Private Link Service. You can use this alias to request a connection to your Private Link Service.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// A list of Subscription UUID/GUID's that will be automatically be able to use this Private Link Service.
        pub auto_approval_subscription_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Should the Private Link Service support the Proxy Protocol?
        pub enable_proxy_protocol: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of FQDNs allowed for the Private Link Service.
        pub fqdns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of Frontend IP Configuration IDs from a Standard Load Balancer, where traffic from the Private Link Service should be routed. You can use Load Balancer Rules to direct this traffic to appropriate backend pools where your applications are running. Changing this forces a new resource to be created.
        pub load_balancer_frontend_ip_configuration_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Private Link Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more (up to 8) `nat_ip_configuration` block as defined below.
        pub nat_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::privatedns::LinkServiceNatIpConfiguration>,
        >,
        /// The name of the Resource Group where the Private Link Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Subscription UUID/GUID's that will be able to see this Private Link Service.
        ///
        /// > **NOTE:** If no Subscription IDs are specified then Azure allows every Subscription to see this Private Link Service.
        pub visibility_subscription_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkServiceArgs,
    ) -> LinkServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_approval_subscription_ids_binding = args
            .auto_approval_subscription_ids
            .get_output(context);
        let enable_proxy_protocol_binding = args
            .enable_proxy_protocol
            .get_output(context);
        let fqdns_binding = args.fqdns.get_output(context);
        let load_balancer_frontend_ip_configuration_ids_binding = args
            .load_balancer_frontend_ip_configuration_ids
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let nat_ip_configurations_binding = args
            .nat_ip_configurations
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let visibility_subscription_ids_binding = args
            .visibility_subscription_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:privatedns/linkService:LinkService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoApprovalSubscriptionIds".into(),
                    value: auto_approval_subscription_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableProxyProtocol".into(),
                    value: enable_proxy_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fqdns".into(),
                    value: fqdns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerFrontendIpConfigurationIds".into(),
                    value: load_balancer_frontend_ip_configuration_ids_binding.get_id(),
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
                    name: "natIpConfigurations".into(),
                    value: nat_ip_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibilitySubscriptionIds".into(),
                    value: visibility_subscription_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkServiceResult {
            alias: o.get_field("alias"),
            auto_approval_subscription_ids: o.get_field("autoApprovalSubscriptionIds"),
            enable_proxy_protocol: o.get_field("enableProxyProtocol"),
            fqdns: o.get_field("fqdns"),
            load_balancer_frontend_ip_configuration_ids: o
                .get_field("loadBalancerFrontendIpConfigurationIds"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            nat_ip_configurations: o.get_field("natIpConfigurations"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            visibility_subscription_ids: o.get_field("visibilitySubscriptionIds"),
        }
    }
}
