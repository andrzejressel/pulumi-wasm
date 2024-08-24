use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::zone_settings_override;
use crate::Component;
use std::collections::HashMap;

impl zone_settings_override::Guest for Component {
    fn invoke(name: String, args: zone_settings_override::Args) -> zone_settings_override::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneSettingsOverride:ZoneSettingsOverride".into(),
            name,
            object: vec![
                ObjectField {
                    name: "settings".into(),
                    value: args.settings,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "initialSettings".into(),
                },
                ResultField {
                    name: "initialSettingsReadAt".into(),
                },
                ResultField {
                    name: "readonlySettings".into(),
                },
                ResultField {
                    name: "settings".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
                ResultField {
                    name: "zoneStatus".into(),
                },
                ResultField {
                    name: "zoneType".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_settings_override::Res {
            initial_settings: hashmap.remove("initialSettings").unwrap(),
            initial_settings_read_at: hashmap.remove("initialSettingsReadAt").unwrap(),
            readonly_settings: hashmap.remove("readonlySettings").unwrap(),
            settings: hashmap.remove("settings").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
            zone_status: hashmap.remove("zoneStatus").unwrap(),
            zone_type: hashmap.remove("zoneType").unwrap(),
        }
    }
}
