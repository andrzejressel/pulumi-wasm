use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::total_tls;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl total_tls::Guest for Component {
    fn invoke(name: String, args: total_tls::Args) -> total_tls::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/totalTls:TotalTls".into(),
            name,
            object: vec![
                ObjectField { name: "certificateAuthority".into(), value: args.certificate_authority },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "certificateAuthority".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        total_tls::Res {
            certificate_authority: hashmap.remove("certificateAuthority").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
