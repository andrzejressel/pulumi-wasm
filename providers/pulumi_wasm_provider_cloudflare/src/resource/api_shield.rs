use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_shield;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_shield::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield::Args
    ) -> api_shield::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShield:ApiShield".into(),
            name,
            object: vec![
                ObjectField { name: "authIdCharacteristics".into(), value: args.auth_id_characteristics },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "authIdCharacteristics".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield::Res {
            auth_id_characteristics: hashmap.remove("authIdCharacteristics").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
