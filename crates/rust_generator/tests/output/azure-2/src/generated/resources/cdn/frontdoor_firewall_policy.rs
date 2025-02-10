/// Manages a Front Door (standard/premium) Firewall Policy instance.
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
///                     .matchValues(vec!["10.0.1.0/24", "10.0.0.0/24",])
///                     .matchVariable("RemoteAddr").negationCondition(false)
///                     .operator("IPMatch").build_struct(),]).name("Rule1").priority(1)
///                     .rateLimitDurationInMinutes(1).rateLimitThreshold(10). type
///                     ("MatchRule").build_struct(),
///                     FrontdoorFirewallPolicyCustomRule::builder().action("Block")
///                     .enabled(true)
///                     .matchConditions(vec![FrontdoorFirewallPolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24",]).matchVariable("RemoteAddr")
///                     .negationCondition(false).operator("IPMatch").build_struct(),
///                     FrontdoorFirewallPolicyCustomRuleMatchCondition::builder()
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
///                     FrontdoorFirewallPolicyManagedRule::builder()
///                     .exclusions(vec![FrontdoorFirewallPolicyManagedRuleExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("not_suspicious").build_struct(),])
///                     .overrides(vec![FrontdoorFirewallPolicyManagedRuleOverride::builder()
///                     .ruleGroupName("PHP")
///                     .rules(vec![FrontdoorFirewallPolicyManagedRuleOverrideRule::builder()
///                     .action("Block").enabled(false).ruleId("933100").build_struct(),])
///                     .build_struct(),
///                     FrontdoorFirewallPolicyManagedRuleOverride::builder()
///                     .exclusions(vec![FrontdoorFirewallPolicyManagedRuleOverrideExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("really_not_suspicious").build_struct(),])
///                     .ruleGroupName("SQLI")
///                     .rules(vec![FrontdoorFirewallPolicyManagedRuleOverrideRule::builder()
///                     .action("Block")
///                     .exclusions(vec![FrontdoorFirewallPolicyManagedRuleOverrideRuleExclusion::builder()
///                     .matchVariable("QueryStringArgNames").operator("Equals")
///                     .selector("innocent").build_struct(),]).ruleId("942200")
///                     .build_struct(),]).build_struct(),]). type ("DefaultRuleSet")
///                     .version("1.0").build_struct(),
///                     FrontdoorFirewallPolicyManagedRule::builder().action("Log"). type
///                     ("Microsoft_BotManagerRuleSet").version("1.0").build_struct(),
///                 ],
///             )
///             .mode("Prevention")
///             .name("examplecdnfdwafpolicy")
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
///             .sku_name("Premium_AzureFrontDoor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Front Door Firewall Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorFirewallPolicy:FrontdoorFirewallPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/frontDoorWebApplicationFirewallPolicies/firewallPolicy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod frontdoor_firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorFirewallPolicyArgs {
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
            Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyCustomRule>>,
        >,
        /// Is the Front Door Firewall Policy enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `managed_rule` blocks as defined below.
        #[builder(into, default)]
        pub managed_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRule>>,
        >,
        /// The Front Door Firewall Policy mode. Possible values are `Detection`, `Prevention`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If action type is redirect, this field represents redirect URL for the client.
        #[builder(into, default)]
        pub redirect_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should policy managed rules inspect the request body content? Defaults to `true`.
        ///
        /// > **NOTE:** When run in `Detection` mode, the Front Door Firewall Policy doesn't take any other actions other than monitoring and logging the request and its matched Front Door Rule to the Web Application Firewall logs.
        #[builder(into, default)]
        pub request_body_check_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The sku's pricing tier for this Front Door Firewall Policy. Possible values include `Standard_AzureFrontDoor` or `Premium_AzureFrontDoor`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `Standard_AzureFrontDoor` Front Door Firewall Policy sku may contain `custom` rules only. The `Premium_AzureFrontDoor` Front Door Firewall Policy skus may contain both `custom` and `managed` rules.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Front Door Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorFirewallPolicyResult {
        /// If a `custom_rule` block's action type is `block`, this is the response body. The body must be specified in base64 encoding.
        pub custom_block_response_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// If a `custom_rule` block's action type is `block`, this is the response status code. Possible values are `200`, `403`, `405`, `406`, or `429`.
        pub custom_block_response_status_code: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `custom_rule` blocks as defined below.
        pub custom_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyCustomRule>>,
        >,
        /// Is the Front Door Firewall Policy enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Front Door Profiles frontend endpoints associated with this Front Door Firewall Policy.
        pub frontend_endpoint_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more `managed_rule` blocks as defined below.
        pub managed_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRule>>,
        >,
        /// The Front Door Firewall Policy mode. Possible values are `Detection`, `Prevention`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If action type is redirect, this field represents redirect URL for the client.
        pub redirect_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should policy managed rules inspect the request body content? Defaults to `true`.
        ///
        /// > **NOTE:** When run in `Detection` mode, the Front Door Firewall Policy doesn't take any other actions other than monitoring and logging the request and its matched Front Door Rule to the Web Application Firewall logs.
        pub request_body_check_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The sku's pricing tier for this Front Door Firewall Policy. Possible values include `Standard_AzureFrontDoor` or `Premium_AzureFrontDoor`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `Standard_AzureFrontDoor` Front Door Firewall Policy sku may contain `custom` rules only. The `Premium_AzureFrontDoor` Front Door Firewall Policy skus may contain both `custom` and `managed` rules.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Front Door Firewall Policy.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FrontdoorFirewallPolicyArgs,
    ) -> FrontdoorFirewallPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_block_response_body_binding = args
            .custom_block_response_body
            .get_output(context);
        let custom_block_response_status_code_binding = args
            .custom_block_response_status_code
            .get_output(context);
        let custom_rules_binding = args.custom_rules.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let managed_rules_binding = args.managed_rules.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let redirect_url_binding = args.redirect_url.get_output(context);
        let request_body_check_enabled_binding = args
            .request_body_check_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorFirewallPolicy:FrontdoorFirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customBlockResponseBody".into(),
                    value: custom_block_response_body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customBlockResponseStatusCode".into(),
                    value: custom_block_response_status_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customRules".into(),
                    value: custom_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedRules".into(),
                    value: managed_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectUrl".into(),
                    value: redirect_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestBodyCheckEnabled".into(),
                    value: request_body_check_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrontdoorFirewallPolicyResult {
            custom_block_response_body: o.get_field("customBlockResponseBody"),
            custom_block_response_status_code: o
                .get_field("customBlockResponseStatusCode"),
            custom_rules: o.get_field("customRules"),
            enabled: o.get_field("enabled"),
            frontend_endpoint_ids: o.get_field("frontendEndpointIds"),
            managed_rules: o.get_field("managedRules"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            redirect_url: o.get_field("redirectUrl"),
            request_body_check_enabled: o.get_field("requestBodyCheckEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
