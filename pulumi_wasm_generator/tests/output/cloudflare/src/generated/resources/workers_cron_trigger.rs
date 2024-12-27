/// Worker Cron Triggers allow users to map a cron expression to a Worker script
/// using a `ScheduledEvent` listener that enables Workers to be executed on a
/// schedule. Worker Cron Triggers are ideal for running periodic jobs for
/// maintenance or calling third-party APIs to collect up-to-date data.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleScript:
///     type: cloudflare:WorkersScript
///     name: example_script
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example-script
///       content:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: path/to/my.js
///           Return: result
///   exampleTrigger:
///     type: cloudflare:WorkersCronTrigger
///     name: example_trigger
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       scriptName: ${exampleScript.name}
///       schedules:
///         - '*/5 * * * *'
///         - 10 7 * * mon-fri
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersCronTrigger:WorkersCronTrigger example <account_id>/<script_name>
/// ```
///
pub mod workers_cron_trigger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersCronTriggerArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Cron expressions to execute the Worker script.
        #[builder(into)]
        pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
        /// Worker script to target for the schedules.
        #[builder(into)]
        pub script_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersCronTriggerResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Cron expressions to execute the Worker script.
        pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
        /// Worker script to target for the schedules.
        pub script_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkersCronTriggerArgs) -> WorkersCronTriggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let schedules_binding = args.schedules.get_inner();
        let script_name_binding = args.script_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersCronTrigger:WorkersCronTrigger".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "schedules".into(),
                    value: &schedules_binding,
                },
                register_interface::ObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "schedules".into(),
                },
                register_interface::ResultField {
                    name: "scriptName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkersCronTriggerResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedules").unwrap(),
            ),
            script_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptName").unwrap(),
            ),
        }
    }
}
