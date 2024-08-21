use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::rate_limit;
use crate::Component;
use std::collections::HashMap;

impl rate_limit::Guest for Component {
    fn invoke(name: String, args: rate_limit::Args) -> rate_limit::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/rateLimit:RateLimit".into(),
            name,
            object: vec![
                ObjectField {
                    name: "action".into(),
                    value: args.action,
                },
                ObjectField {
                    name: "bypassUrlPatterns".into(),
                    value: args.bypass_url_patterns,
                },
                ObjectField {
                    name: "correlate".into(),
                    value: args.correlate,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disabled".into(),
                    value: args.disabled,
                },
                ObjectField {
                    name: "match".into(),
                    value: args.match_,
                },
                ObjectField {
                    name: "period".into(),
                    value: args.period,
                },
                ObjectField {
                    name: "threshold".into(),
                    value: args.threshold,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "action".into(),
                },
                ResultField {
                    name: "bypassUrlPatterns".into(),
                },
                ResultField {
                    name: "correlate".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disabled".into(),
                },
                ResultField {
                    name: "match".into(),
                },
                ResultField {
                    name: "period".into(),
                },
                ResultField {
                    name: "threshold".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        rate_limit::Res {
            action: hashmap.remove("action").unwrap(),
            bypass_url_patterns: hashmap.remove("bypassUrlPatterns").unwrap(),
            correlate: hashmap.remove("correlate").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disabled: hashmap.remove("disabled").unwrap(),
            match_: hashmap.remove("match").unwrap(),
            period: hashmap.remove("period").unwrap(),
            threshold: hashmap.remove("threshold").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
