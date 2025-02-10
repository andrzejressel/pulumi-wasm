/// Provides a Cloudflare Waiting Room resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = waiting_room::create(
///         "example",
///         WaitingRoomArgs::builder()
///             .additional_routes(
///                 vec![
///                     WaitingRoomAdditionalRoute::builder().host("shop1.example.com")
///                     .path("/example-path").build_struct(),
///                     WaitingRoomAdditionalRoute::builder().host("shop2.example.com")
///                     .build_struct(),
///                 ],
///             )
///             .cookie_suffix("queue1")
///             .enabled_origin_commands(vec!["revoke",])
///             .host("foo.example.com")
///             .name("foo")
///             .new_users_per_minute(200)
///             .path("/")
///             .queueing_status_code(200)
///             .total_active_users(200)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Use the Zone ID and Waiting Room ID to import.
///
/// ```sh
/// $ pulumi import cloudflare:index/waitingRoom:WaitingRoom default <zone_id>/<waiting_room_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod waiting_room {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomArgs {
        /// A list of additional hostname and paths combination to be applied on the waiting room.
        #[builder(into, default)]
        pub additional_routes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::WaitingRoomAdditionalRoute>>,
        >,
        /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
        #[builder(into, default)]
        pub cookie_suffix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This is a templated html file that will be rendered at the edge.
        #[builder(into, default)]
        pub custom_page_html: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`, `bg-BG`, `hr-HR`, `cs-CZ`, `da-DK`, `fi-FI`, `lt-LT`, `ms-MY`, `nb-NO`, `ro-RO`, `el-GR`, `he-IL`, `hi-IN`, `hu-HU`, `sr-BA`, `sk-SK`, `sl-SI`, `sv-SE`, `tl-PH`, `th-TH`, `uk-UA`, `vi-VN`. Defaults to `en-US`.
        #[builder(into, default)]
        pub default_template_language: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A description to add more details about the waiting room.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Disables automatic renewal of session cookies.
        #[builder(into, default)]
        pub disable_session_renewal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The list of enabled origin commands for the waiting room. Available values: `revoke`.
        #[builder(into, default)]
        pub enabled_origin_commands: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Host name for which the waiting room will be applied (no wildcards).
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
        #[builder(into, default)]
        pub json_response_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of new users that will be let into the route every minute.
        #[builder(into)]
        pub new_users_per_minute: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The path within the host to enable the waiting room on. Defaults to `/`.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If queue_all is true, then all traffic will be sent to the waiting room.
        #[builder(into, default)]
        pub queue_all: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
        #[builder(into, default)]
        pub queueing_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP status code returned to a user while in the queue. Defaults to `200`.
        #[builder(into, default)]
        pub queueing_status_code: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Suspends the waiting room.
        #[builder(into, default)]
        pub suspended: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        #[builder(into)]
        pub total_active_users: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomResult {
        /// A list of additional hostname and paths combination to be applied on the waiting room.
        pub additional_routes: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::WaitingRoomAdditionalRoute>>,
        >,
        /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
        pub cookie_suffix: pulumi_gestalt_rust::Output<Option<String>>,
        /// This is a templated html file that will be rendered at the edge.
        pub custom_page_html: pulumi_gestalt_rust::Output<Option<String>>,
        /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`, `bg-BG`, `hr-HR`, `cs-CZ`, `da-DK`, `fi-FI`, `lt-LT`, `ms-MY`, `nb-NO`, `ro-RO`, `el-GR`, `he-IL`, `hi-IN`, `hu-HU`, `sr-BA`, `sk-SK`, `sl-SI`, `sv-SE`, `tl-PH`, `th-TH`, `uk-UA`, `vi-VN`. Defaults to `en-US`.
        pub default_template_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description to add more details about the waiting room.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Disables automatic renewal of session cookies.
        pub disable_session_renewal: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The list of enabled origin commands for the waiting room. Available values: `revoke`.
        pub enabled_origin_commands: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Host name for which the waiting room will be applied (no wildcards).
        pub host: pulumi_gestalt_rust::Output<String>,
        /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
        pub json_response_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of new users that will be let into the route every minute.
        pub new_users_per_minute: pulumi_gestalt_rust::Output<i32>,
        /// The path within the host to enable the waiting room on. Defaults to `/`.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// If queue_all is true, then all traffic will be sent to the waiting room.
        pub queue_all: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
        pub queueing_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// HTTP status code returned to a user while in the queue. Defaults to `200`.
        pub queueing_status_code: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
        pub session_duration: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Suspends the waiting room.
        pub suspended: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        pub total_active_users: pulumi_gestalt_rust::Output<i32>,
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
        args: WaitingRoomArgs,
    ) -> WaitingRoomResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_routes_binding = args.additional_routes.get_output(context);
        let cookie_suffix_binding = args.cookie_suffix.get_output(context);
        let custom_page_html_binding = args.custom_page_html.get_output(context);
        let default_template_language_binding = args
            .default_template_language
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_session_renewal_binding = args
            .disable_session_renewal
            .get_output(context);
        let enabled_origin_commands_binding = args
            .enabled_origin_commands
            .get_output(context);
        let host_binding = args.host.get_output(context);
        let json_response_enabled_binding = args
            .json_response_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let new_users_per_minute_binding = args.new_users_per_minute.get_output(context);
        let path_binding = args.path.get_output(context);
        let queue_all_binding = args.queue_all.get_output(context);
        let queueing_method_binding = args.queueing_method.get_output(context);
        let queueing_status_code_binding = args.queueing_status_code.get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let suspended_binding = args.suspended.get_output(context);
        let total_active_users_binding = args.total_active_users.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoom:WaitingRoom".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalRoutes".into(),
                    value: additional_routes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cookieSuffix".into(),
                    value: cookie_suffix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customPageHtml".into(),
                    value: custom_page_html_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultTemplateLanguage".into(),
                    value: default_template_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableSessionRenewal".into(),
                    value: disable_session_renewal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledOriginCommands".into(),
                    value: enabled_origin_commands_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: host_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jsonResponseEnabled".into(),
                    value: json_response_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "newUsersPerMinute".into(),
                    value: new_users_per_minute_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueAll".into(),
                    value: queue_all_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueingMethod".into(),
                    value: queueing_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueingStatusCode".into(),
                    value: queueing_status_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: session_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspended".into(),
                    value: suspended_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "totalActiveUsers".into(),
                    value: total_active_users_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WaitingRoomResult {
            additional_routes: o.get_field("additionalRoutes"),
            cookie_suffix: o.get_field("cookieSuffix"),
            custom_page_html: o.get_field("customPageHtml"),
            default_template_language: o.get_field("defaultTemplateLanguage"),
            description: o.get_field("description"),
            disable_session_renewal: o.get_field("disableSessionRenewal"),
            enabled_origin_commands: o.get_field("enabledOriginCommands"),
            host: o.get_field("host"),
            json_response_enabled: o.get_field("jsonResponseEnabled"),
            name: o.get_field("name"),
            new_users_per_minute: o.get_field("newUsersPerMinute"),
            path: o.get_field("path"),
            queue_all: o.get_field("queueAll"),
            queueing_method: o.get_field("queueingMethod"),
            queueing_status_code: o.get_field("queueingStatusCode"),
            session_duration: o.get_field("sessionDuration"),
            suspended: o.get_field("suspended"),
            total_active_users: o.get_field("totalActiveUsers"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
