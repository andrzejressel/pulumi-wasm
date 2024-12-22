use std::collections::HashMap;
use crate::bindings::exports::pulumi::example::func_with_all_optional_inputs;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl func_with_all_optional_inputs::Guest for Component {
    fn invoke(
        args: func_with_all_optional_inputs::Args
    ) -> func_with_all_optional_inputs::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::funcWithAllOptionalInputs".into(),
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
        func_with_all_optional_inputs::Res {
            r: hashmap.remove("r").unwrap(),
        }
    }
}
