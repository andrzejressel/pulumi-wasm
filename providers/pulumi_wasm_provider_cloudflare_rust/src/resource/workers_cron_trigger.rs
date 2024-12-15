//! Worker Cron Triggers allow users to map a cron expression to a Worker script
//! using a `ScheduledEvent` listener that enables Workers to be executed on a
//! schedule. Worker Cron Triggers are ideal for running periodic jobs for
//! maintenance or calling third-party APIs to collect up-to-date data.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   exampleScript:
//!     type: cloudflare:WorkersScript
//!     name: example_script
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example-script
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: path/to/my.js
//!           Return: result
//!   exampleTrigger:
//!     type: cloudflare:WorkersCronTrigger
//!     name: example_trigger
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       scriptName: ${exampleScript.name}
//!       schedules:
//!         - '*/5 * * * *'
//!         - 10 7 * * mon-fri
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersCronTrigger:WorkersCronTrigger example <account_id>/<script_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
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
pub fn create(name: &str, args: WorkersCronTriggerArgs) -> WorkersCronTriggerResult {

    let result = crate::bindings::pulumi::cloudflare::workers_cron_trigger::invoke(name, &crate::bindings::pulumi::cloudflare::workers_cron_trigger::Args {
        account_id: &args.account_id.get_inner(),
        schedules: &args.schedules.get_inner(),
        script_name: &args.script_name.get_inner(),
    });

    WorkersCronTriggerResult {
        account_id: crate::into_domain(result.account_id),
        schedules: crate::into_domain(result.schedules),
        script_name: crate::into_domain(result.script_name),
    }
}
