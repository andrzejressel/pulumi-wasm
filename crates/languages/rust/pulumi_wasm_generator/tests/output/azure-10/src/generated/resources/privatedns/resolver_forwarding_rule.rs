/// Manages a Private DNS Resolver Forwarding Rule.
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
///       name: example-drdfr
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       privateDnsResolverOutboundEndpointIds:
///         - ${exampleResolverOutboundEndpoint.id}
///   exampleResolverForwardingRule:
///     type: azure:privatedns:ResolverForwardingRule
///     name: example
///     properties:
///       name: example-rule
///       dnsForwardingRulesetId: ${exampleResolverDnsForwardingRuleset.id}
///       domainName: onprem.local.
///       enabled: true
///       targetDnsServers:
///         - ipAddress: 10.10.0.1
///           port: 53
///       metadata:
///         key: value
/// ```
///
/// ## Import
///
/// Private DNS Resolver Forwarding Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/resolverForwardingRule:ResolverForwardingRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/dnsForwardingRulesets/dnsForwardingRuleset1/forwardingRules/forwardingRule1
/// ```
///
pub mod resolver_forwarding_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverForwardingRuleArgs {
        /// Specifies the ID of the Private DNS Resolver Forwarding Ruleset. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        #[builder(into)]
        pub dns_forwarding_ruleset_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the domain name for the Private DNS Resolver Forwarding Rule. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the state of the Private DNS Resolver Forwarding Rule. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Metadata attached to the Private DNS Resolver Forwarding Rule.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the name which should be used for this Private DNS Resolver Forwarding Rule. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Can be specified multiple times to define multiple target DNS servers. Each `target_dns_servers` block as defined below.
        #[builder(into)]
        pub target_dns_servers: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::privatedns::ResolverForwardingRuleTargetDnsServer>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverForwardingRuleResult {
        /// Specifies the ID of the Private DNS Resolver Forwarding Ruleset. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        pub dns_forwarding_ruleset_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the domain name for the Private DNS Resolver Forwarding Rule. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the state of the Private DNS Resolver Forwarding Rule. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Metadata attached to the Private DNS Resolver Forwarding Rule.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the name which should be used for this Private DNS Resolver Forwarding Rule. Changing this forces a new Private DNS Resolver Forwarding Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Can be specified multiple times to define multiple target DNS servers. Each `target_dns_servers` block as defined below.
        pub target_dns_servers: pulumi_wasm_rust::Output<
            Vec<super::super::types::privatedns::ResolverForwardingRuleTargetDnsServer>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResolverForwardingRuleArgs,
    ) -> ResolverForwardingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_forwarding_ruleset_id_binding = args
            .dns_forwarding_ruleset_id
            .get_output(context)
            .get_inner();
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let target_dns_servers_binding = args
            .target_dns_servers
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/resolverForwardingRule:ResolverForwardingRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsForwardingRulesetId".into(),
                    value: &dns_forwarding_ruleset_id_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "targetDnsServers".into(),
                    value: &target_dns_servers_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverForwardingRuleResult {
            dns_forwarding_ruleset_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsForwardingRulesetId"),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            target_dns_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetDnsServers"),
            ),
        }
    }
}
