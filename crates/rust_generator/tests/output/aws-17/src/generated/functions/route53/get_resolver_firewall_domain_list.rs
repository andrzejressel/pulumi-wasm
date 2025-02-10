#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetResolverFirewallDomainListArgs,
    ) -> GetResolverFirewallDomainListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let firewall_domain_list_id_binding = args
            .firewall_domain_list_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getResolverFirewallDomainList:getResolverFirewallDomainList"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallDomainListId".into(),
                    value: firewall_domain_list_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResolverFirewallDomainListResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            creator_request_id: o.get_field("creatorRequestId"),
            domain_count: o.get_field("domainCount"),
            firewall_domain_list_id: o.get_field("firewallDomainListId"),
            id: o.get_field("id"),
            managed_owner_name: o.get_field("managedOwnerName"),
            modification_time: o.get_field("modificationTime"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
        }
    }
}
