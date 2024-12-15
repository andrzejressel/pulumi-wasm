use std::collections::HashMap;
use crate::bindings::exports::pulumi::docker::get_logs;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_logs::Guest for Component {
    fn invoke(
        args: get_logs::Args
    ) -> get_logs::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "docker:index/getLogs:getLogs".into(),
            object: vec![
                ObjectField { name: "details".into(), value: args.details },
                ObjectField { name: "discardHeaders".into(), value: args.discard_headers },
                ObjectField { name: "follow".into(), value: args.follow },
                ObjectField { name: "logsListStringEnabled".into(), value: args.logs_list_string_enabled },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "showStderr".into(), value: args.show_stderr },
                ObjectField { name: "showStdout".into(), value: args.show_stdout },
                ObjectField { name: "since".into(), value: args.since },
                ObjectField { name: "tail".into(), value: args.tail },
                ObjectField { name: "timestamps".into(), value: args.timestamps },
                ObjectField { name: "until".into(), value: args.until },
            ],
            results: vec![
                ResultField { name: "details".into() },
                ResultField { name: "discardHeaders".into() },
                ResultField { name: "follow".into() },
                ResultField { name: "id".into() },
                ResultField { name: "logsListStringEnabled".into() },
                ResultField { name: "logsListStrings".into() },
                ResultField { name: "name".into() },
                ResultField { name: "showStderr".into() },
                ResultField { name: "showStdout".into() },
                ResultField { name: "since".into() },
                ResultField { name: "tail".into() },
                ResultField { name: "timestamps".into() },
                ResultField { name: "until".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_logs::Res {
            details: hashmap.remove("details").unwrap(),
            discard_headers: hashmap.remove("discardHeaders").unwrap(),
            follow: hashmap.remove("follow").unwrap(),
            id: hashmap.remove("id").unwrap(),
            logs_list_string_enabled: hashmap.remove("logsListStringEnabled").unwrap(),
            logs_list_strings: hashmap.remove("logsListStrings").unwrap(),
            name: hashmap.remove("name").unwrap(),
            show_stderr: hashmap.remove("showStderr").unwrap(),
            show_stdout: hashmap.remove("showStdout").unwrap(),
            since: hashmap.remove("since").unwrap(),
            tail: hashmap.remove("tail").unwrap(),
            timestamps: hashmap.remove("timestamps").unwrap(),
            until: hashmap.remove("until").unwrap(),
        }
    }
}
