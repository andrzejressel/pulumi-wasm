/// Provides a Cloudflare Web Analytics Site resource.
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
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/webAnalyticsSite:WebAnalyticsSite example <account_id>/<site_tag>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_analytics_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAnalyticsSiteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub auto_install: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub host: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebAnalyticsSiteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
        pub auto_install: pulumi_gestalt_rust::Output<bool>,
        /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
        pub host: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID for the ruleset associated to this Web Analytics Site.
        pub ruleset_id: pulumi_gestalt_rust::Output<String>,
        /// The Web Analytics site tag.
        pub site_tag: pulumi_gestalt_rust::Output<String>,
        /// The token for the Web Analytics site.
        pub site_token: pulumi_gestalt_rust::Output<String>,
        /// The encoded JS snippet to add to your site's HTML page if auto_install is false.
        pub snippet: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
        pub zone_tag: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAnalyticsSiteArgs,
    ) -> WebAnalyticsSiteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let auto_install_binding = args.auto_install.get_output(context);
        let host_binding = args.host.get_output(context);
        let zone_tag_binding = args.zone_tag.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsSite:WebAnalyticsSite".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoInstall".into(),
                    value: auto_install_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: host_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneTag".into(),
                    value: zone_tag_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAnalyticsSiteResult {
            account_id: o.get_field("accountId"),
            auto_install: o.get_field("autoInstall"),
            host: o.get_field("host"),
            ruleset_id: o.get_field("rulesetId"),
            site_tag: o.get_field("siteTag"),
            site_token: o.get_field("siteToken"),
            snippet: o.get_field("snippet"),
            zone_tag: o.get_field("zoneTag"),
        }
    }
}
