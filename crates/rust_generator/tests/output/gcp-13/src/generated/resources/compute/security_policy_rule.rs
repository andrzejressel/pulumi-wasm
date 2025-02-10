/// A rule for the SecurityPolicy.
///
///
/// To get more information about SecurityPolicyRule, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/securityPolicies/addRule)
/// * How-to Guides
///     * [Creating global security policy rules](https://cloud.google.com/armor/docs/configure-security-policies)
///
/// ## Example Usage
///
/// ### Security Policy Rule Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = security_policy::create(
///         "default",
///         SecurityPolicyArgs::builder()
///             .description("basic global security policy")
///             .name("policyruletest")
///             .type_("CLOUD_ARMOR")
///             .build_struct(),
///     );
///     let policyRule = security_policy_rule::create(
///         "policyRule",
///         SecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule")
///             .match_(
///                 SecurityPolicyRuleMatch::builder()
///                     .config(
///                         SecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["10.10.0.0/16",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(100)
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Security Policy Rule Default Rule
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:SecurityPolicy
///     properties:
///       name: policyruletest
///       description: basic global security policy
///       type: CLOUD_ARMOR
///   defaultRule:
///     type: gcp:compute:SecurityPolicyRule
///     name: default_rule
///     properties:
///       securityPolicy: ${default.name}
///       description: default rule
///       action: deny
///       priority: '2147483647'
///       match:
///         versionedExpr: SRC_IPS_V1
///         config:
///           srcIpRanges:
///             - '*'
///   policyRule:
///     type: gcp:compute:SecurityPolicyRule
///     name: policy_rule
///     properties:
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
/// ### Security Policy Rule Multiple Rules
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = security_policy::create(
///         "default",
///         SecurityPolicyArgs::builder()
///             .description("basic global security policy")
///             .name("policywithmultiplerules")
///             .type_("CLOUD_ARMOR")
///             .build_struct(),
///     );
///     let policyRuleOne = security_policy_rule::create(
///         "policyRuleOne",
///         SecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule one")
///             .match_(
///                 SecurityPolicyRuleMatch::builder()
///                     .config(
///                         SecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["10.10.0.0/16",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(100)
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
///     let policyRuleTwo = security_policy_rule::create(
///         "policyRuleTwo",
///         SecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .description("new rule two")
///             .match_(
///                 SecurityPolicyRuleMatch::builder()
///                     .config(
///                         SecurityPolicyRuleMatchConfig::builder()
///                             .srcIpRanges(vec!["192.168.0.0/16", "10.0.0.0/8",])
///                             .build_struct(),
///                     )
///                     .versionedExpr("SRC_IPS_V1")
///                     .build_struct(),
///             )
///             .preview(true)
///             .priority(101)
///             .security_policy("${default.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SecurityPolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/securityPolicies/{{security_policy}}/priority/{{priority}}`
///
/// * `{{project}}/{{security_policy}}/{{priority}}`
///
/// * `{{security_policy}}/{{priority}}`
///
/// When using the `pulumi import` command, SecurityPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicyRule:SecurityPolicyRule default projects/{{project}}/global/securityPolicies/{{security_policy}}/priority/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicyRule:SecurityPolicyRule default {{project}}/{{security_policy}}/{{priority}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicyRule:SecurityPolicyRule default {{security_policy}}/{{priority}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_policy_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityPolicyRuleArgs {
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
        /// Optional, additional actions that are performed on headers. This field is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// Structure is documented below.
        #[builder(into, default)]
        pub header_action: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyRuleHeaderAction>,
        >,
        /// A match condition that incoming traffic is evaluated against.
        /// If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        #[builder(into, default)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyRuleMatch>,
        >,
        /// Preconfigured WAF configuration to be applied for the rule.
        /// If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
        /// Structure is documented below.
        #[builder(into, default)]
        pub preconfigured_waf_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfig,
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
            Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptions>,
        >,
        /// Parameters defining the redirect action. Cannot be specified for any other actions. This field is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// Structure is documented below.
        #[builder(into, default)]
        pub redirect_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyRuleRedirectOptions>,
        >,
        /// The name of the security policy this rule belongs to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityPolicyRuleResult {
        /// The Action to perform when the rule is matched. The following are the valid actions:
        /// * allow: allow access to target.
        /// * deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.
        /// * rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.
        /// * redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// * throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional, additional actions that are performed on headers. This field is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// Structure is documented below.
        pub header_action: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SecurityPolicyRuleHeaderAction>,
        >,
        /// A match condition that incoming traffic is evaluated against.
        /// If it evaluates to true, the corresponding 'action' is enforced.
        /// Structure is documented below.
        pub match_: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SecurityPolicyRuleMatch>,
        >,
        /// Preconfigured WAF configuration to be applied for the rule.
        /// If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
        /// Structure is documented below.
        pub preconfigured_waf_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfig,
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
            Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptions>,
        >,
        /// Parameters defining the redirect action. Cannot be specified for any other actions. This field is only supported in Global Security Policies of type CLOUD_ARMOR.
        /// Structure is documented below.
        pub redirect_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SecurityPolicyRuleRedirectOptions>,
        >,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityPolicyRuleArgs,
    ) -> SecurityPolicyRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let header_action_binding = args.header_action.get_output(context);
        let match__binding = args.match_.get_output(context);
        let preconfigured_waf_config_binding = args
            .preconfigured_waf_config
            .get_output(context);
        let preview_binding = args.preview.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let project_binding = args.project.get_output(context);
        let rate_limit_options_binding = args.rate_limit_options.get_output(context);
        let redirect_options_binding = args.redirect_options.get_output(context);
        let security_policy_binding = args.security_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/securityPolicyRule:SecurityPolicyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headerAction".into(),
                    value: header_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "match".into(),
                    value: match__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preconfiguredWafConfig".into(),
                    value: preconfigured_waf_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preview".into(),
                    value: preview_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rateLimitOptions".into(),
                    value: rate_limit_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectOptions".into(),
                    value: redirect_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicy".into(),
                    value: security_policy_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityPolicyRuleResult {
            action: o.get_field("action"),
            description: o.get_field("description"),
            header_action: o.get_field("headerAction"),
            match_: o.get_field("match"),
            preconfigured_waf_config: o.get_field("preconfiguredWafConfig"),
            preview: o.get_field("preview"),
            priority: o.get_field("priority"),
            project: o.get_field("project"),
            rate_limit_options: o.get_field("rateLimitOptions"),
            redirect_options: o.get_field("redirectOptions"),
            security_policy: o.get_field("securityPolicy"),
        }
    }
}
