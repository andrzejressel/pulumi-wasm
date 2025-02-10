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
///     type: cloudflare:WorkerCronTrigger
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
/// $ pulumi import cloudflare:index/workerCronTrigger:WorkerCronTrigger example <account_id>/<script_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod worker_cron_trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerCronTriggerArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Cron expressions to execute the Worker script.
        #[builder(into)]
        pub schedules: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Worker script to target for the schedules.
        #[builder(into)]
        pub script_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkerCronTriggerResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Cron expressions to execute the Worker script.
        pub schedules: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Worker script to target for the schedules.
        pub script_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkerCronTriggerArgs,
    ) -> WorkerCronTriggerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let schedules_binding = args.schedules.get_output(context);
        let script_name_binding = args.script_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workerCronTrigger:WorkerCronTrigger".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedules".into(),
                    value: schedules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptName".into(),
                    value: script_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkerCronTriggerResult {
            account_id: o.get_field("accountId"),
            schedules: o.get_field("schedules"),
            script_name: o.get_field("scriptName"),
        }
    }
}
