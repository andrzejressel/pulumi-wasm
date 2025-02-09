/// Manages a Automation Runbook.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: account1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleRunBook:
///     type: azure:automation:RunBook
///     name: example
///     properties:
///       name: Get-AzureVMTutorial
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       logVerbose: 'true'
///       logProgress: 'true'
///       description: This is an example runbook
///       runbookType: PowerShellWorkflow
///       publishContentLink:
///         uri: https://raw.githubusercontent.com/Azure/azure-quickstart-templates/c4935ffb69246a6058eb24f54640f53f69d3ac9f/101-automation-runbook-getvms/Runbooks/Get-AzureVMTutorial.ps1
/// ```
///
/// ## Import
///
/// Automation Runbooks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/runBook:RunBook Get-AzureVMTutorial /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/runbooks/Get-AzureVMTutorial
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod run_book {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RunBookArgs {
        /// The name of the automation account in which the Runbook is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The desired content of the runbook.
        ///
        /// > **NOTE** The Azure API requires a `publish_content_link` to be supplied even when specifying your own `content`.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description for this credential.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `draft` block as defined below.
        #[builder(into, default)]
        pub draft: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automation::RunBookDraft>,
        >,
        /// One or more `job_schedule` block as defined below.
        ///
        /// > **NOTE** AzureRM provides a stand-alone azure.automation.JobSchedule and this inlined `job_schedule` property to manage the job schedules. At this time you should choose one of them to manage the job schedule resources.
        #[builder(into, default)]
        pub job_schedules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::automation::RunBookJobSchedule>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the activity-level tracing options of the runbook, available only for Graphical runbooks. Possible values are `0` for None, `9` for Basic, and `15` for Detailed. Must turn on Verbose logging in order to see the tracing.
        #[builder(into, default)]
        pub log_activity_trace_level: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Progress log option.
        #[builder(into)]
        pub log_progress: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Verbose log option.
        #[builder(into)]
        pub log_verbose: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Specifies the name of the Runbook. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One `publish_content_link` block as defined below.
        #[builder(into, default)]
        pub publish_content_link: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automation::RunBookPublishContentLink>,
        >,
        /// The name of the resource group in which the Runbook is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the runbook - can be either `Graph`, `GraphPowerShell`, `GraphPowerShellWorkflow`, `PowerShellWorkflow`, `PowerShell`, `PowerShell72`, `Python3`, `Python2` or `Script`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub runbook_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RunBookResult {
        /// The name of the automation account in which the Runbook is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The desired content of the runbook.
        ///
        /// > **NOTE** The Azure API requires a `publish_content_link` to be supplied even when specifying your own `content`.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// A description for this credential.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `draft` block as defined below.
        pub draft: pulumi_gestalt_rust::Output<
            Option<super::super::types::automation::RunBookDraft>,
        >,
        /// One or more `job_schedule` block as defined below.
        ///
        /// > **NOTE** AzureRM provides a stand-alone azure.automation.JobSchedule and this inlined `job_schedule` property to manage the job schedules. At this time you should choose one of them to manage the job schedule resources.
        pub job_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::automation::RunBookJobSchedule>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the activity-level tracing options of the runbook, available only for Graphical runbooks. Possible values are `0` for None, `9` for Basic, and `15` for Detailed. Must turn on Verbose logging in order to see the tracing.
        pub log_activity_trace_level: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Progress log option.
        pub log_progress: pulumi_gestalt_rust::Output<bool>,
        /// Verbose log option.
        pub log_verbose: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the Runbook. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One `publish_content_link` block as defined below.
        pub publish_content_link: pulumi_gestalt_rust::Output<
            Option<super::super::types::automation::RunBookPublishContentLink>,
        >,
        /// The name of the resource group in which the Runbook is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the runbook - can be either `Graph`, `GraphPowerShell`, `GraphPowerShellWorkflow`, `PowerShellWorkflow`, `PowerShell`, `PowerShell72`, `Python3`, `Python2` or `Script`. Changing this forces a new resource to be created.
        pub runbook_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RunBookArgs,
    ) -> RunBookResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding_1 = args
            .automation_account_name
            .get_output(context);
        let automation_account_name_binding = automation_account_name_binding_1
            .get_inner();
        let content_binding_1 = args.content.get_output(context);
        let content_binding = content_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let draft_binding_1 = args.draft.get_output(context);
        let draft_binding = draft_binding_1.get_inner();
        let job_schedules_binding_1 = args.job_schedules.get_output(context);
        let job_schedules_binding = job_schedules_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let log_activity_trace_level_binding_1 = args
            .log_activity_trace_level
            .get_output(context);
        let log_activity_trace_level_binding = log_activity_trace_level_binding_1
            .get_inner();
        let log_progress_binding_1 = args.log_progress.get_output(context);
        let log_progress_binding = log_progress_binding_1.get_inner();
        let log_verbose_binding_1 = args.log_verbose.get_output(context);
        let log_verbose_binding = log_verbose_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let publish_content_link_binding_1 = args
            .publish_content_link
            .get_output(context);
        let publish_content_link_binding = publish_content_link_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let runbook_type_binding_1 = args.runbook_type.get_output(context);
        let runbook_type_binding = runbook_type_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/runBook:RunBook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "draft".into(),
                    value: &draft_binding,
                },
                register_interface::ObjectField {
                    name: "jobSchedules".into(),
                    value: &job_schedules_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logActivityTraceLevel".into(),
                    value: &log_activity_trace_level_binding,
                },
                register_interface::ObjectField {
                    name: "logProgress".into(),
                    value: &log_progress_binding,
                },
                register_interface::ObjectField {
                    name: "logVerbose".into(),
                    value: &log_verbose_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publishContentLink".into(),
                    value: &publish_content_link_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "runbookType".into(),
                    value: &runbook_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RunBookResult {
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            draft: pulumi_gestalt_rust::__private::into_domain(o.extract_field("draft")),
            job_schedules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jobSchedules"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_activity_trace_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logActivityTraceLevel"),
            ),
            log_progress: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logProgress"),
            ),
            log_verbose: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logVerbose"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            publish_content_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publishContentLink"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            runbook_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runbookType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
