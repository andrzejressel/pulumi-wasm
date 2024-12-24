#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkerCronTriggerArgs {
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
pub struct WorkerCronTriggerResult {
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
pub fn create(name: &str, args: WorkerCronTriggerArgs) -> WorkerCronTriggerResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let schedules_binding = args.schedules.get_inner();
    let script_name_binding = args.script_name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workerCronTrigger:WorkerCronTrigger".into(),
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
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "schedules".into() },
            register_interface::ResultField { name : "scriptName".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    WorkerCronTriggerResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        schedules: into_domain(hashmap.remove("schedules").unwrap()),
        script_name: into_domain(hashmap.remove("scriptName").unwrap()),
    }
}
