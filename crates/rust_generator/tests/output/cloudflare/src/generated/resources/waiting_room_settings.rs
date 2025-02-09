/// Configure zone-wide settings for Cloudflare waiting rooms.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = waiting_room_settings::create(
///         "example",
///         WaitingRoomSettingsArgs::builder()
///             .search_engine_crawler_bypass(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/waitingRoomSettings:WaitingRoomSettings example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod waiting_room_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomSettingsArgs {
        /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
        #[builder(into, default)]
        pub search_engine_crawler_bypass: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomSettingsResult {
        /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
        pub search_engine_crawler_bypass: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WaitingRoomSettingsArgs,
    ) -> WaitingRoomSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let search_engine_crawler_bypass_binding_1 = args
            .search_engine_crawler_bypass
            .get_output(context);
        let search_engine_crawler_bypass_binding = search_engine_crawler_bypass_binding_1
            .get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomSettings:WaitingRoomSettings".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "searchEngineCrawlerBypass".into(),
                    value: &search_engine_crawler_bypass_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WaitingRoomSettingsResult {
            search_engine_crawler_bypass: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("searchEngineCrawlerBypass"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
