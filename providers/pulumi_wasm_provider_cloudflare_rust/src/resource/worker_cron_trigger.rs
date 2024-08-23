pub struct WorkerCronTriggerArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
    pub script_name: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerCronTriggerResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
    pub script_name: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkerCronTriggerArgs) -> WorkerCronTriggerResult {
    let result = crate::bindings::pulumi::cloudflare::worker_cron_trigger::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::worker_cron_trigger::Args {
            account_id: args.account_id.get_inner(),
            schedules: args.schedules.get_inner(),
            script_name: args.script_name.get_inner(),
        },
    );

    WorkerCronTriggerResult {
        account_id: crate::into_domain(result.account_id),
        schedules: crate::into_domain(result.schedules),
        script_name: crate::into_domain(result.script_name),
    }
}
