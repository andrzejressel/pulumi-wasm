use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::logpush_job;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl logpush_job::Guest for Component {
    fn invoke(
        name: String,
        args: logpush_job::Args
    ) -> logpush_job::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/logpushJob:LogpushJob".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "dataset".into(), value: args.dataset },
                ObjectField { name: "destinationConf".into(), value: args.destination_conf },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "filter".into(), value: args.filter },
                ObjectField { name: "frequency".into(), value: args.frequency },
                ObjectField { name: "kind".into(), value: args.kind },
                ObjectField { name: "logpullOptions".into(), value: args.logpull_options },
                ObjectField { name: "maxUploadBytes".into(), value: args.max_upload_bytes },
                ObjectField { name: "maxUploadIntervalSeconds".into(), value: args.max_upload_interval_seconds },
                ObjectField { name: "maxUploadRecords".into(), value: args.max_upload_records },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "outputOptions".into(), value: args.output_options },
                ObjectField { name: "ownershipChallenge".into(), value: args.ownership_challenge },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "dataset".into() },
                ResultField { name: "destinationConf".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "filter".into() },
                ResultField { name: "frequency".into() },
                ResultField { name: "kind".into() },
                ResultField { name: "logpullOptions".into() },
                ResultField { name: "maxUploadBytes".into() },
                ResultField { name: "maxUploadIntervalSeconds".into() },
                ResultField { name: "maxUploadRecords".into() },
                ResultField { name: "name".into() },
                ResultField { name: "outputOptions".into() },
                ResultField { name: "ownershipChallenge".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        logpush_job::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            dataset: hashmap.remove("dataset").unwrap(),
            destination_conf: hashmap.remove("destinationConf").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filter: hashmap.remove("filter").unwrap(),
            frequency: hashmap.remove("frequency").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            logpull_options: hashmap.remove("logpullOptions").unwrap(),
            max_upload_bytes: hashmap.remove("maxUploadBytes").unwrap(),
            max_upload_interval_seconds: hashmap.remove("maxUploadIntervalSeconds").unwrap(),
            max_upload_records: hashmap.remove("maxUploadRecords").unwrap(),
            name: hashmap.remove("name").unwrap(),
            output_options: hashmap.remove("outputOptions").unwrap(),
            ownership_challenge: hashmap.remove("ownershipChallenge").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
