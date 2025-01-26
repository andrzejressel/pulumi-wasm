/// The GatewaySecurityPolicyRule resource is in a nested collection within a GatewaySecurityPolicy and represents
/// a traffic matching condition and associated action to perform.
///
///
/// To get more information about GatewaySecurityPolicyRule, see:
///
/// * [API documentation](https://cloud.google.com/secure-web-proxy/docs/reference/network-security/rest/v1/projects.locations.gatewaySecurityPolicies.rules)
///
/// ## Example Usage
///
/// ### Network Security Gateway Security Policy Rules Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = gateway_security_policy::create(
///         "default",
///         GatewaySecurityPolicyArgs::builder()
///             .description(
///                 "gateway security policy created to be used as reference by the rule.",
///             )
///             .location("us-central1")
///             .name("my-gateway-security-policy")
///             .build_struct(),
///     );
///     let defaultGatewaySecurityPolicyRule = gateway_security_policy_rule::create(
///         "defaultGatewaySecurityPolicyRule",
///         GatewaySecurityPolicyRuleArgs::builder()
///             .basic_profile("ALLOW")
///             .description("my description")
///             .enabled(true)
///             .gateway_security_policy("${default.name}")
///             .location("us-central1")
///             .name("my-gateway-security-policy-rule")
///             .priority(0)
///             .session_matcher("host() == 'example.com'")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Security Gateway Security Policy Rules Advanced
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = gateway_security_policy::create(
///         "default",
///         GatewaySecurityPolicyArgs::builder()
///             .description(
///                 "gateway security policy created to be used as reference by the rule.",
///             )
///             .location("us-central1")
///             .name("my-gateway-security-policy")
///             .build_struct(),
///     );
///     let defaultGatewaySecurityPolicyRule = gateway_security_policy_rule::create(
///         "defaultGatewaySecurityPolicyRule",
///         GatewaySecurityPolicyRuleArgs::builder()
///             .application_matcher("request.method == 'POST'")
///             .basic_profile("ALLOW")
///             .description("my description")
///             .enabled(true)
///             .gateway_security_policy("${default.name}")
///             .location("us-central1")
///             .name("my-gateway-security-policy-rule")
///             .priority(0)
///             .session_matcher("host() == 'example.com'")
///             .tls_inspection_enabled(false)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GatewaySecurityPolicyRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/gatewaySecurityPolicies/{{gateway_security_policy}}/rules/{{name}}`
///
/// * `{{project}}/{{location}}/{{gateway_security_policy}}/{{name}}`
///
/// * `{{location}}/{{gateway_security_policy}}/{{name}}`
///
/// When using the `pulumi import` command, GatewaySecurityPolicyRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicyRule:GatewaySecurityPolicyRule default projects/{{project}}/locations/{{location}}/gatewaySecurityPolicies/{{gateway_security_policy}}/rules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicyRule:GatewaySecurityPolicyRule default {{project}}/{{location}}/{{gateway_security_policy}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicyRule:GatewaySecurityPolicyRule default {{location}}/{{gateway_security_policy}}/{{name}}
/// ```
///
pub mod gateway_security_policy_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewaySecurityPolicyRuleArgs {
        /// CEL expression for matching on L7/application level criteria.
        #[builder(into, default)]
        pub application_matcher: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Profile which tells what the primitive action should be. Possible values are: * ALLOW * DENY.
        /// Possible values are: `BASIC_PROFILE_UNSPECIFIED`, `ALLOW`, `DENY`.
        #[builder(into)]
        pub basic_profile: pulumi_wasm_rust::InputOrOutput<String>,
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the rule is enforced.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The name of the gatewat security policy this rule belongs to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub gateway_security_policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the gateway security policy.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule}
        /// rule should match the pattern: (^a-z?$).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Priority of the rule. Lower number corresponds to higher precedence.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CEL expression for matching on session criteria.
        #[builder(into)]
        pub session_matcher: pulumi_wasm_rust::InputOrOutput<String>,
        /// Flag to enable TLS inspection of traffic matching on. Can only be true if the
        /// parent GatewaySecurityPolicy references a TLSInspectionConfig.
        #[builder(into, default)]
        pub tls_inspection_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GatewaySecurityPolicyRuleResult {
        /// CEL expression for matching on L7/application level criteria.
        pub application_matcher: pulumi_wasm_rust::Output<Option<String>>,
        /// Profile which tells what the primitive action should be. Possible values are: * ALLOW * DENY.
        /// Possible values are: `BASIC_PROFILE_UNSPECIFIED`, `ALLOW`, `DENY`.
        pub basic_profile: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the rule is enforced.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the gatewat security policy this rule belongs to.
        ///
        ///
        /// - - -
        pub gateway_security_policy: pulumi_wasm_rust::Output<String>,
        /// The location of the gateway security policy.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule}
        /// rule should match the pattern: (^a-z?$).
        pub name: pulumi_wasm_rust::Output<String>,
        /// Priority of the rule. Lower number corresponds to higher precedence.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// CEL expression for matching on session criteria.
        pub session_matcher: pulumi_wasm_rust::Output<String>,
        /// Flag to enable TLS inspection of traffic matching on. Can only be true if the
        /// parent GatewaySecurityPolicy references a TLSInspectionConfig.
        pub tls_inspection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timestamp when the resource was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GatewaySecurityPolicyRuleArgs,
    ) -> GatewaySecurityPolicyRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_matcher_binding = args
            .application_matcher
            .get_output(context)
            .get_inner();
        let basic_profile_binding = args.basic_profile.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let gateway_security_policy_binding = args
            .gateway_security_policy
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let session_matcher_binding = args
            .session_matcher
            .get_output(context)
            .get_inner();
        let tls_inspection_enabled_binding = args
            .tls_inspection_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/gatewaySecurityPolicyRule:GatewaySecurityPolicyRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationMatcher".into(),
                    value: &application_matcher_binding,
                },
                register_interface::ObjectField {
                    name: "basicProfile".into(),
                    value: &basic_profile_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "gatewaySecurityPolicy".into(),
                    value: &gateway_security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "sessionMatcher".into(),
                    value: &session_matcher_binding,
                },
                register_interface::ObjectField {
                    name: "tlsInspectionEnabled".into(),
                    value: &tls_inspection_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationMatcher".into(),
                },
                register_interface::ResultField {
                    name: "basicProfile".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "gatewaySecurityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "sessionMatcher".into(),
                },
                register_interface::ResultField {
                    name: "tlsInspectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GatewaySecurityPolicyRuleResult {
            application_matcher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationMatcher").unwrap(),
            ),
            basic_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicProfile").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            gateway_security_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewaySecurityPolicy").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            session_matcher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionMatcher").unwrap(),
            ),
            tls_inspection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsInspectionEnabled").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
