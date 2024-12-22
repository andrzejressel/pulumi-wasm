use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::workers_script;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl workers_script::Guest for Component {
    fn invoke(
        name: String,
        args: workers_script::Args
    ) -> workers_script::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersScript:WorkersScript".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "analyticsEngineBindings".into(), value: args.analytics_engine_bindings },
                ObjectField { name: "compatibilityDate".into(), value: args.compatibility_date },
                ObjectField { name: "compatibilityFlags".into(), value: args.compatibility_flags },
                ObjectField { name: "content".into(), value: args.content },
                ObjectField { name: "d1DatabaseBindings".into(), value: args.d1_database_bindings },
                ObjectField { name: "dispatchNamespace".into(), value: args.dispatch_namespace },
                ObjectField { name: "hyperdriveConfigBindings".into(), value: args.hyperdrive_config_bindings },
                ObjectField { name: "kvNamespaceBindings".into(), value: args.kv_namespace_bindings },
                ObjectField { name: "logpush".into(), value: args.logpush },
                ObjectField { name: "module".into(), value: args.module },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "placements".into(), value: args.placements },
                ObjectField { name: "plainTextBindings".into(), value: args.plain_text_bindings },
                ObjectField { name: "queueBindings".into(), value: args.queue_bindings },
                ObjectField { name: "r2BucketBindings".into(), value: args.r2_bucket_bindings },
                ObjectField { name: "secretTextBindings".into(), value: args.secret_text_bindings },
                ObjectField { name: "serviceBindings".into(), value: args.service_bindings },
                ObjectField { name: "tags".into(), value: args.tags },
                ObjectField { name: "webassemblyBindings".into(), value: args.webassembly_bindings },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "analyticsEngineBindings".into() },
                ResultField { name: "compatibilityDate".into() },
                ResultField { name: "compatibilityFlags".into() },
                ResultField { name: "content".into() },
                ResultField { name: "d1DatabaseBindings".into() },
                ResultField { name: "dispatchNamespace".into() },
                ResultField { name: "hyperdriveConfigBindings".into() },
                ResultField { name: "kvNamespaceBindings".into() },
                ResultField { name: "logpush".into() },
                ResultField { name: "module".into() },
                ResultField { name: "name".into() },
                ResultField { name: "placements".into() },
                ResultField { name: "plainTextBindings".into() },
                ResultField { name: "queueBindings".into() },
                ResultField { name: "r2BucketBindings".into() },
                ResultField { name: "secretTextBindings".into() },
                ResultField { name: "serviceBindings".into() },
                ResultField { name: "tags".into() },
                ResultField { name: "webassemblyBindings".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        workers_script::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            analytics_engine_bindings: hashmap.remove("analyticsEngineBindings").unwrap(),
            compatibility_date: hashmap.remove("compatibilityDate").unwrap(),
            compatibility_flags: hashmap.remove("compatibilityFlags").unwrap(),
            content: hashmap.remove("content").unwrap(),
            d1_database_bindings: hashmap.remove("d1DatabaseBindings").unwrap(),
            dispatch_namespace: hashmap.remove("dispatchNamespace").unwrap(),
            hyperdrive_config_bindings: hashmap.remove("hyperdriveConfigBindings").unwrap(),
            kv_namespace_bindings: hashmap.remove("kvNamespaceBindings").unwrap(),
            logpush: hashmap.remove("logpush").unwrap(),
            module: hashmap.remove("module").unwrap(),
            name: hashmap.remove("name").unwrap(),
            placements: hashmap.remove("placements").unwrap(),
            plain_text_bindings: hashmap.remove("plainTextBindings").unwrap(),
            queue_bindings: hashmap.remove("queueBindings").unwrap(),
            r2_bucket_bindings: hashmap.remove("r2BucketBindings").unwrap(),
            secret_text_bindings: hashmap.remove("secretTextBindings").unwrap(),
            service_bindings: hashmap.remove("serviceBindings").unwrap(),
            tags: hashmap.remove("tags").unwrap(),
            webassembly_bindings: hashmap.remove("webassemblyBindings").unwrap(),
        }
    }
}
