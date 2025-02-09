/// Links an Automation Runbook and Schedule.
///
/// > **NOTE** AzureRM provides this stand-alone azure.automation.JobSchedule and an inlined `job_schedule` property in azurerm_runbook to manage the job schedules. You can only make use of one of these methods to manage a job schedule.
///
/// ## Example Usage
///
/// This is an example of just the Job Schedule.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:automation:JobSchedule
///     properties:
///       resourceGroupName: tf-rgr-automation
///       automationAccountName: tf-automation-account
///       scheduleName: hour
///       runbookName: Get-VirtualMachine
///       parameters:
///         resourcegroup: tf-rgr-vm
///         vmname: TF-VM-01
/// ```
///
/// ## Import
///
/// Automation Job Schedules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/jobSchedule:JobSchedule example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/schedules/schedule1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/runbooks/runbook1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobScheduleArgs {
        /// The name of the Automation Account in which the Job Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The UUID identifying the Automation Job Schedule.
        #[builder(into, default)]
        pub job_schedule_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of key/value pairs corresponding to the arguments that can be passed to the Runbook. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The parameter keys/names must strictly be in lowercase, even if this is not the case in the runbook. This is due to a limitation in Azure Automation where the parameter names are normalized. The values specified don't have this limitation.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the resource group in which the Job Schedule is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of a Hybrid Worker Group the Runbook will be executed on. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub run_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a Runbook to link to a Schedule. It needs to be in the same Automation Account as the Schedule and Job Schedule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub runbook_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Schedule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub schedule_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct JobScheduleResult {
        /// The name of the Automation Account in which the Job Schedule is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The UUID identifying the Automation Job Schedule.
        pub job_schedule_id: pulumi_gestalt_rust::Output<String>,
        /// A map of key/value pairs corresponding to the arguments that can be passed to the Runbook. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The parameter keys/names must strictly be in lowercase, even if this is not the case in the runbook. This is due to a limitation in Azure Automation where the parameter names are normalized. The values specified don't have this limitation.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the resource group in which the Job Schedule is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Resource Manager ID of the Automation Job Schedule.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// Name of a Hybrid Worker Group the Runbook will be executed on. Changing this forces a new resource to be created.
        pub run_on: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of a Runbook to link to a Schedule. It needs to be in the same Automation Account as the Schedule and Job Schedule. Changing this forces a new resource to be created.
        pub runbook_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Schedule. Changing this forces a new resource to be created.
        pub schedule_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobScheduleArgs,
    ) -> JobScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let job_schedule_id_binding = args.job_schedule_id.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let run_on_binding = args.run_on.get_output(context);
        let runbook_name_binding = args.runbook_name.get_output(context);
        let schedule_name_binding = args.schedule_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/jobSchedule:JobSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: automation_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobScheduleId".into(),
                    value: job_schedule_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runOn".into(),
                    value: run_on_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runbookName".into(),
                    value: runbook_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleName".into(),
                    value: schedule_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobScheduleResult {
            automation_account_name: o.get_field("automationAccountName"),
            job_schedule_id: o.get_field("jobScheduleId"),
            parameters: o.get_field("parameters"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_manager_id: o.get_field("resourceManagerId"),
            run_on: o.get_field("runOn"),
            runbook_name: o.get_field("runbookName"),
            schedule_name: o.get_field("scheduleName"),
        }
    }
}
