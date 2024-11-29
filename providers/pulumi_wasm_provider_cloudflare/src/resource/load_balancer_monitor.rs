use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::load_balancer_monitor;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl load_balancer_monitor::Guest for Component {
    fn invoke(name: String, args: load_balancer_monitor::Args) -> load_balancer_monitor::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "allowInsecure".into(), value: args.allow_insecure },
                ObjectField { name: "consecutiveDown".into(), value: args.consecutive_down },
                ObjectField { name: "consecutiveUp".into(), value: args.consecutive_up },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "expectedBody".into(), value: args.expected_body },
                ObjectField { name: "expectedCodes".into(), value: args.expected_codes },
                ObjectField { name: "followRedirects".into(), value: args.follow_redirects },
                ObjectField { name: "headers".into(), value: args.headers },
                ObjectField { name: "interval".into(), value: args.interval },
                ObjectField { name: "method".into(), value: args.method },
                ObjectField { name: "path".into(), value: args.path },
                ObjectField { name: "port".into(), value: args.port },
                ObjectField { name: "probeZone".into(), value: args.probe_zone },
                ObjectField { name: "retries".into(), value: args.retries },
                ObjectField { name: "timeout".into(), value: args.timeout },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "allowInsecure".into() },
                ResultField { name: "consecutiveDown".into() },
                ResultField { name: "consecutiveUp".into() },
                ResultField { name: "createdOn".into() },
                ResultField { name: "description".into() },
                ResultField { name: "expectedBody".into() },
                ResultField { name: "expectedCodes".into() },
                ResultField { name: "followRedirects".into() },
                ResultField { name: "headers".into() },
                ResultField { name: "interval".into() },
                ResultField { name: "method".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "path".into() },
                ResultField { name: "port".into() },
                ResultField { name: "probeZone".into() },
                ResultField { name: "retries".into() },
                ResultField { name: "timeout".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        load_balancer_monitor::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_insecure: hashmap.remove("allowInsecure").unwrap(),
            consecutive_down: hashmap.remove("consecutiveDown").unwrap(),
            consecutive_up: hashmap.remove("consecutiveUp").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expected_body: hashmap.remove("expectedBody").unwrap(),
            expected_codes: hashmap.remove("expectedCodes").unwrap(),
            follow_redirects: hashmap.remove("followRedirects").unwrap(),
            headers: hashmap.remove("headers").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            method: hashmap.remove("method").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            path: hashmap.remove("path").unwrap(),
            port: hashmap.remove("port").unwrap(),
            probe_zone: hashmap.remove("probeZone").unwrap(),
            retries: hashmap.remove("retries").unwrap(),
            timeout: hashmap.remove("timeout").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
