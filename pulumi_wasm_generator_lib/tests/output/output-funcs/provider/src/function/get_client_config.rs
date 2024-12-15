use std::collections::HashMap;
use crate::bindings::exports::pulumi::mypkg::get_client_config;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_client_config::Guest for Component {
    fn invoke(
    ) -> get_client_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "mypkg::getClientConfig".into(),
            object: vec![
            ],
            results: vec![
                ResultField { name: "clientId".into() },
                ResultField { name: "objectId".into() },
                ResultField { name: "subscriptionId".into() },
                ResultField { name: "tenantId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_client_config::Res {
            client_id: hashmap.remove("clientId").unwrap(),
            object_id: hashmap.remove("objectId").unwrap(),
            subscription_id: hashmap.remove("subscriptionId").unwrap(),
            tenant_id: hashmap.remove("tenantId").unwrap(),
        }
    }
}
