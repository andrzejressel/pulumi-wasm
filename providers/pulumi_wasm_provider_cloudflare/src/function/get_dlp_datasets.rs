use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_dlp_datasets;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_dlp_datasets::Guest for Component {
    fn invoke(
        name: String,
        args: get_dlp_datasets::Args
    ) -> get_dlp_datasets::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getDlpDatasets:getDlpDatasets".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "datasets".into() },
                ResultField { name: "id".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_dlp_datasets::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            datasets: hashmap.remove("datasets").unwrap(),
            id: hashmap.remove("id").unwrap(),
        }

    }
}
