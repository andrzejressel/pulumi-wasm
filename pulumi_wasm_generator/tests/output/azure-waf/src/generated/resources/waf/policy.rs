/// Manages a Azure Web Application Firewall Policy instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let examplePolicy = policy::create(
///         "examplePolicy",
///         PolicyArgs::builder()
///             .custom_rules(
///                 vec![
///                     PolicyCustomRule::builder().action("Block")
///                     .matchConditions(vec![PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24", "10.0.0.0/24",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .variableName("RemoteAddr").build_struct(),])
///                     .negationCondition(false).operator("IPMatch").build_struct(),])
///                     .name("Rule1").priority(1).ruleType("MatchRule").build_struct(),
///                     PolicyCustomRule::builder().action("Block")
///                     .matchConditions(vec![PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["192.168.1.0/24",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .variableName("RemoteAddr").build_struct(),])
///                     .negationCondition(false).operator("IPMatch").build_struct(),
///                     PolicyCustomRuleMatchCondition::builder()
///                     .matchValues(vec!["Windows",])
///                     .matchVariables(vec![PolicyCustomRuleMatchConditionMatchVariable::builder()
///                     .selector("UserAgent").variableName("RequestHeaders")
///                     .build_struct(),]).negationCondition(false).operator("Contains")
///                     .build_struct(),]).name("Rule2").priority(2).ruleType("MatchRule")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .managed_rules(
///                 PolicyManagedRules::builder()
///                     .exclusions(
///                         vec![
///                             PolicyManagedRulesExclusion::builder()
///                             .matchVariable("RequestHeaderNames")
///                             .selector("x-company-secret-header")
///                             .selectorMatchOperator("Equals").build_struct(),
///                             PolicyManagedRulesExclusion::builder()
///                             .matchVariable("RequestCookieNames").selector("too-tasty")
///                             .selectorMatchOperator("EndsWith").build_struct(),
///                         ],
///                     )
///                     .managedRuleSets(
///                         vec![
///                             PolicyManagedRulesManagedRuleSet::builder()
///                             .ruleGroupOverrides(vec![PolicyManagedRulesManagedRuleSetRuleGroupOverride::builder()
///                             .ruleGroupName("REQUEST-920-PROTOCOL-ENFORCEMENT")
///                             .rules(vec![PolicyManagedRulesManagedRuleSetRuleGroupOverrideRule::builder()
///                             .action("Log").enabled(true).id("920300").build_struct(),
///                             PolicyManagedRulesManagedRuleSetRuleGroupOverrideRule::builder()
///                             .action("Block").enabled(true).id("920440").build_struct(),])
///                             .build_struct(),]). type ("OWASP").version("3.2")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example-wafpolicy")
///             .policy_settings(
///                 PolicyPolicySettings::builder()
///                     .enabled(true)
///                     .fileUploadLimitInMb(100)
///                     .maxRequestBodySizeInKb(128)
///                     .mode("Prevention")
///                     .requestBodyCheck(true)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Web Application Firewall Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:waf/policy:Policy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Network/applicationGatewayWebApplicationFirewallPolicies/example-wafpolicy
/// ```
///
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// One or more `custom_rules` blocks as defined below.
        #[builder(into, default)]
        pub custom_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::PolicyCustomRule>>,
        >,
        /// Resource location. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `managed_rules` blocks as defined below.
        #[builder(into)]
        pub managed_rules: pulumi_wasm_rust::Output<
            super::super::types::waf::PolicyManagedRules,
        >,
        /// The name of the policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `policy_settings` block as defined below.
        #[builder(into, default)]
        pub policy_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::waf::PolicyPolicySettings>,
        >,
        /// The name of the resource group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// One or more `custom_rules` blocks as defined below.
        pub custom_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::PolicyCustomRule>>,
        >,
        /// A list of HTTP Listener IDs from an `azure.network.ApplicationGateway`.
        pub http_listener_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Resource location. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `managed_rules` blocks as defined below.
        pub managed_rules: pulumi_wasm_rust::Output<
            super::super::types::waf::PolicyManagedRules,
        >,
        /// The name of the policy. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of URL Path Map Path Rule IDs from an `azure.network.ApplicationGateway`.
        pub path_based_rule_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `policy_settings` block as defined below.
        pub policy_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::waf::PolicyPolicySettings>,
        >,
        /// The name of the resource group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Web Application Firewall Policy.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_rules_binding = args.custom_rules.get_inner();
        let location_binding = args.location.get_inner();
        let managed_rules_binding = args.managed_rules.get_inner();
        let name_binding = args.name.get_inner();
        let policy_settings_binding = args.policy_settings.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:waf/policy:Policy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customRules".into(),
                    value: &custom_rules_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedRules".into(),
                    value: &managed_rules_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policySettings".into(),
                    value: &policy_settings_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "customRules".into(),
                },
                register_interface::ResultField {
                    name: "httpListenerIds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedRules".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pathBasedRuleIds".into(),
                },
                register_interface::ResultField {
                    name: "policySettings".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyResult {
            custom_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRules").unwrap(),
            ),
            http_listener_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpListenerIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedRules").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path_based_rule_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathBasedRuleIds").unwrap(),
            ),
            policy_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policySettings").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}