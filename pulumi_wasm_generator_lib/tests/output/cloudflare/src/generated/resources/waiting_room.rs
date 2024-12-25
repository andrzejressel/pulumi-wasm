#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomArgs {
    /// A list of additional hostname and paths combination to be applied on the waiting room.
    #[builder(into, default)]
    pub additional_routes: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WaitingRoomAdditionalRoute>>,
    >,
    /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
    #[builder(into, default)]
    pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
    /// This is a templated html file that will be rendered at the edge.
    #[builder(into, default)]
    pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
    /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`, `bg-BG`, `hr-HR`, `cs-CZ`, `da-DK`, `fi-FI`, `lt-LT`, `ms-MY`, `nb-NO`, `ro-RO`, `el-GR`, `he-IL`, `hi-IN`, `hu-HU`, `sr-BA`, `sk-SK`, `sl-SI`, `sv-SE`, `tl-PH`, `th-TH`, `uk-UA`, `vi-VN`. Defaults to `en-US`.
    #[builder(into, default)]
    pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
    /// A description to add more details about the waiting room.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Disables automatic renewal of session cookies.
    #[builder(into, default)]
    pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
    /// The list of enabled origin commands for the waiting room. Available values: `revoke`.
    #[builder(into, default)]
    pub enabled_origin_commands: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Host name for which the waiting room will be applied (no wildcards).
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
    #[builder(into, default)]
    pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The number of new users that will be let into the route every minute.
    #[builder(into)]
    pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
    /// The path within the host to enable the waiting room on. Defaults to `/`.
    #[builder(into, default)]
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// If queue_all is true, then all traffic will be sent to the waiting room.
    #[builder(into, default)]
    pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
    /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
    #[builder(into, default)]
    pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP status code returned to a user while in the queue. Defaults to `200`.
    #[builder(into, default)]
    pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
    /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
    #[builder(into, default)]
    pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
    /// Suspends the waiting room.
    #[builder(into, default)]
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The total number of active user sessions on the route at a point in time.
    #[builder(into)]
    pub total_active_users: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct WaitingRoomResult {
    /// A list of additional hostname and paths combination to be applied on the waiting room.
    pub additional_routes: pulumi_wasm_rust::Output<
        Option<Vec<super::types::WaitingRoomAdditionalRoute>>,
    >,
    /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
    pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
    /// This is a templated html file that will be rendered at the edge.
    pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
    /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`, `bg-BG`, `hr-HR`, `cs-CZ`, `da-DK`, `fi-FI`, `lt-LT`, `ms-MY`, `nb-NO`, `ro-RO`, `el-GR`, `he-IL`, `hi-IN`, `hu-HU`, `sr-BA`, `sk-SK`, `sl-SI`, `sv-SE`, `tl-PH`, `th-TH`, `uk-UA`, `vi-VN`. Defaults to `en-US`.
    pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
    /// A description to add more details about the waiting room.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Disables automatic renewal of session cookies.
    pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
    /// The list of enabled origin commands for the waiting room. Available values: `revoke`.
    pub enabled_origin_commands: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Host name for which the waiting room will be applied (no wildcards).
    pub host: pulumi_wasm_rust::Output<String>,
    /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
    pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The number of new users that will be let into the route every minute.
    pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
    /// The path within the host to enable the waiting room on. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// If queue_all is true, then all traffic will be sent to the waiting room.
    pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
    /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
    pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP status code returned to a user while in the queue. Defaults to `200`.
    pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
    /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
    pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
    /// Suspends the waiting room.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The total number of active user sessions on the route at a point in time.
    pub total_active_users: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WaitingRoomArgs) -> WaitingRoomResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let additional_routes_binding = args.additional_routes.get_inner();
    let cookie_suffix_binding = args.cookie_suffix.get_inner();
    let custom_page_html_binding = args.custom_page_html.get_inner();
    let default_template_language_binding = args.default_template_language.get_inner();
    let description_binding = args.description.get_inner();
    let disable_session_renewal_binding = args.disable_session_renewal.get_inner();
    let enabled_origin_commands_binding = args.enabled_origin_commands.get_inner();
    let host_binding = args.host.get_inner();
    let json_response_enabled_binding = args.json_response_enabled.get_inner();
    let name_binding = args.name.get_inner();
    let new_users_per_minute_binding = args.new_users_per_minute.get_inner();
    let path_binding = args.path.get_inner();
    let queue_all_binding = args.queue_all.get_inner();
    let queueing_method_binding = args.queueing_method.get_inner();
    let queueing_status_code_binding = args.queueing_status_code.get_inner();
    let session_duration_binding = args.session_duration.get_inner();
    let suspended_binding = args.suspended.get_inner();
    let total_active_users_binding = args.total_active_users.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/waitingRoom:WaitingRoom".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "additionalRoutes".into(),
                value: &additional_routes_binding,
            },
            register_interface::ObjectField {
                name: "cookieSuffix".into(),
                value: &cookie_suffix_binding,
            },
            register_interface::ObjectField {
                name: "customPageHtml".into(),
                value: &custom_page_html_binding,
            },
            register_interface::ObjectField {
                name: "defaultTemplateLanguage".into(),
                value: &default_template_language_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "disableSessionRenewal".into(),
                value: &disable_session_renewal_binding,
            },
            register_interface::ObjectField {
                name: "enabledOriginCommands".into(),
                value: &enabled_origin_commands_binding,
            },
            register_interface::ObjectField {
                name: "host".into(),
                value: &host_binding,
            },
            register_interface::ObjectField {
                name: "jsonResponseEnabled".into(),
                value: &json_response_enabled_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "newUsersPerMinute".into(),
                value: &new_users_per_minute_binding,
            },
            register_interface::ObjectField {
                name: "path".into(),
                value: &path_binding,
            },
            register_interface::ObjectField {
                name: "queueAll".into(),
                value: &queue_all_binding,
            },
            register_interface::ObjectField {
                name: "queueingMethod".into(),
                value: &queueing_method_binding,
            },
            register_interface::ObjectField {
                name: "queueingStatusCode".into(),
                value: &queueing_status_code_binding,
            },
            register_interface::ObjectField {
                name: "sessionDuration".into(),
                value: &session_duration_binding,
            },
            register_interface::ObjectField {
                name: "suspended".into(),
                value: &suspended_binding,
            },
            register_interface::ObjectField {
                name: "totalActiveUsers".into(),
                value: &total_active_users_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "additionalRoutes".into() },
            register_interface::ResultField { name : "cookieSuffix".into() },
            register_interface::ResultField { name : "customPageHtml".into() },
            register_interface::ResultField { name : "defaultTemplateLanguage".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "disableSessionRenewal".into() },
            register_interface::ResultField { name : "enabledOriginCommands".into() },
            register_interface::ResultField { name : "host".into() },
            register_interface::ResultField { name : "jsonResponseEnabled".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "newUsersPerMinute".into() },
            register_interface::ResultField { name : "path".into() },
            register_interface::ResultField { name : "queueAll".into() },
            register_interface::ResultField { name : "queueingMethod".into() },
            register_interface::ResultField { name : "queueingStatusCode".into() },
            register_interface::ResultField { name : "sessionDuration".into() },
            register_interface::ResultField { name : "suspended".into() },
            register_interface::ResultField { name : "totalActiveUsers".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    WaitingRoomResult {
        additional_routes: into_domain(hashmap.remove("additionalRoutes").unwrap()),
        cookie_suffix: into_domain(hashmap.remove("cookieSuffix").unwrap()),
        custom_page_html: into_domain(hashmap.remove("customPageHtml").unwrap()),
        default_template_language: into_domain(
            hashmap.remove("defaultTemplateLanguage").unwrap(),
        ),
        description: into_domain(hashmap.remove("description").unwrap()),
        disable_session_renewal: into_domain(
            hashmap.remove("disableSessionRenewal").unwrap(),
        ),
        enabled_origin_commands: into_domain(
            hashmap.remove("enabledOriginCommands").unwrap(),
        ),
        host: into_domain(hashmap.remove("host").unwrap()),
        json_response_enabled: into_domain(
            hashmap.remove("jsonResponseEnabled").unwrap(),
        ),
        name: into_domain(hashmap.remove("name").unwrap()),
        new_users_per_minute: into_domain(hashmap.remove("newUsersPerMinute").unwrap()),
        path: into_domain(hashmap.remove("path").unwrap()),
        queue_all: into_domain(hashmap.remove("queueAll").unwrap()),
        queueing_method: into_domain(hashmap.remove("queueingMethod").unwrap()),
        queueing_status_code: into_domain(hashmap.remove("queueingStatusCode").unwrap()),
        session_duration: into_domain(hashmap.remove("sessionDuration").unwrap()),
        suspended: into_domain(hashmap.remove("suspended").unwrap()),
        total_active_users: into_domain(hashmap.remove("totalActiveUsers").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
