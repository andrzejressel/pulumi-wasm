use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_devices;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_devices::Guest for Component {
    fn invoke(
        name: String,
        args: get_devices::Args
    ) -> get_devices::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getDevices:getDevices".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "devices".into() },
                ResultField { name: "id".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_devices::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            devices: hashmap.remove("devices").unwrap(),
            id: hashmap.remove("id").unwrap(),
        }

    }
}
