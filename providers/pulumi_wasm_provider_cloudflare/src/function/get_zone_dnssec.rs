use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zone_dnssec;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zone_dnssec::Guest for Component {
    fn invoke(
        name: String,
        args: get_zone_dnssec::Args
    ) -> get_zone_dnssec::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZoneDnssec:getZoneDnssec".into(),
            object: vec![
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "algorithm".into() },
                ResultField { name: "digest".into() },
                ResultField { name: "digestAlgorithm".into() },
                ResultField { name: "digestType".into() },
                ResultField { name: "ds".into() },
                ResultField { name: "flags".into() },
                ResultField { name: "id".into() },
                ResultField { name: "keyTag".into() },
                ResultField { name: "keyType".into() },
                ResultField { name: "publicKey".into() },
                ResultField { name: "status".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zone_dnssec::Res {
            algorithm: hashmap.remove("algorithm").unwrap(),
            digest: hashmap.remove("digest").unwrap(),
            digest_algorithm: hashmap.remove("digestAlgorithm").unwrap(),
            digest_type: hashmap.remove("digestType").unwrap(),
            ds: hashmap.remove("ds").unwrap(),
            flags: hashmap.remove("flags").unwrap(),
            id: hashmap.remove("id").unwrap(),
            key_tag: hashmap.remove("keyTag").unwrap(),
            key_type: hashmap.remove("keyType").unwrap(),
            public_key: hashmap.remove("publicKey").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
