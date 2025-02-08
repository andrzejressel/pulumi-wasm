/// ## Example Usage
///
/// ### Region Security Policy Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_security_policy::create(
///         "default",
///         RegionSecurityPolicyArgs::builder()
///             .description("basic region security policy")
///             .name("policyruletest")
///             .region("us-west2")
///             .type_("CLOUD_ARMOR")
///             .build_struct(),
///     );
///     let policyRule = region_security_policy_rule::create(
///         "policyRule",
///         RegionSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule")
///             .match_(
///                 RegionSecurityPolicyRuleMatch::builder()
///                     .config(
///                         RegionSecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["10.10.0.0/16",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(100)
///             .region("us-west2")
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Security Policy Rule Multiple Rules
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_security_policy::create(
///         "default",
///         RegionSecurityPolicyArgs::builder()
///             .description("basic region security policy")
///             .name("policywithmultiplerules")
///             .region("us-west2")
///             .type_("CLOUD_ARMOR")
///             .build_struct(),
///     );
///     let policyRuleOne = region_security_policy_rule::create(
///         "policyRuleOne",
///         RegionSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule one")
///             .match_(
///                 RegionSecurityPolicyRuleMatch::builder()
///                     .config(
///                         RegionSecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["10.10.0.0/16",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(100)
///             .region("us-west2")
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
///     let policyRuleTwo = region_security_policy_rule::create(
///         "policyRuleTwo",
///         RegionSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule two")
///             .match_(
///                 RegionSecurityPolicyRuleMatch::builder()
///                     .config(
///                         RegionSecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["192.168.0.0/16", "10.0.0.0/8",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(101)
///             .region("us-west2")
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Security Policy Rule Default Rule
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       region: us-west2
///       name: policywithdefaultrule
///       description: basic region security policy
///       type: CLOUD_ARMOR
///   defaultRule:
///     type: gcp:compute:RegionSecurityPolicyRule
///     name: default_rule
///     properties:
///       region: us-west2
///       securityPolicy: ${default.name}
///       description: new rule
///       action: deny
///       priority: '2147483647'
///       match:
///         versionedExpr: SRC_IPS_V1
///         config:
///           srcIpRanges:
///             - '*'
///   policyRule:
///     type: gcp:compute:RegionSecurityPolicyRule
///     name: policy_rule
///     properties:
///       region: us-west2
///       securityPolicy: ${default.name}
///       description: new rule
///       priority: 100
///       match:
///         versionedExpr: SRC_IPS_V1
///         config:
///           srcIpRanges:
///             - 10.10.0.0/16
///       action: allow
///       preview: true
/// ```
/// ### Region Security Policy Rule With Preconfigured Waf Config
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_security_policy::create(
///         "default",
///         RegionSecurityPolicyArgs::builder()
///             .description("basic region security policy")
///             .name("policyruletest")
///             .region("asia-southeast1")
///             .type_("CLOUD_ARMOR")
///             .build_struct(),
///     );
///     let policyRule = region_security_policy_rule::create(
///         "policyRule",
///         RegionSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule")
///             .match_(
///                 RegionSecurityPolicyRuleMatch::builder()
///                     .config(
///                         RegionSecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["10.10.0.0/16",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preconfigured_waf_config(
///                 RegionSecurityPolicyRulePreconfiguredWafConfig::builder()
///                     .exclusions(
///                         vec![
///                             RegionSecurityPolicyRulePreconfiguredWafConfigExclusion::builder()
///                             .requestUris(vec![RegionSecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri::builder()
///                             .operator("STARTS_WITH").value("/admin").build_struct(),])
///                             .targetRuleSet("rce-stable").build_struct(),
///                             RegionSecurityPolicyRulePreconfiguredWafConfigExclusion::builder()
///                             .requestQueryParams(vec![RegionSecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam::builder()
///                             .operator("CONTAINS").value("password").build_struct(),
///                             RegionSecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam::builder()
///                             .operator("STARTS_WITH").value("freeform").build_struct(),
///                             RegionSecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam::builder()
///                             .operator("EQUALS").value("description").build_struct(),])
///                             .targetRuleIds(vec!["owasp-crs-v030001-id941330-xss",
///                             "owasp-crs-v030001-id941340-xss",])
///                             .targetRuleSet("xss-stable").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(100)
///             .region("asia-southeast1")
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Security Policy Rule With Network Match
///
///
/// ```yaml
/// resources:
///   # First activate advanced network DDoS protection for the desired region
///   policyddosprotection:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       region: us-west2
///       name: policyddosprotection
///       description: policy for activating network DDoS protection for the desired region
///       type: CLOUD_ARMOR_NETWORK
///       ddosProtectionConfig:
///         ddosProtection: ADVANCED_PREVIEW
///   edgeSecService:
///     type: gcp:compute:NetworkEdgeSecurityService
///     name: edge_sec_service
///     properties:
///       region: us-west2
///       name: edgesecservice
///       description: linking policy to edge security service
///       securityPolicy: ${policyddosprotection.selfLink}
///   # Add the desired policy and custom rule.
///   policynetworkmatch:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       region: us-west2
///       name: policyfornetworkmatch
///       description: region security policy for network match
///       type: CLOUD_ARMOR_NETWORK
///       userDefinedFields:
///         - name: SIG1_AT_0
///           base: TCP
///           offset: 8
///           size: 2
///           mask: 0x8F00
///     options:
///       dependsOn:
///         - ${edgeSecService}
///   policyRuleNetworkMatch:
///     type: gcp:compute:RegionSecurityPolicyRule
///     name: policy_rule_network_match
///     properties:
///       region: us-west2
///       securityPolicy: ${policynetworkmatch.name}
///       description: custom rule for network match
///       priority: 100
///       networkMatch:
///         srcIpRanges:
///           - 10.10.0.0/16
///         userDefinedFields:
///           - name: SIG1_AT_0
///             values:
///               - 0x8F00
///       action: allow
///       preview: true
/// ```
///
/// ## Import
///
/// RegionSecurityPolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/securityPolicies/{{security_policy}}/priority/{{priority}}`
///
/// * `{{project}}/{{region}}/{{security_policy}}/{{priority}}`
///
/// * `{{region}}/{{security_policy}}/{{priority}}`
///
/// * `{{security_policy}}/{{priority}}`
///
/// When using the `pulumi import` command, RegionSecurityPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicyRule:RegionSecurityPolicyRule default projects/{{project}}/regions/{{region}}/securityPolicies/{{security_policy}}/priority/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicyRule:RegionSecurityPolicyRule default {{project}}/{{region}}/{{security_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicyRule:RegionSecurityPolicyRule default {{region}}/{{security_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicyRule:RegionSecurityPolicyRule default {{security_policy}}/{{priority}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_security_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionSecurityPolicyRuleArgs {
        /// The Action to perform when the rule is matched. The following are the valid actions:
        /// * allow: allow access to target.
        /// * deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.
        /// * rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.
        /// * redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// * throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A match condition that incoming traffic is evaluated against.
        /// If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        #[builder(into, default)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionSecurityPolicyRuleMatch>,
        >,
        /// A match condition that incoming packets are evaluated against for CLOUD_ARMOR_NETWORK security policies. If it matches, the corresponding 'action' is enforced.
        /// The match criteria for a rule consists of built-in match fields (like 'srcIpRanges') and potentially multiple user-defined match fields ('userDefinedFields').
        /// Field values may be extracted directly from the packet or derived from it (e.g. 'srcRegionCodes'). Some fields may not be present in every packet (e.g. 'srcPorts'). A user-defined field is only present if the base header is found in the packet and the entire field is in bounds.
        /// Each match field may specify which values can match it, listing one or more ranges, prefixes, or exact values that are considered a match for the field. A field value must be present in order to match a specified match field. If no match values are specified for a match field, then any field value is considered to match it, and it's not required to be present. For strings specifying '*' is also equivalent to match all.
        /// For a packet to match a rule, all specified match fields must match the corresponding field values derived from the packet.
        /// Example:
        /// networkMatch: srcIpRanges: - "192.0.2.0/24" - "198.51.100.0/24" userDefinedFields: - name: "ipv4_fragment_offset" values: - "1-0x1fff"
        /// The above match condition matches packets with a source IP in 192.0.2.0/24 or 198.51.100.0/24 and a user-defined field named "ipv4_fragment_offset" with a value between 1 and 0x1fff inclusive
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_match: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatch>,
        >,
        /// Preconfigured WAF configuration to be applied for the rule.
        /// If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
        /// Structure is documented below.
        #[builder(into, default)]
        pub preconfigured_waf_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionSecurityPolicyRulePreconfiguredWafConfig,
            >,
        >,
        /// If set to true, the specified action is not enforced.
        #[builder(into, default)]
        pub preview: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Must be specified if the action is "rate_based_ban" or "throttle". Cannot be specified for any other actions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rate_limit_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptions,
            >,
        >,
        /// The Region in which the created Region Security Policy rule should reside.
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the security policy this rule belongs to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionSecurityPolicyRuleResult {
        /// The Action to perform when the rule is matched. The following are the valid actions:
        /// * allow: allow access to target.
        /// * deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.
        /// * rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.
        /// * redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// * throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A match condition that incoming traffic is evaluated against.
        /// If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        pub match_: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionSecurityPolicyRuleMatch>,
        >,
        /// A match condition that incoming packets are evaluated against for CLOUD_ARMOR_NETWORK security policies. If it matches, the corresponding 'action' is enforced.
        /// The match criteria for a rule consists of built-in match fields (like 'srcIpRanges') and potentially multiple user-defined match fields ('userDefinedFields').
        /// Field values may be extracted directly from the packet or derived from it (e.g. 'srcRegionCodes'). Some fields may not be present in every packet (e.g. 'srcPorts'). A user-defined field is only present if the base header is found in the packet and the entire field is in bounds.
        /// Each match field may specify which values can match it, listing one or more ranges, prefixes, or exact values that are considered a match for the field. A field value must be present in order to match a specified match field. If no match values are specified for a match field, then any field value is considered to match it, and it's not required to be present. For strings specifying '*' is also equivalent to match all.
        /// For a packet to match a rule, all specified match fields must match the corresponding field values derived from the packet.
        /// Example:
        /// networkMatch: srcIpRanges: - "192.0.2.0/24" - "198.51.100.0/24" userDefinedFields: - name: "ipv4_fragment_offset" values: - "1-0x1fff"
        /// The above match condition matches packets with a source IP in 192.0.2.0/24 or 198.51.100.0/24 and a user-defined field named "ipv4_fragment_offset" with a value between 1 and 0x1fff inclusive
        /// Structure is documented below.
        pub network_match: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatch>,
        >,
        /// Preconfigured WAF configuration to be applied for the rule.
        /// If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
        /// Structure is documented below.
        pub preconfigured_waf_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionSecurityPolicyRulePreconfiguredWafConfig,
            >,
        >,
        /// If set to true, the specified action is not enforced.
        pub preview: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An integer indicating the priority of a rule in the list.
        /// The priority must be a positive value between 0 and 2147483647.
        /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Must be specified if the action is "rate_based_ban" or "throttle". Cannot be specified for any other actions.
        /// Structure is documented below.
        pub rate_limit_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptions,
            >,
        >,
        /// The Region in which the created Region Security Policy rule should reside.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The name of the security policy this rule belongs to.
        ///
        ///
        /// - - -
        pub security_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionSecurityPolicyRuleArgs,
    ) -> RegionSecurityPolicyRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let match__binding = args.match_.get_output(context).get_inner();
        let network_match_binding = args.network_match.get_output(context).get_inner();
        let preconfigured_waf_config_binding = args
            .preconfigured_waf_config
            .get_output(context)
            .get_inner();
        let preview_binding = args.preview.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let rate_limit_options_binding = args
            .rate_limit_options
            .get_output(context)
            .get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let security_policy_binding = args
            .security_policy
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionSecurityPolicyRule:RegionSecurityPolicyRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "match".into(),
                    value: &match__binding,
                },
                register_interface::ObjectField {
                    name: "networkMatch".into(),
                    value: &network_match_binding,
                },
                register_interface::ObjectField {
                    name: "preconfiguredWafConfig".into(),
                    value: &preconfigured_waf_config_binding,
                },
                register_interface::ObjectField {
                    name: "preview".into(),
                    value: &preview_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rateLimitOptions".into(),
                    value: &rate_limit_options_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionSecurityPolicyRuleResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            match_: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("match"),
            ),
            network_match: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkMatch"),
            ),
            preconfigured_waf_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preconfiguredWafConfig"),
            ),
            preview: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preview"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            rate_limit_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rateLimitOptions"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicy"),
            ),
        }
    }
}
