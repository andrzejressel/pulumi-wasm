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
pub mod watcher {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WatcherArgs {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// A description of this Automation Watcher.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A string of etag assigned to this Automation Watcher.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the frequency at which the watcher is invoked.
        #[builder(into)]
        pub execution_frequency_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The Azure Region where the Automation Watcher should exist. Changing this forces a new Automation Watcher to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Automation Watcher. Changing this forces a new Automation Watcher to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the name of an existing runbook this watcher is attached to. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub script_name: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of key-vaule parameters. Changing this forces a new Automation watcher to be created.
        #[builder(into, default)]
        pub script_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify the name of the Hybrid work group the watcher will run on.
        #[builder(into)]
        pub script_run_on: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Automation Watcher.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WatcherResult {
        /// The ID of Automation Account to manage this Watcher. Changing this forces a new Watcher to be created.
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// A description of this Automation Watcher.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A string of etag assigned to this Automation Watcher.
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the frequency at which the watcher is invoked.
        pub execution_frequency_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The Azure Region where the Automation Watcher should exist. Changing this forces a new Automation Watcher to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Automation Watcher. Changing this forces a new Automation Watcher to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specify the name of an existing runbook this watcher is attached to. Changing this forces a new Automation to be created.
        pub script_name: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of key-vaule parameters. Changing this forces a new Automation watcher to be created.
        pub script_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify the name of the Hybrid work group the watcher will run on.
        pub script_run_on: pulumi_wasm_rust::Output<String>,
        /// The current status of the Automation Watcher.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Automation Watcher.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WatcherArgs) -> WatcherResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_id_binding = args.automation_account_id.get_inner();
        let description_binding = args.description.get_inner();
        let etag_binding = args.etag.get_inner();
        let execution_frequency_in_seconds_binding = args
            .execution_frequency_in_seconds
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let script_name_binding = args.script_name.get_inner();
        let script_parameters_binding = args.script_parameters.get_inner();
        let script_run_on_binding = args.script_run_on.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "executionFrequencyInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scriptName".into(),
                },
                register_interface::ResultField {
                    name: "scriptParameters".into(),
                },
                register_interface::ResultField {
                    name: "scriptRunOn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WatcherResult {
            automation_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            execution_frequency_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionFrequencyInSeconds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            script_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptName").unwrap(),
            ),
            script_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptParameters").unwrap(),
            ),
            script_run_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptRunOn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
