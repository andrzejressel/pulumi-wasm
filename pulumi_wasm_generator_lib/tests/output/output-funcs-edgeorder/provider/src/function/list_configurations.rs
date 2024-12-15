use std::collections::HashMap;
use crate::bindings::exports::pulumi::myedgeorder::list_configurations;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl list_configurations::Guest for Component {
    fn invoke(
        args: list_configurations::Args
    ) -> list_configurations::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "myedgeorder::listConfigurations".into(),
            object: vec![
                ObjectField { name: "configurationFilters".into(), value: args.configuration_filters },
                ObjectField { name: "customerSubscriptionDetails".into(), value: args.customer_subscription_details },
                ObjectField { name: "skipToken".into(), value: args.skip_token },
            ],
            results: vec![
                ResultField { name: "nextLink".into() },
                ResultField { name: "value".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list_configurations::Res {
            next_link: hashmap.remove("nextLink").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }

    }
}
