/// Provides a Cloudflare Web Analytics Site resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod web_analytics_site {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAnalyticsSiteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub auto_install: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub host: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_tag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebAnalyticsSiteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
        pub auto_install: pulumi_wasm_rust::Output<bool>,
        /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
        pub host: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID for the ruleset associated to this Web Analytics Site.
        pub ruleset_id: pulumi_wasm_rust::Output<String>,
        /// The Web Analytics site tag.
        pub site_tag: pulumi_wasm_rust::Output<String>,
        /// The token for the Web Analytics site.
        pub site_token: pulumi_wasm_rust::Output<String>,
        /// The encoded JS snippet to add to your site's HTML page if auto_install is false.
        pub snippet: pulumi_wasm_rust::Output<String>,
        /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
        pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebAnalyticsSiteArgs,
    ) -> WebAnalyticsSiteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let auto_install_binding = args.auto_install.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let zone_tag_binding = args.zone_tag.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsSite:WebAnalyticsSite".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "autoInstall".into(),
                    value: &auto_install_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "zoneTag".into(),
                    value: &zone_tag_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "autoInstall".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "rulesetId".into(),
                },
                register_interface::ResultField {
                    name: "siteTag".into(),
                },
                register_interface::ResultField {
                    name: "siteToken".into(),
                },
                register_interface::ResultField {
                    name: "snippet".into(),
                },
                register_interface::ResultField {
                    name: "zoneTag".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAnalyticsSiteResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            auto_install: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoInstall").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            ruleset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rulesetId").unwrap(),
            ),
            site_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteTag").unwrap(),
            ),
            site_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteToken").unwrap(),
            ),
            snippet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snippet").unwrap(),
            ),
            zone_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneTag").unwrap(),
            ),
        }
    }
}
