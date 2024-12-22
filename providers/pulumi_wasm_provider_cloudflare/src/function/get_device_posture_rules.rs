use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_device_posture_rules;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_device_posture_rules::Guest for Component {
    fn invoke(
        args: get_device_posture_rules::Args
    ) -> get_device_posture_rules::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getDevicePostureRules:getDevicePostureRules".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "rules".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_device_posture_rules::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
