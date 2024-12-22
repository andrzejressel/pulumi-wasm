use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::keyless_certificate;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl keyless_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: keyless_certificate::Args
    ) -> keyless_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/keylessCertificate:KeylessCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "bundleMethod".into(), value: args.bundle_method },
                ObjectField { name: "certificate".into(), value: args.certificate },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "host".into(), value: args.host },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "port".into(), value: args.port },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "bundleMethod".into() },
                ResultField { name: "certificate".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "host".into() },
                ResultField { name: "name".into() },
                ResultField { name: "port".into() },
                ResultField { name: "status".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        keyless_certificate::Res {
            bundle_method: hashmap.remove("bundleMethod").unwrap(),
            certificate: hashmap.remove("certificate").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            host: hashmap.remove("host").unwrap(),
            name: hashmap.remove("name").unwrap(),
            port: hashmap.remove("port").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
