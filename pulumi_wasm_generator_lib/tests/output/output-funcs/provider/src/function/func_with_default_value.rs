use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::func_with_default_value;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl func_with_default_value::Guest for Component {
    fn invoke(
        args: func_with_default_value::Args
    ) -> func_with_default_value::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::funcWithDefaultValue".into(),
            object: vec![
                ObjectField { name: "a".into(), value: args.a },
                ObjectField { name: "b".into(), value: args.b },
            ],
            results: vec![
                ResultField { name: "r".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        func_with_default_value::Res {
            r: hashmap.remove("r").unwrap(),
        }
    }
}
