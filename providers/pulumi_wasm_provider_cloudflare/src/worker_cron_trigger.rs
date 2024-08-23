use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::worker_cron_trigger;
use crate::Component;
use std::collections::HashMap;

impl worker_cron_trigger::Guest for Component {
    fn invoke(name: String, args: worker_cron_trigger::Args) -> worker_cron_trigger::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerCronTrigger:WorkerCronTrigger".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "schedules".into(),
                    value: args.schedules,
                },
                ObjectField {
                    name: "scriptName".into(),
                    value: args.script_name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "schedules".into(),
                },
                ResultField {
                    name: "scriptName".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_cron_trigger::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            schedules: hashmap.remove("schedules").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
        }
    }
}
