use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::func_with_secrets;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl func_with_secrets::Guest for Component {
    fn invoke(
        name: String,
        args: func_with_secrets::Args
    ) -> func_with_secrets::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::funcWithSecrets".into(),
            object: vec![
                ObjectField { name: "cryptoKey".into(), value: args.crypto_key },
                ObjectField { name: "plaintext".into(), value: args.plaintext },
            ],
            results: vec![
                ResultField { name: "ciphertext".into() },
                ResultField { name: "cryptoKey".into() },
                ResultField { name: "id".into() },
                ResultField { name: "plaintext".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        func_with_secrets::Res {
            ciphertext: hashmap.remove("ciphertext").unwrap(),
            crypto_key: hashmap.remove("cryptoKey").unwrap(),
            id: hashmap.remove("id").unwrap(),
            plaintext: hashmap.remove("plaintext").unwrap(),
        }

    }
}
