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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bot_management {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotManagementArgs {
        /// Enable rule to block AI Scrapers and Crawlers.
        #[builder(into, default)]
        pub ai_bots_protection: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
        #[builder(into, default)]
        pub auto_update_model: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
        #[builder(into, default)]
        pub enable_js: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable Bot Fight Mode.
        #[builder(into, default)]
        pub fight_mode: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
        #[builder(into, default)]
        pub optimize_wordpress: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
        #[builder(into, default)]
        pub sbfm_definitely_automated: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
        #[builder(into, default)]
        pub sbfm_likely_automated: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
        #[builder(into, default)]
        pub sbfm_static_resource_protection: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
        #[builder(into, default)]
        pub sbfm_verified_bots: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
        #[builder(into, default)]
        pub suppress_session_score: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BotManagementResult {
        /// Enable rule to block AI Scrapers and Crawlers.
        pub ai_bots_protection: pulumi_gestalt_rust::Output<String>,
        /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
        pub auto_update_model: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
        pub enable_js: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable Bot Fight Mode.
        pub fight_mode: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
        pub optimize_wordpress: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
        pub sbfm_definitely_automated: pulumi_gestalt_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
        pub sbfm_likely_automated: pulumi_gestalt_rust::Output<Option<String>>,
        /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
        pub sbfm_static_resource_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
        pub sbfm_verified_bots: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
        pub suppress_session_score: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A read-only field that indicates whether the zone currently is running the latest ML model.
        pub using_latest_model: pulumi_gestalt_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BotManagementArgs,
    ) -> BotManagementResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ai_bots_protection_binding = args.ai_bots_protection.get_output(context);
        let auto_update_model_binding = args.auto_update_model.get_output(context);
        let enable_js_binding = args.enable_js.get_output(context);
        let fight_mode_binding = args.fight_mode.get_output(context);
        let optimize_wordpress_binding = args.optimize_wordpress.get_output(context);
        let sbfm_definitely_automated_binding = args
            .sbfm_definitely_automated
            .get_output(context);
        let sbfm_likely_automated_binding = args
            .sbfm_likely_automated
            .get_output(context);
        let sbfm_static_resource_protection_binding = args
            .sbfm_static_resource_protection
            .get_output(context);
        let sbfm_verified_bots_binding = args.sbfm_verified_bots.get_output(context);
        let suppress_session_score_binding = args
            .suppress_session_score
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/botManagement:BotManagement".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aiBotsProtection".into(),
                    value: ai_bots_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpdateModel".into(),
                    value: auto_update_model_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableJs".into(),
                    value: enable_js_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fightMode".into(),
                    value: fight_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optimizeWordpress".into(),
                    value: optimize_wordpress_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sbfmDefinitelyAutomated".into(),
                    value: sbfm_definitely_automated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sbfmLikelyAutomated".into(),
                    value: sbfm_likely_automated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sbfmStaticResourceProtection".into(),
                    value: sbfm_static_resource_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sbfmVerifiedBots".into(),
                    value: sbfm_verified_bots_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suppressSessionScore".into(),
                    value: suppress_session_score_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BotManagementResult {
            ai_bots_protection: o.get_field("aiBotsProtection"),
            auto_update_model: o.get_field("autoUpdateModel"),
            enable_js: o.get_field("enableJs"),
            fight_mode: o.get_field("fightMode"),
            optimize_wordpress: o.get_field("optimizeWordpress"),
            sbfm_definitely_automated: o.get_field("sbfmDefinitelyAutomated"),
            sbfm_likely_automated: o.get_field("sbfmLikelyAutomated"),
            sbfm_static_resource_protection: o.get_field("sbfmStaticResourceProtection"),
            sbfm_verified_bots: o.get_field("sbfmVerifiedBots"),
            suppress_session_score: o.get_field("suppressSessionScore"),
            using_latest_model: o.get_field("usingLatestModel"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
