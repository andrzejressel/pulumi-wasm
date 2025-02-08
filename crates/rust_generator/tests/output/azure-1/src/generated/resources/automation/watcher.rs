/// Manages an Automation Wacher.
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
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleHybridRunbookWorkerGroup:
///     type: azure:automation:HybridRunbookWorkerGroup
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
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
///   exampleWatcher:
///     type: azure:automation:Watcher
///     name: example
///     properties:
///       name: example
///       automationAccountId: ${exampleAccount.id}
///       location: West Europe
///       scriptName: ${exampleRunBook.name}
///       scriptRunOn: ${exampleHybridRunbookWorkerGroup.name}
///       description: example-watcher desc
///       executionFrequencyInSeconds: 42
///       tags:
///         foo: bar
///       scriptParameters:
///         foo: bar
/// ```
///
/// ## Import
///
/// Automation Watchers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/watcher:Watcher example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/watchers/watch1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod watcher {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WatcherArgs {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of this Automation Watcher.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A string of etag assigned to this Automation Watcher.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the frequency at which the watcher is invoked.
        #[builder(into)]
        pub execution_frequency_in_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Azure Region where the Automation Watcher should exist. Changing this forces a new Automation Watcher to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Automation Watcher. Changing this forces a new Automation Watcher to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the name of an existing runbook this watcher is attached to. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub script_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of key-vaule parameters. Changing this forces a new Automation watcher to be created.
        #[builder(into, default)]
        pub script_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify the name of the Hybrid work group the watcher will run on.
        #[builder(into)]
        pub script_run_on: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Automation Watcher.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WatcherResult {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        pub automation_account_id: pulumi_gestalt_rust::Output<String>,
        /// A description of this Automation Watcher.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A string of etag assigned to this Automation Watcher.
        pub etag: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify the frequency at which the watcher is invoked.
        pub execution_frequency_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The Azure Region where the Automation Watcher should exist. Changing this forces a new Automation Watcher to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Automation Watcher. Changing this forces a new Automation Watcher to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specify the name of an existing runbook this watcher is attached to. Changing this forces a new Automation to be created.
        pub script_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of key-vaule parameters. Changing this forces a new Automation watcher to be created.
        pub script_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify the name of the Hybrid work group the watcher will run on.
        pub script_run_on: pulumi_gestalt_rust::Output<String>,
        /// The current status of the Automation Watcher.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Automation Watcher.
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
        args: WatcherArgs,
    ) -> WatcherResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_id_binding = args
            .automation_account_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let execution_frequency_in_seconds_binding = args
            .execution_frequency_in_seconds
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let script_name_binding = args.script_name.get_output(context).get_inner();
        let script_parameters_binding = args
            .script_parameters
            .get_output(context)
            .get_inner();
        let script_run_on_binding = args.script_run_on.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/watcher:Watcher".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountId".into(),
                    value: &automation_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "executionFrequencyInSeconds".into(),
                    value: &execution_frequency_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding,
                },
                register_interface::ObjectField {
                    name: "scriptParameters".into(),
                    value: &script_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "scriptRunOn".into(),
                    value: &script_run_on_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WatcherResult {
            automation_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            execution_frequency_in_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionFrequencyInSeconds"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            script_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptName"),
            ),
            script_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptParameters"),
            ),
            script_run_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scriptRunOn"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
