#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorCustomDomainArgs {
        /// The name of the Front Door Custom Domain.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Front Door Profile which the Front Door Custom Domain is bound to.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorCustomDomainResult {
        /// The ID of the Front Door Profile which the Front Door Custom Domain is bound to.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        pub dns_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The date time that the token expires.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The host name of the domain.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `tls` block as defined below.
        pub tls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorCustomDomainTl>,
        >,
        /// The challenge used for DNS TXT record or file based validation.
        pub validation_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFrontdoorCustomDomainArgs,
    ) -> GetFrontdoorCustomDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let profile_name_binding = args.profile_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cdn/getFrontdoorCustomDomain:getFrontdoorCustomDomain".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileName".into(),
                    value: profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFrontdoorCustomDomainResult {
            cdn_frontdoor_profile_id: o.get_field("cdnFrontdoorProfileId"),
            dns_zone_id: o.get_field("dnsZoneId"),
            expiration_date: o.get_field("expirationDate"),
            host_name: o.get_field("hostName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            profile_name: o.get_field("profileName"),
            resource_group_name: o.get_field("resourceGroupName"),
            tls: o.get_field("tls"),
            validation_token: o.get_field("validationToken"),
        }
    }
}
