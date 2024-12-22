use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::hostname_tls_setting;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl hostname_tls_setting::Guest for Component {
    fn invoke(
        name: String,
        args: hostname_tls_setting::Args
    ) -> hostname_tls_setting::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSetting:HostnameTlsSetting".into(),
            name,
            object: vec![
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "setting".into(), value: args.setting },
                ObjectField { name: "value".into(), value: args.value },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "createdAt".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "setting".into() },
                ResultField { name: "updatedAt".into() },
                ResultField { name: "value".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hostname_tls_setting::Res {
            created_at: hashmap.remove("createdAt").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            setting: hashmap.remove("setting").unwrap(),
            updated_at: hashmap.remove("updatedAt").unwrap(),
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
