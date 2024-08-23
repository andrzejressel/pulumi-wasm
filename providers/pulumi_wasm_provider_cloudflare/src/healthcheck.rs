use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::healthcheck;
use crate::Component;
use std::collections::HashMap;

impl healthcheck::Guest for Component {
    fn invoke(name: String, args: healthcheck::Args) -> healthcheck::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/healthcheck:Healthcheck".into(),
            name,
            object: vec![
                ObjectField {
                    name: "address".into(),
                    value: args.address,
                },
                ObjectField {
                    name: "allowInsecure".into(),
                    value: args.allow_insecure,
                },
                ObjectField {
                    name: "checkRegions".into(),
                    value: args.check_regions,
                },
                ObjectField {
                    name: "consecutiveFails".into(),
                    value: args.consecutive_fails,
                },
                ObjectField {
                    name: "consecutiveSuccesses".into(),
                    value: args.consecutive_successes,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expectedBody".into(),
                    value: args.expected_body,
                },
                ObjectField {
                    name: "expectedCodes".into(),
                    value: args.expected_codes,
                },
                ObjectField {
                    name: "followRedirects".into(),
                    value: args.follow_redirects,
                },
                ObjectField {
                    name: "headers".into(),
                    value: args.headers,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "method".into(),
                    value: args.method,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "path".into(),
                    value: args.path,
                },
                ObjectField {
                    name: "port".into(),
                    value: args.port,
                },
                ObjectField {
                    name: "retries".into(),
                    value: args.retries,
                },
                ObjectField {
                    name: "suspended".into(),
                    value: args.suspended,
                },
                ObjectField {
                    name: "timeout".into(),
                    value: args.timeout,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "address".into(),
                },
                ResultField {
                    name: "allowInsecure".into(),
                },
                ResultField {
                    name: "checkRegions".into(),
                },
                ResultField {
                    name: "consecutiveFails".into(),
                },
                ResultField {
                    name: "consecutiveSuccesses".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expectedBody".into(),
                },
                ResultField {
                    name: "expectedCodes".into(),
                },
                ResultField {
                    name: "followRedirects".into(),
                },
                ResultField {
                    name: "headers".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "method".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "path".into(),
                },
                ResultField {
                    name: "port".into(),
                },
                ResultField {
                    name: "retries".into(),
                },
                ResultField {
                    name: "suspended".into(),
                },
                ResultField {
                    name: "timeout".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        healthcheck::Res {
            address: hashmap.remove("address").unwrap(),
            allow_insecure: hashmap.remove("allowInsecure").unwrap(),
            check_regions: hashmap.remove("checkRegions").unwrap(),
            consecutive_fails: hashmap.remove("consecutiveFails").unwrap(),
            consecutive_successes: hashmap.remove("consecutiveSuccesses").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expected_body: hashmap.remove("expectedBody").unwrap(),
            expected_codes: hashmap.remove("expectedCodes").unwrap(),
            follow_redirects: hashmap.remove("followRedirects").unwrap(),
            headers: hashmap.remove("headers").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            method: hashmap.remove("method").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            path: hashmap.remove("path").unwrap(),
            port: hashmap.remove("port").unwrap(),
            retries: hashmap.remove("retries").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            timeout: hashmap.remove("timeout").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
