use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::waiting_room;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl waiting_room::Guest for Component {
    fn invoke(name: String, args: waiting_room::Args) -> waiting_room::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoom:WaitingRoom".into(),
            name,
            object: vec![
                ObjectField { name: "additionalRoutes".into(), value: args.additional_routes },
                ObjectField { name: "cookieSuffix".into(), value: args.cookie_suffix },
                ObjectField { name: "customPageHtml".into(), value: args.custom_page_html },
                ObjectField { name: "defaultTemplateLanguage".into(), value: args.default_template_language },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "disableSessionRenewal".into(), value: args.disable_session_renewal },
                ObjectField { name: "host".into(), value: args.host },
                ObjectField { name: "jsonResponseEnabled".into(), value: args.json_response_enabled },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "newUsersPerMinute".into(), value: args.new_users_per_minute },
                ObjectField { name: "path".into(), value: args.path },
                ObjectField { name: "queueAll".into(), value: args.queue_all },
                ObjectField { name: "queueingMethod".into(), value: args.queueing_method },
                ObjectField { name: "queueingStatusCode".into(), value: args.queueing_status_code },
                ObjectField { name: "sessionDuration".into(), value: args.session_duration },
                ObjectField { name: "suspended".into(), value: args.suspended },
                ObjectField { name: "totalActiveUsers".into(), value: args.total_active_users },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "additionalRoutes".into() },
                ResultField { name: "cookieSuffix".into() },
                ResultField { name: "customPageHtml".into() },
                ResultField { name: "defaultTemplateLanguage".into() },
                ResultField { name: "description".into() },
                ResultField { name: "disableSessionRenewal".into() },
                ResultField { name: "host".into() },
                ResultField { name: "jsonResponseEnabled".into() },
                ResultField { name: "name".into() },
                ResultField { name: "newUsersPerMinute".into() },
                ResultField { name: "path".into() },
                ResultField { name: "queueAll".into() },
                ResultField { name: "queueingMethod".into() },
                ResultField { name: "queueingStatusCode".into() },
                ResultField { name: "sessionDuration".into() },
                ResultField { name: "suspended".into() },
                ResultField { name: "totalActiveUsers".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room::Res {
            additional_routes: hashmap.remove("additionalRoutes").unwrap(),
            cookie_suffix: hashmap.remove("cookieSuffix").unwrap(),
            custom_page_html: hashmap.remove("customPageHtml").unwrap(),
            default_template_language: hashmap.remove("defaultTemplateLanguage").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_session_renewal: hashmap.remove("disableSessionRenewal").unwrap(),
            host: hashmap.remove("host").unwrap(),
            json_response_enabled: hashmap.remove("jsonResponseEnabled").unwrap(),
            name: hashmap.remove("name").unwrap(),
            new_users_per_minute: hashmap.remove("newUsersPerMinute").unwrap(),
            path: hashmap.remove("path").unwrap(),
            queue_all: hashmap.remove("queueAll").unwrap(),
            queueing_method: hashmap.remove("queueingMethod").unwrap(),
            queueing_status_code: hashmap.remove("queueingStatusCode").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            total_active_users: hashmap.remove("totalActiveUsers").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
