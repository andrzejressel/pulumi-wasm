#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverForwardingRuleArgs,
    ) -> GetResolverForwardingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dns_forwarding_ruleset_id_binding = args
            .dns_forwarding_ruleset_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatedns/getResolverForwardingRule:getResolverForwardingRule"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsForwardingRulesetId".into(),
                    value: dns_forwarding_ruleset_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverForwardingRuleResult {
            dns_forwarding_ruleset_id: o.get_field("dnsForwardingRulesetId"),
            domain_name: o.get_field("domainName"),
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            target_dns_servers: o.get_field("targetDnsServers"),
        }
    }
}
