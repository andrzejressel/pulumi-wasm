/// Provides a Cloudflare Teams rule resource. Teams rules comprise secure web gateway policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_gateway_policy::create(
///         "example",
///         ZeroTrustGatewayPolicyArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .action("block")
///             .description("desc")
///             .filters(vec!["http",])
///             .name("office")
///             .precedence(1)
///             .rule_settings(
///                 ZeroTrustGatewayPolicyRuleSettings::builder()
///                     .blockPageEnabled(true)
///                     .blockPageReason("access not permitted")
///                     .build_struct(),
///             )
///             .traffic("http.request.uri == \"https://www.example.com/malicious\"")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustGatewayPolicy:ZeroTrustGatewayPolicy example <account_id>/<teams_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_gateway_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayPolicyArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the teams rule.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The wirefilter expression to be used for device_posture check matching.
        #[builder(into, default)]
        pub device_posture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicator of rule enablement.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The protocol or layer to evaluate the traffic and identity expressions.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The wirefilter expression to be used for identity matching.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the teams rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The evaluation precedence of the teams rule.
        #[builder(into)]
        pub precedence: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Additional rule settings.
        #[builder(into, default)]
        pub rule_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustGatewayPolicyRuleSettings>,
        >,
        /// The wirefilter expression to be used for traffic matching.
        #[builder(into, default)]
        pub traffic: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayPolicyResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The description of the teams rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The wirefilter expression to be used for device_posture check matching.
        pub device_posture: pulumi_gestalt_rust::Output<String>,
        /// Indicator of rule enablement.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The protocol or layer to evaluate the traffic and identity expressions.
        pub filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The wirefilter expression to be used for identity matching.
        pub identity: pulumi_gestalt_rust::Output<String>,
        /// The name of the teams rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The evaluation precedence of the teams rule.
        pub precedence: pulumi_gestalt_rust::Output<i32>,
        /// Additional rule settings.
        pub rule_settings: pulumi_gestalt_rust::Output<
            super::types::ZeroTrustGatewayPolicyRuleSettings,
        >,
        /// The wirefilter expression to be used for traffic matching.
        pub traffic: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustGatewayPolicyArgs,
    ) -> ZeroTrustGatewayPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let action_binding = args.action.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let device_posture_binding = args.device_posture.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let precedence_binding = args.precedence.get_output(context).get_inner();
        let rule_settings_binding = args.rule_settings.get_output(context).get_inner();
        let traffic_binding = args.traffic.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayPolicy:ZeroTrustGatewayPolicy"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "devicePosture".into(),
                    value: &device_posture_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "precedence".into(),
                    value: &precedence_binding,
                },
                register_interface::ObjectField {
                    name: "ruleSettings".into(),
                    value: &rule_settings_binding,
                },
                register_interface::ObjectField {
                    name: "traffic".into(),
                    value: &traffic_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustGatewayPolicyResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            device_posture: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("devicePosture"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            precedence: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("precedence"),
            ),
            rule_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleSettings"),
            ),
            traffic: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("traffic"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
