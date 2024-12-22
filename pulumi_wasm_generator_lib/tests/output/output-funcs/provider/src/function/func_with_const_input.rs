use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::func_with_const_input;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl func_with_const_input::Guest for Component {
    fn invoke(
        args: func_with_const_input::Args
    ) {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::funcWithConstInput".into(),
            object: vec![
                ObjectField { name: "plainInput".into(), value: args.plain_input },
            ],
            results: vec![
            ],
        };

        invoke(&request);
    }
}
