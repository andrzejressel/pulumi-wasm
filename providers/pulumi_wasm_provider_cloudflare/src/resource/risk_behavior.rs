use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::risk_behavior;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl risk_behavior::Guest for Component {
    fn invoke(
        name: String,
        args: risk_behavior::Args
    ) -> risk_behavior::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/riskBehavior:RiskBehavior".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "behaviors".into(), value: args.behaviors },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "behaviors".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        risk_behavior::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            behaviors: hashmap.remove("behaviors").unwrap(),
        }

    }
}
