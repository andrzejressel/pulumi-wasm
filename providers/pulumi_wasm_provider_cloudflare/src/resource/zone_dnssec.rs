use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zone_dnssec;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zone_dnssec::Guest for Component {
    fn invoke(
        name: String,
        args: zone_dnssec::Args
    ) -> zone_dnssec::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneDnssec:ZoneDnssec".into(),
            name,
            object: vec![
                ObjectField { name: "modifiedOn".into(), value: args.modified_on },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "algorithm".into() },
                ResultField { name: "digest".into() },
                ResultField { name: "digestAlgorithm".into() },
                ResultField { name: "digestType".into() },
                ResultField { name: "ds".into() },
                ResultField { name: "flags".into() },
                ResultField { name: "keyTag".into() },
                ResultField { name: "keyType".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "publicKey".into() },
                ResultField { name: "status".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zone_dnssec::Res {
            algorithm: hashmap.remove("algorithm").unwrap(),
            digest: hashmap.remove("digest").unwrap(),
            digest_algorithm: hashmap.remove("digestAlgorithm").unwrap(),
            digest_type: hashmap.remove("digestType").unwrap(),
            ds: hashmap.remove("ds").unwrap(),
            flags: hashmap.remove("flags").unwrap(),
            key_tag: hashmap.remove("keyTag").unwrap(),
            key_type: hashmap.remove("keyType").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            public_key: hashmap.remove("publicKey").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
