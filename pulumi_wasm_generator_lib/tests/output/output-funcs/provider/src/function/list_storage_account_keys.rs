use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::list_storage_account_keys;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl list_storage_account_keys::Guest for Component {
    fn invoke(
        args: list_storage_account_keys::Args
    ) -> list_storage_account_keys::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::listStorageAccountKeys".into(),
            object: vec![
                ObjectField { name: "accountName".into(), value: args.account_name },
                ObjectField { name: "expand".into(), value: args.expand },
                ObjectField { name: "resourceGroupName".into(), value: args.resource_group_name },
            ],
            results: vec![
                ResultField { name: "keys".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        list_storage_account_keys::Res {
            keys: hashmap.remove("keys").unwrap(),
        }
    }
}
