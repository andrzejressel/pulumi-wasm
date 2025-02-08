#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_virtual_network_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverVirtualNetworkLinkArgs {
        /// ID of the Private DNS Resolver DNS Forwarding Ruleset.
        #[builder(into)]
        pub dns_forwarding_ruleset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Private DNS Resolver Virtual Network Link.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverVirtualNetworkLinkResult {
        pub dns_forwarding_ruleset_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The metadata attached to the Private DNS Resolver Virtual Network Link.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Network that is linked to the Private DNS Resolver Virtual Network Link.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverVirtualNetworkLinkArgs,
    ) -> GetResolverVirtualNetworkLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dns_forwarding_ruleset_id_binding = args
            .dns_forwarding_ruleset_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getResolverVirtualNetworkLink:getResolverVirtualNetworkLink"
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
        GetResolverVirtualNetworkLinkResult {
            dns_forwarding_ruleset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsForwardingRulesetId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            virtual_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}
