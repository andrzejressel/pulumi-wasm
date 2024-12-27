/// Provides a resource to configure Bot Management.
///
/// Specifically, this resource can be used to manage:
///
/// - **Bot Fight Mode**
/// - **Super Bot Fight Mode**
/// - **Bot Management for Enterprise**
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bot_management::create(
///         "example",
///         BotManagementArgs::builder()
///             .enable_js(true)
///             .optimize_wordpress(true)
///             .sbfm_definitely_automated("block")
///             .sbfm_likely_automated("managed_challenge")
///             .sbfm_static_resource_protection(false)
///             .sbfm_verified_bots("allow")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/botManagement:BotManagement example <zone_id>
/// ```
///
pub mod bot_management {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotManagementArgs {
        /// Enable rule to block AI Scrapers and Crawlers.
        #[builder(into, default)]
        pub ai_bots_protection: pulumi_wasm_rust::Output<Option<String>>,
        /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
        #[builder(into, default)]
        pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
        /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
        #[builder(into, default)]
        pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable Bot Fight Mode.
        #[builder(into, default)]
        pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
        #[builder(into, default)]
        pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
        #[builder(into, default)]
        pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
        #[builder(into, default)]
        pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
        #[builder(into, default)]
        pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
        #[builder(into, default)]
        pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
        #[builder(into, default)]
        pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BotManagementResult {
        /// Enable rule to block AI Scrapers and Crawlers.
        pub ai_bots_protection: pulumi_wasm_rust::Output<String>,
        /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
        pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
        /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
        pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable Bot Fight Mode.
        pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
        pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
        pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
        pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
        pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
        pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
        pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
        /// A read-only field that indicates whether the zone currently is running the latest ML model.
        pub using_latest_model: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BotManagementArgs) -> BotManagementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ai_bots_protection_binding = args.ai_bots_protection.get_inner();
        let auto_update_model_binding = args.auto_update_model.get_inner();
        let enable_js_binding = args.enable_js.get_inner();
        let fight_mode_binding = args.fight_mode.get_inner();
        let optimize_wordpress_binding = args.optimize_wordpress.get_inner();
        let sbfm_definitely_automated_binding = args
            .sbfm_definitely_automated
            .get_inner();
        let sbfm_likely_automated_binding = args.sbfm_likely_automated.get_inner();
        let sbfm_static_resource_protection_binding = args
            .sbfm_static_resource_protection
            .get_inner();
        let sbfm_verified_bots_binding = args.sbfm_verified_bots.get_inner();
        let suppress_session_score_binding = args.suppress_session_score.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/botManagement:BotManagement".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aiBotsProtection".into(),
                    value: &ai_bots_protection_binding,
                },
                register_interface::ObjectField {
                    name: "autoUpdateModel".into(),
                    value: &auto_update_model_binding,
                },
                register_interface::ObjectField {
                    name: "enableJs".into(),
                    value: &enable_js_binding,
                },
                register_interface::ObjectField {
                    name: "fightMode".into(),
                    value: &fight_mode_binding,
                },
                register_interface::ObjectField {
                    name: "optimizeWordpress".into(),
                    value: &optimize_wordpress_binding,
                },
                register_interface::ObjectField {
                    name: "sbfmDefinitelyAutomated".into(),
                    value: &sbfm_definitely_automated_binding,
                },
                register_interface::ObjectField {
                    name: "sbfmLikelyAutomated".into(),
                    value: &sbfm_likely_automated_binding,
                },
                register_interface::ObjectField {
                    name: "sbfmStaticResourceProtection".into(),
                    value: &sbfm_static_resource_protection_binding,
                },
                register_interface::ObjectField {
                    name: "sbfmVerifiedBots".into(),
                    value: &sbfm_verified_bots_binding,
                },
                register_interface::ObjectField {
                    name: "suppressSessionScore".into(),
                    value: &suppress_session_score_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aiBotsProtection".into(),
                },
                register_interface::ResultField {
                    name: "autoUpdateModel".into(),
                },
                register_interface::ResultField {
                    name: "enableJs".into(),
                },
                register_interface::ResultField {
                    name: "fightMode".into(),
                },
                register_interface::ResultField {
                    name: "optimizeWordpress".into(),
                },
                register_interface::ResultField {
                    name: "sbfmDefinitelyAutomated".into(),
                },
                register_interface::ResultField {
                    name: "sbfmLikelyAutomated".into(),
                },
                register_interface::ResultField {
                    name: "sbfmStaticResourceProtection".into(),
                },
                register_interface::ResultField {
                    name: "sbfmVerifiedBots".into(),
                },
                register_interface::ResultField {
                    name: "suppressSessionScore".into(),
                },
                register_interface::ResultField {
                    name: "usingLatestModel".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BotManagementResult {
            ai_bots_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aiBotsProtection").unwrap(),
            ),
            auto_update_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoUpdateModel").unwrap(),
            ),
            enable_js: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableJs").unwrap(),
            ),
            fight_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fightMode").unwrap(),
            ),
            optimize_wordpress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optimizeWordpress").unwrap(),
            ),
            sbfm_definitely_automated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sbfmDefinitelyAutomated").unwrap(),
            ),
            sbfm_likely_automated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sbfmLikelyAutomated").unwrap(),
            ),
            sbfm_static_resource_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sbfmStaticResourceProtection").unwrap(),
            ),
            sbfm_verified_bots: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sbfmVerifiedBots").unwrap(),
            ),
            suppress_session_score: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressSessionScore").unwrap(),
            ),
            using_latest_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usingLatestModel").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
