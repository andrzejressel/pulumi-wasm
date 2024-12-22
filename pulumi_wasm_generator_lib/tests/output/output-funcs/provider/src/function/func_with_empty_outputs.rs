use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::func_with_empty_outputs;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl func_with_empty_outputs::Guest for Component {
    fn invoke(
        args: func_with_empty_outputs::Args
    ) {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::funcWithEmptyOutputs".into(),
            object: vec![
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
            ],
        };

        invoke(&request);
    }
}
