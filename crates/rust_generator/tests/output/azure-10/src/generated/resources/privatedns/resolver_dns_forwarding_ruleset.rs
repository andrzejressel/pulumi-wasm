/// Manages a Private DNS Resolver Dns Forwarding Ruleset.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: west europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: outbounddns
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.0.64/28
///       delegations:
///         - name: Microsoft.Network.dnsResolvers
///           serviceDelegation:
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///             name: Microsoft.Network/dnsResolvers
///   exampleResolver:
///     type: azure:privatedns:Resolver
///     name: example
///     properties:
///       name: example-resolver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       virtualNetworkId: ${exampleVirtualNetwork.id}
///   exampleResolverOutboundEndpoint:
///     type: azure:privatedns:ResolverOutboundEndpoint
///     name: example
///     properties:
///       name: example-endpoint
///       privateDnsResolverId: ${exampleResolver.id}
///       location: ${exampleResolver.location}
///       subnetId: ${exampleSubnet.id}
///       tags:
///         key: value
///   exampleResolverDnsForwardingRuleset:
///     type: azure:privatedns:ResolverDnsForwardingRuleset
///     name: example
///     properties:
///       name: example-ruleset
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       privateDnsResolverOutboundEndpointIds:
///         - ${exampleResolverOutboundEndpoint.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Private DNS Resolver Dns Forwarding Ruleset can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/resolverDnsForwardingRuleset:ResolverDnsForwardingRuleset example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/dnsForwardingRulesets/dnsForwardingRuleset1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_dns_forwarding_ruleset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverDnsForwardingRulesetArgs {
        /// Specifies the Azure Region where the Private DNS Resolver Dns Forwarding Ruleset should exist. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Private DNS Resolver Dns Forwarding Ruleset. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of IDs of the Private DNS Resolver Outbound Endpoint that is linked to the Private DNS Resolver Dns Forwarding Ruleset.
        #[builder(into)]
        pub private_dns_resolver_outbound_endpoint_ids: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// Specifies the name of the Resource Group where the Private DNS Resolver Dns Forwarding Ruleset should exist. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Private DNS Resolver Dns Forwarding Ruleset.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverDnsForwardingRulesetResult {
        /// Specifies the Azure Region where the Private DNS Resolver Dns Forwarding Ruleset should exist. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Private DNS Resolver Dns Forwarding Ruleset. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The list of IDs of the Private DNS Resolver Outbound Endpoint that is linked to the Private DNS Resolver Dns Forwarding Ruleset.
        pub private_dns_resolver_outbound_endpoint_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Specifies the name of the Resource Group where the Private DNS Resolver Dns Forwarding Ruleset should exist. Changing this forces a new Private DNS Resolver Dns Forwarding Ruleset to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Private DNS Resolver Dns Forwarding Ruleset.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResolverDnsForwardingRulesetArgs,
    ) -> ResolverDnsForwardingRulesetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let private_dns_resolver_outbound_endpoint_ids_binding_1 = args
            .private_dns_resolver_outbound_endpoint_ids
            .get_output(context);
        let private_dns_resolver_outbound_endpoint_ids_binding = private_dns_resolver_outbound_endpoint_ids_binding_1
            .get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/resolverDnsForwardingRuleset:ResolverDnsForwardingRuleset"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsResolverOutboundEndpointIds".into(),
                    value: &private_dns_resolver_outbound_endpoint_ids_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverDnsForwardingRulesetResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_dns_resolver_outbound_endpoint_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsResolverOutboundEndpointIds"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
