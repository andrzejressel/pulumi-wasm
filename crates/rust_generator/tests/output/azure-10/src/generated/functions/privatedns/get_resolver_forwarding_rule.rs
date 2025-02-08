#[allow(clippy::doc_lazy_continuation)]
pub mod get_resolver_forwarding_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverForwardingRuleArgs {
        /// ID of the Private DNS Resolver Forwarding Ruleset.
        #[builder(into)]
        pub dns_forwarding_ruleset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Private DNS Resolver Forwarding Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverForwardingRuleResult {
        pub dns_forwarding_ruleset_id: pulumi_gestalt_rust::Output<String>,
        /// The domain name for the Private DNS Resolver Forwarding Rule.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Is the Private DNS Resolver Forwarding Rule enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The metadata attached to the Private DNS Resolver Forwarding Rule.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of `target_dns_servers` block as defined below.
        pub target_dns_servers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privatedns::GetResolverForwardingRuleTargetDnsServer,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverForwardingRuleArgs,
    ) -> GetResolverForwardingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dns_forwarding_ruleset_id_binding = args
            .dns_forwarding_ruleset_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverForwardingRule:getResolverForwardingRule"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsForwardingRulesetId".into(),
                    value: &dns_forwarding_ruleset_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverForwardingRuleResult {
            dns_forwarding_ruleset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsForwardingRulesetId"),
            ),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            target_dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetDnsServers"),
            ),
        }
    }
}
