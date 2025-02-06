/// Manages a Front Door (standard/premium) Security Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-cdn-frontdoor")
///             .build_struct(),
///     );
///     let exampleFrontdoorCustomDomain = frontdoor_custom_domain::create(
///         "exampleFrontdoorCustomDomain",
///         FrontdoorCustomDomainArgs::builder()
///             .cdn_frontdoor_profile_id("${exampleFrontdoorProfile.id}")
///             .dns_zone_id("${exampleZone.id}")
///             .host_name("contoso.fabrikam.com")
///             .name("example-customDomain")
///             .tls(
///                 FrontdoorCustomDomainTls::builder()
///                     .certificateType("ManagedCertificate")
///                     .minimumTlsVersion("TLS12")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleFrontdoorFirewallPolicy = frontdoor_firewall_policy::create(
///         "exampleFrontdoorFirewallPolicy",
///         FrontdoorFirewallPolicyArgs::builder()
///             .custom_block_response_body(
///                 "PGh0bWw+CjxoZWFkZXI+PHRpdGxlPkhlbGxvPC90aXRsZT48L2hlYWRlcj4KPGJvZHk+CkhlbGxvIHdvcmxkCjwvYm9keT4KPC9odG1sPg==",
///             )
///             .custom_block_response_status_code(403)
///             .custom_rules(
///                 vec![
///                     FrontdoorFirewallPolicyCustomRule::builder().action("Block")
///                     .enabled(true)
///                     .matchConditions(vec![FrontdoorFirewallPolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24", "10.0.1.0/24",])
///                     .matchVariable("RemoteAddr").negationCondition(false)
///                     .operator("IPMatch").build_struct(),]).name("Rule1").priority(1)
///                     .rateLimitDurationInMinutes(1).rateLimitThreshold(10). type
///                     ("MatchRule").build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .mode("Prevention")
///             .name("exampleWAF")
///             .redirect_url("https://www.contoso.com")
///             .resource_group_name("${example.name}")
///             .sku_name("${exampleFrontdoorProfile.skuName}")
///             .build_struct(),
///     );
///     let exampleFrontdoorProfile = frontdoor_profile::create(
///         "exampleFrontdoorProfile",
///         FrontdoorProfileArgs::builder()
///             .name("example-profile")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_AzureFrontDoor")
///             .build_struct(),
///     );
///     let exampleFrontdoorSecurityPolicy = frontdoor_security_policy::create(
///         "exampleFrontdoorSecurityPolicy",
///         FrontdoorSecurityPolicyArgs::builder()
///             .cdn_frontdoor_profile_id("${exampleFrontdoorProfile.id}")
///             .name("Example-Security-Policy")
///             .security_policies(
///                 FrontdoorSecurityPolicySecurityPolicies::builder()
///                     .firewall(
///                         FrontdoorSecurityPolicySecurityPoliciesFirewall::builder()
///                             .association(
///                                 FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation::builder()
///                                     .domains(
///                                         vec![
///                                             FrontdoorSecurityPolicySecurityPoliciesFirewallAssociationDomain::builder()
///                                             .cdnFrontdoorDomainId("${exampleFrontdoorCustomDomain.id}")
///                                             .build_struct(),
///                                         ],
///                                     )
///                                     .patternsToMatch("/*")
///                                     .build_struct(),
///                             )
///                             .cdnFrontdoorFirewallPolicyId(
///                                 "${exampleFrontdoorFirewallPolicy.id}",
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("sub-domain.domain.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Front Door Security Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorSecurityPolicy:FrontdoorSecurityPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Cdn/profiles/profile1/securityPolicies/policy1
/// ```
///
pub mod frontdoor_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorSecurityPolicyArgs {
        /// The Front Door Profile Resource Id that is linked to this Front Door Security Policy. Changing this forces a new Front Door Security Policy to be created.
        #[builder(into)]
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Front Door Security Policy. Possible values must not be an empty string. Changing this forces a new Front Door Security Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `security_policies` block as defined below. Changing this forces a new Front Door Security Policy to be created.
        #[builder(into)]
        pub security_policies: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cdn::FrontdoorSecurityPolicySecurityPolicies,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorSecurityPolicyResult {
        /// The Front Door Profile Resource Id that is linked to this Front Door Security Policy. Changing this forces a new Front Door Security Policy to be created.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Front Door Security Policy. Possible values must not be an empty string. Changing this forces a new Front Door Security Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `security_policies` block as defined below. Changing this forces a new Front Door Security Policy to be created.
        pub security_policies: pulumi_gestalt_rust::Output<
            super::super::types::cdn::FrontdoorSecurityPolicySecurityPolicies,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FrontdoorSecurityPolicyArgs,
    ) -> FrontdoorSecurityPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cdn_frontdoor_profile_id_binding = args
            .cdn_frontdoor_profile_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let security_policies_binding = args
            .security_policies
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorSecurityPolicy:FrontdoorSecurityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdnFrontdoorProfileId".into(),
                    value: &cdn_frontdoor_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicies".into(),
                    value: &security_policies_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrontdoorSecurityPolicyResult {
            cdn_frontdoor_profile_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorProfileId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            security_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicies"),
            ),
        }
    }
}
