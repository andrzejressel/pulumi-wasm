use std::collections::HashMap;
use crate::bindings::exports::pulumi::myedgeorder::list_product_families;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl list_product_families::Guest for Component {
    fn invoke(
        args: list_product_families::Args
    ) -> list_product_families::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "myedgeorder::listProductFamilies".into(),
            object: vec![
                ObjectField { name: "customerSubscriptionDetails".into(), value: args.customer_subscription_details },
                ObjectField { name: "expand".into(), value: args.expand },
                ObjectField { name: "filterableProperties".into(), value: args.filterable_properties },
                ObjectField { name: "skipToken".into(), value: args.skip_token },
            ],
            results: vec![
                ResultField { name: "nextLink".into() },
                ResultField { name: "value".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list_product_families::Res {
            next_link: hashmap.remove("nextLink").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }

    }
}
