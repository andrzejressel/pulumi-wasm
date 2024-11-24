use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::dlp_profile;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl dlp_profile::Guest for Component {
    fn invoke(name: String, args: dlp_profile::Args) -> dlp_profile::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/dlpProfile:DlpProfile".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "allowedMatchCount".into(), value: args.allowed_match_count },
                ObjectField { name: "contextAwareness".into(), value: args.context_awareness },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "entries".into(), value: args.entries },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "ocrEnabled".into(), value: args.ocr_enabled },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "allowedMatchCount".into() },
                ResultField { name: "contextAwareness".into() },
                ResultField { name: "description".into() },
                ResultField { name: "entries".into() },
                ResultField { name: "name".into() },
                ResultField { name: "ocrEnabled".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        dlp_profile::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allowed_match_count: hashmap.remove("allowedMatchCount").unwrap(),
            context_awareness: hashmap.remove("contextAwareness").unwrap(),
            description: hashmap.remove("description").unwrap(),
            entries: hashmap.remove("entries").unwrap(),
            name: hashmap.remove("name").unwrap(),
            ocr_enabled: hashmap.remove("ocrEnabled").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
