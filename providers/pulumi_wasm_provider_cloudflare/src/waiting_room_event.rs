use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::waiting_room_event;
use crate::Component;
use std::collections::HashMap;

impl waiting_room_event::Guest for Component {
    fn invoke(name: String, args: waiting_room_event::Args) -> waiting_room_event::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomEvent:WaitingRoomEvent".into(),
            name,
            object: vec![
                ObjectField {
                    name: "customPageHtml".into(),
                    value: args.custom_page_html,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disableSessionRenewal".into(),
                    value: args.disable_session_renewal,
                },
                ObjectField {
                    name: "eventEndTime".into(),
                    value: args.event_end_time,
                },
                ObjectField {
                    name: "eventStartTime".into(),
                    value: args.event_start_time,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "newUsersPerMinute".into(),
                    value: args.new_users_per_minute,
                },
                ObjectField {
                    name: "prequeueStartTime".into(),
                    value: args.prequeue_start_time,
                },
                ObjectField {
                    name: "queueingMethod".into(),
                    value: args.queueing_method,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "shuffleAtEventStart".into(),
                    value: args.shuffle_at_event_start,
                },
                ObjectField {
                    name: "suspended".into(),
                    value: args.suspended,
                },
                ObjectField {
                    name: "totalActiveUsers".into(),
                    value: args.total_active_users,
                },
                ObjectField {
                    name: "waitingRoomId".into(),
                    value: args.waiting_room_id,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "customPageHtml".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disableSessionRenewal".into(),
                },
                ResultField {
                    name: "eventEndTime".into(),
                },
                ResultField {
                    name: "eventStartTime".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "newUsersPerMinute".into(),
                },
                ResultField {
                    name: "prequeueStartTime".into(),
                },
                ResultField {
                    name: "queueingMethod".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "shuffleAtEventStart".into(),
                },
                ResultField {
                    name: "suspended".into(),
                },
                ResultField {
                    name: "totalActiveUsers".into(),
                },
                ResultField {
                    name: "waitingRoomId".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_event::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            custom_page_html: hashmap.remove("customPageHtml").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_session_renewal: hashmap.remove("disableSessionRenewal").unwrap(),
            event_end_time: hashmap.remove("eventEndTime").unwrap(),
            event_start_time: hashmap.remove("eventStartTime").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            new_users_per_minute: hashmap.remove("newUsersPerMinute").unwrap(),
            prequeue_start_time: hashmap.remove("prequeueStartTime").unwrap(),
            queueing_method: hashmap.remove("queueingMethod").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            shuffle_at_event_start: hashmap.remove("shuffleAtEventStart").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            total_active_users: hashmap.remove("totalActiveUsers").unwrap(),
            waiting_room_id: hashmap.remove("waitingRoomId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
