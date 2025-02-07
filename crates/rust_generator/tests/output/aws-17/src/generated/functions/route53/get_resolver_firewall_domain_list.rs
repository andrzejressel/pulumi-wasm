pub mod get_resolver_firewall_domain_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallDomainListArgs {
        /// The ID of the domain list.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub firewall_domain_list_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallDomainListResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        pub creator_request_id: pulumi_gestalt_rust::Output<String>,
        pub domain_count: pulumi_gestalt_rust::Output<i32>,
        pub firewall_domain_list_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_owner_name: pulumi_gestalt_rust::Output<String>,
        pub modification_time: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub status_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverFirewallDomainListArgs,
    ) -> GetResolverFirewallDomainListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let firewall_domain_list_id_binding = args
            .firewall_domain_list_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallDomainList:getResolverFirewallDomainList"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallDomainListId".into(),
                    value: &firewall_domain_list_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverFirewallDomainListResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            creation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            creator_request_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creatorRequestId"),
            ),
            domain_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainCount"),
            ),
            firewall_domain_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallDomainListId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed_owner_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedOwnerName"),
            ),
            modification_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modificationTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
        }
    }
}
