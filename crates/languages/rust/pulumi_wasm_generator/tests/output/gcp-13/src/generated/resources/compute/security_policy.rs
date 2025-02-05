/// A Security Policy defines an IP blacklist or whitelist that protects load balanced Google Cloud services by denying or permitting traffic from specified IP ranges. For more information
/// see the [official documentation](https://cloud.google.com/armor/docs/configure-security-policies)
/// and the [API](https://cloud.google.com/compute/docs/reference/rest/beta/securityPolicies).
///
/// Security Policy is used by google_compute_backend_service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SecurityPolicy
///     properties:
///       name: my-policy
///       rules:
///         - action: deny(403)
///           priority: '1000'
///           match:
///             versionedExpr: SRC_IPS_V1
///             config:
///               srcIpRanges:
///                 - 9.9.9.0/24
///           description: Deny access to IPs in 9.9.9.0/24
///         - action: allow
///           priority: '2147483647'
///           match:
///             versionedExpr: SRC_IPS_V1
///             config:
///               srcIpRanges:
///                 - '*'
///           description: default rule
/// ```
///
/// ### With ReCAPTCHA Configuration Options
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:recaptcha:EnterpriseKey
///     properties:
///       displayName: display-name
///       labels:
///         label-one: value-one
///       project: my-project-name
///       webSettings:
///         integrationType: INVISIBLE
///         allowAllDomains: true
///         allowedDomains:
///           - localhost
///   policy:
///     type: gcp:compute:SecurityPolicy
///     properties:
///       name: my-policy
///       description: basic security policy
///       type: CLOUD_ARMOR
///       recaptchaOptionsConfig:
///         redirectSiteKey: ${primary.name}
/// ```
///
/// ### With Header Actions
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SecurityPolicy
///     properties:
///       name: my-policy
///       rules:
///         - action: allow
///           priority: '2147483647'
///           match:
///             versionedExpr: SRC_IPS_V1
///             config:
///               srcIpRanges:
///                 - '*'
///           description: default rule
///         - action: allow
///           priority: '1000'
///           match:
///             expr:
///               expression: request.path.matches("/login.html") && token.recaptcha_session.score < 0.2
///           headerAction:
///             requestHeadersToAdds:
///               - headerName: reCAPTCHA-Warning
///                 headerValue: high
///               - headerName: X-Resource
///                 headerValue: test
/// ```
///
/// ### With EnforceOnKey Value As Empty String
/// A scenario example that won't cause any conflict between `enforce_on_key` and `enforce_on_key_configs`, because `enforce_on_key` was specified as an empty string:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SecurityPolicy
///     properties:
///       name: '%s'
///       description: throttle rule with enforce_on_key_configs
///       rules:
///         - action: throttle
///           priority: '2147483647'
///           match:
///             versionedExpr: SRC_IPS_V1
///             config:
///               srcIpRanges:
///                 - '*'
///           description: default rule
///           rateLimitOptions:
///             conformAction: allow
///             exceedAction: redirect
///             enforceOnKey: ""
///             enforceOnKeyConfigs:
///               - enforceOnKeyType: IP
///             exceedRedirectOptions:
///               type: EXTERNAL_302
///               target: <https://www.example.com>
///             rateLimitThreshold:
///               count: 10
///               intervalSec: 60
/// ```
///
/// ## Import
///
/// Security policies can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/securityPolicies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, security policies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicy:SecurityPolicy default projects/{{project}}/global/securityPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicy:SecurityPolicy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/securityPolicy:SecurityPolicy default {{name}}
/// ```
///
pub mod security_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityPolicyArgs {
        /// Configuration for [Google Cloud Armor Adaptive Protection](https://cloud.google.com/armor/docs/adaptive-protection-overview?hl=en). Structure is documented below.
        #[builder(into, default)]
        pub adaptive_protection_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfig>,
        >,
        /// [Advanced Configuration Options](https://cloud.google.com/armor/docs/security-policy-overview#json-parsing).
        /// Structure is documented below.
        #[builder(into, default)]
        pub advanced_options_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyAdvancedOptionsConfig>,
        >,
        /// An optional description of this security policy. Max size is 2048.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the security policy.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// [reCAPTCHA Configuration Options](https://cloud.google.com/armor/docs/configure-security-policies?hl=en#use_a_manual_challenge_to_distinguish_between_human_or_automated_clients). Structure is documented below.
        #[builder(into, default)]
        pub recaptcha_options_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::SecurityPolicyRecaptchaOptionsConfig>,
        >,
        /// The set of rules that belong to this policy. There must always be a default
        /// rule (rule with priority 2147483647 and match "\*"). If no rules are provided when creating a
        /// security policy, a default rule with action "allow" will be added. Structure is documented below.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::SecurityPolicyRule>>,
        >,
        /// The type indicates the intended use of the security policy. This field can be set only at resource creation time.
        /// * `CLOUD_ARMOR` - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services.
        /// They filter requests before they hit the origin servers.
        /// * `CLOUD_ARMOR_EDGE` - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services
        /// (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage).
        /// They filter requests before the request is served from Google's cache.
        /// * `CLOUD_ARMOR_INTERNAL_SERVICE` - Cloud Armor internal service policies can be configured to filter HTTP requests targeting services
        /// managed by Traffic Director in a service mesh. They filter requests before the request is served from the application.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityPolicyResult {
        /// Configuration for [Google Cloud Armor Adaptive Protection](https://cloud.google.com/armor/docs/adaptive-protection-overview?hl=en). Structure is documented below.
        pub adaptive_protection_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfig>,
        >,
        /// [Advanced Configuration Options](https://cloud.google.com/armor/docs/security-policy-overview#json-parsing).
        /// Structure is documented below.
        pub advanced_options_config: pulumi_wasm_rust::Output<
            super::super::types::compute::SecurityPolicyAdvancedOptionsConfig,
        >,
        /// An optional description of this security policy. Max size is 2048.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The name of the security policy.
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// [reCAPTCHA Configuration Options](https://cloud.google.com/armor/docs/configure-security-policies?hl=en#use_a_manual_challenge_to_distinguish_between_human_or_automated_clients). Structure is documented below.
        pub recaptcha_options_config: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::SecurityPolicyRecaptchaOptionsConfig>,
        >,
        /// The set of rules that belong to this policy. There must always be a default
        /// rule (rule with priority 2147483647 and match "\*"). If no rules are provided when creating a
        /// security policy, a default rule with action "allow" will be added. Structure is documented below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::SecurityPolicyRule>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The type indicates the intended use of the security policy. This field can be set only at resource creation time.
        /// * `CLOUD_ARMOR` - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services.
        /// They filter requests before they hit the origin servers.
        /// * `CLOUD_ARMOR_EDGE` - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services
        /// (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage).
        /// They filter requests before the request is served from Google's cache.
        /// * `CLOUD_ARMOR_INTERNAL_SERVICE` - Cloud Armor internal service policies can be configured to filter HTTP requests targeting services
        /// managed by Traffic Director in a service mesh. They filter requests before the request is served from the application.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecurityPolicyArgs,
    ) -> SecurityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let adaptive_protection_config_binding = args
            .adaptive_protection_config
            .get_output(context)
            .get_inner();
        let advanced_options_config_binding = args
            .advanced_options_config
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let recaptcha_options_config_binding = args
            .recaptcha_options_config
            .get_output(context)
            .get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/securityPolicy:SecurityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adaptiveProtectionConfig".into(),
                    value: &adaptive_protection_config_binding,
                },
                register_interface::ObjectField {
                    name: "advancedOptionsConfig".into(),
                    value: &advanced_options_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "recaptchaOptionsConfig".into(),
                    value: &recaptcha_options_config_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecurityPolicyResult {
            adaptive_protection_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adaptiveProtectionConfig"),
            ),
            advanced_options_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedOptionsConfig"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            recaptcha_options_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recaptchaOptionsConfig"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
