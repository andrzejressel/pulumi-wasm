/// Provides a Cloudflare Web Analytics Rule resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = web_analytics_site::create(
///         "example",
///         WebAnalyticsSiteArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .auto_install(true)
///             .zone_tag("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let exampleWebAnalyticsRule = web_analytics_rule::create(
///         "exampleWebAnalyticsRule",
///         WebAnalyticsRuleArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .host("*")
///             .inclusive(false)
///             .is_paused(false)
///             .paths(vec!["/excluded",])
///             .ruleset_id("${example.rulesetId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/webAnalyticsRule:WebAnalyticsRule example <account_id>/<ruleset_id>/<rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_analytics_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAnalyticsRuleArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The host to apply the rule to.
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
        #[builder(into)]
        pub inclusive: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Whether the rule is paused or not.
        #[builder(into)]
        pub is_paused: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// A list of paths to apply the rule to.
        #[builder(into)]
        pub paths: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub ruleset_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAnalyticsRuleResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The host to apply the rule to.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
        pub inclusive: pulumi_gestalt_rust::Output<bool>,
        /// Whether the rule is paused or not.
        pub is_paused: pulumi_gestalt_rust::Output<bool>,
        /// A list of paths to apply the rule to.
        pub paths: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
        pub ruleset_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WebAnalyticsRuleArgs,
    ) -> WebAnalyticsRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let host_binding_1 = args.host.get_output(context);
        let host_binding = host_binding_1.get_inner();
        let inclusive_binding_1 = args.inclusive.get_output(context);
        let inclusive_binding = inclusive_binding_1.get_inner();
        let is_paused_binding_1 = args.is_paused.get_output(context);
        let is_paused_binding = is_paused_binding_1.get_inner();
        let paths_binding_1 = args.paths.get_output(context);
        let paths_binding = paths_binding_1.get_inner();
        let ruleset_id_binding_1 = args.ruleset_id.get_output(context);
        let ruleset_id_binding = ruleset_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsRule:WebAnalyticsRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "inclusive".into(),
                    value: &inclusive_binding,
                },
                register_interface::ObjectField {
                    name: "isPaused".into(),
                    value: &is_paused_binding,
                },
                register_interface::ObjectField {
                    name: "paths".into(),
                    value: &paths_binding,
                },
                register_interface::ObjectField {
                    name: "rulesetId".into(),
                    value: &ruleset_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebAnalyticsRuleResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            inclusive: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inclusive"),
            ),
            is_paused: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isPaused"),
            ),
            paths: pulumi_gestalt_rust::__private::into_domain(o.extract_field("paths")),
            ruleset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rulesetId"),
            ),
        }
    }
}
