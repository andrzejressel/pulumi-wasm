use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::hostname_tls_setting_ciphers;
use crate::Component;
use std::collections::HashMap;

impl hostname_tls_setting_ciphers::Guest for Component {
    fn invoke(
        name: String,
        args: hostname_tls_setting_ciphers::Args,
    ) -> hostname_tls_setting_ciphers::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "ports".into(),
                    value: args.ports,
                },
                ObjectField {
                    name: "values".into(),
                    value: args.values,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdAt".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "ports".into(),
                },
                ResultField {
                    name: "updatedAt".into(),
                },
                ResultField {
                    name: "values".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hostname_tls_setting_ciphers::Res {
            created_at: hashmap.remove("createdAt").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ports: hashmap.remove("ports").unwrap(),
            updated_at: hashmap.remove("updatedAt").unwrap(),
            values: hashmap.remove("values").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
