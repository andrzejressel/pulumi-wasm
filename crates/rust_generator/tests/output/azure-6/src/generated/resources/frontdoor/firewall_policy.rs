/// !> **IMPORTANT** This deploys an Azure Front Door (classic) resource which has been deprecated and will receive security updates only. Please migrate your existing Azure Front Door (classic) deployments to the new Azure Front Door (standard/premium) resources. For your convenience, the service team has exposed a `Front Door Classic` to `Front Door Standard/Premium` [migration tool](https://learn.microsoft.com/azure/frontdoor/tier-migration) to allow you to migrate your existing `Front Door Classic` instances to the new `Front Door Standard/Premium` product tiers.
///
/// Manages an Azure Front Door (classic) Web Application Firewall Policy instance.
///
/// !> **Be Aware:** Azure is rolling out a breaking change on Friday 9th April 2021 which may cause issues with the CDN/FrontDoor resources. More information is available in this GitHub issue - however unfortunately this may necessitate a breaking change to the CDN and Front Door resources, more information will be posted in the GitHub issue as the necessary changes are identified.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleFirewallPolicy = firewall_policy::create(
///         "exampleFirewallPolicy",
///         FirewallPolicyArgs::builder()
///             .custom_block_response_body(
///                 "PGh0bWw+CjxoZWFkZXI+PHRpdGxlPkhlbGxvPC90aXRsZT48L2hlYWRlcj4KPGJvZHk+CkhlbGxvIHdvcmxkCjwvYm9keT4KPC9odG1sPg==",
///             )
///             .custom_block_response_status_code(403)
///             .custom_rules(
///                 vec![
///                     FirewallPolicyCustomRule::builder().action("Block").enabled(true)
///                     .matchConditions(vec![FirewallPolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24", "10.0.0.0/24",])
///                     .matchVariable("RemoteAddr").negationCondition(false)
///                     .operator("IPMatch").build_struct(),]).name("Rule1").priority(1)
///                     .rateLimitDurationInMinutes(1).rateLimitThreshold(10). type
///                     ("MatchRule").build_struct(), FirewallPolicyCustomRule::builder()
///                     .action("Block").enabled(true)
///                     .matchConditions(vec![FirewallPolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24",]).matchVariable("RemoteAddr")
///                     .negationCondition(false).operator("IPMatch").build_struct(),
///                     FirewallPolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["windows",]).matchVariable("RequestHeader")
///                     .negationCondition(false).operator("Contains").selector("UserAgent")
///                     .transforms(vec!["Lowercase", "Trim",]).build_struct(),])
///                     .name("Rule2").priority(2).rateLimitDurationInMinutes(1)
///                     .rateLimitThreshold(10). type ("MatchRule").build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .managed_rules(
///                 vec![
///                     FirewallPolicyManagedRule::builder()
///                     .exclusions(vec![FirewallPolicyManagedRuleExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("not_suspicious").build_struct(),])
///                     .overrides(vec![FirewallPolicyManagedRuleOverride::builder()
///                     .ruleGroupName("PHP")
///                     .rules(vec![FirewallPolicyManagedRuleOverrideRule::builder()
///                     .action("Block").enabled(false).ruleId("933100").build_struct(),])
///                     .build_struct(), FirewallPolicyManagedRuleOverride::builder()
///                     .exclusions(vec![FirewallPolicyManagedRuleOverrideExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("really_not_suspicious").build_struct(),])
///                     .ruleGroupName("SQLI")
///                     .rules(vec![FirewallPolicyManagedRuleOverrideRule::builder()
///                     .action("Block")
///                     .exclusions(vec![FirewallPolicyManagedRuleOverrideRuleExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("innocent").build_struct(),]).ruleId("942200")
///                     .build_struct(),]).build_struct(),]). type ("DefaultRuleSet")
///                     .version("1.0").build_struct(), FirewallPolicyManagedRule::builder().
///                     type ("Microsoft_BotManagerRuleSet").version("1.0").build_struct(),
///                 ],
///             )
///             .mode("Prevention")
///             .name("examplefdwafpolicy")
///             .redirect_url("https://www.contoso.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FrontDoor Web Application Firewall Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:frontdoor/firewallPolicy:FirewallPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Network/frontDoorWebApplicationFirewallPolicies/examplefdwafpolicy
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// If a `custom_rule` block's action type is `block`, this is the response body. The body must be specified in base64 encoding.
        #[builder(into, default)]
        pub custom_block_response_body: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// If a `custom_rule` block's action type is `block`, this is the response status code. Possible values are `200`, `403`, `405`, `406`, or `429`.
        #[builder(into, default)]
        pub custom_block_response_status_code: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// One or more `custom_rule` blocks as defined below.
        #[builder(into, default)]
        pub custom_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::frontdoor::FirewallPolicyCustomRule>>,
        >,
        /// Is the policy a enabled state or disabled state. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `managed_rule` blocks as defined below.
        #[builder(into, default)]
        pub managed_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRule>>,
        >,
        /// The firewall policy mode. Possible values are `Detection`, `Prevention`. Defaults to `Prevention`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If action type is redirect, this field represents redirect URL for the client.
        #[builder(into, default)]
        pub redirect_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// If a `custom_rule` block's action type is `block`, this is the response body. The body must be specified in base64 encoding.
        pub custom_block_response_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// If a `custom_rule` block's action type is `block`, this is the response status code. Possible values are `200`, `403`, `405`, `406`, or `429`.
        pub custom_block_response_status_code: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `custom_rule` blocks as defined below.
        pub custom_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::frontdoor::FirewallPolicyCustomRule>>,
        >,
        /// Is the policy a enabled state or disabled state. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Frontend Endpoints associated with this Front Door Web Application Firewall policy.
        pub frontend_endpoint_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Azure Region where this Front Door Firewall Policy exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// One or more `managed_rule` blocks as defined below.
        pub managed_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::frontdoor::FirewallPolicyManagedRule>>,
        >,
        /// The firewall policy mode. Possible values are `Detection`, `Prevention`. Defaults to `Prevention`.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If action type is redirect, this field represents redirect URL for the client.
        pub redirect_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
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
        args: FirewallPolicyArgs,
    ) -> FirewallPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_block_response_body_binding = args
            .custom_block_response_body
            .get_output(context)
            .get_inner();
        let custom_block_response_status_code_binding = args
            .custom_block_response_status_code
            .get_output(context)
            .get_inner();
        let custom_rules_binding = args.custom_rules.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let managed_rules_binding = args.managed_rules.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let redirect_url_binding = args.redirect_url.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:frontdoor/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customBlockResponseBody".into(),
                    value: &custom_block_response_body_binding,
                },
                register_interface::ObjectField {
                    name: "customBlockResponseStatusCode".into(),
                    value: &custom_block_response_status_code_binding,
                },
                register_interface::ObjectField {
                    name: "customRules".into(),
                    value: &custom_rules_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "managedRules".into(),
                    value: &managed_rules_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "redirectUrl".into(),
                    value: &redirect_url_binding,
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
        FirewallPolicyResult {
            custom_block_response_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customBlockResponseBody"),
            ),
            custom_block_response_status_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customBlockResponseStatusCode"),
            ),
            custom_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customRules"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frontend_endpoint_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendEndpointIds"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedRules"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            redirect_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redirectUrl"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
