/// Manages an Automation Software Update Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: East US
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: example
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
///       description: This is a example runbook for terraform acceptance example
///       runbookType: Python3
///       content: |
///         # Some example content
///         # for Terraform acceptance example
///       tags:
///         ENV: runbook_test
///   exampleSoftwareUpdateConfiguration:
///     type: azure:automation:SoftwareUpdateConfiguration
///     name: example
///     properties:
///       name: example
///       automationAccountId: ${exampleAccount.id}
///       linux:
///         classificationsIncludeds: Security
///         excludedPackages:
///           - apt
///         includedPackages:
///           - vim
///         reboot: IfRequired
///       preTask:
///         source: ${exampleRunBook.name}
///         parameters:
///           COMPUTER_NAME: Foo
///       duration: PT2H2M2S
/// ```
///
/// ## Import
///
/// Automations Software Update Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/softwareUpdateConfiguration:SoftwareUpdateConfiguration example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/softwareUpdateConfigurations/suc1
/// ```
///
pub mod software_update_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SoftwareUpdateConfigurationArgs {
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        #[builder(into)]
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// Maximum time allowed for the software update configuration run. using format `PT[n]H[n]M[n]S` as per ISO8601. Defaults to `PT2H`.
        #[builder(into, default)]
        pub duration: pulumi_wasm_rust::Output<Option<String>>,
        /// A `linux` block as defined below.
        #[builder(into, default)]
        pub linux: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationLinux>,
        >,
        /// The name which should be used for this Automation. Changing this forces a new Automation to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of names of non-Azure machines for the software update configuration.
        #[builder(into, default)]
        pub non_azure_computer_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `post_task` blocks as defined below.
        #[builder(into, default)]
        pub post_task: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationPostTask>,
        >,
        /// A `pre_task` blocks as defined below.
        #[builder(into, default)]
        pub pre_task: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationPreTask>,
        >,
        /// A `schedule` blocks as defined below.
        #[builder(into)]
        pub schedule: pulumi_wasm_rust::Output<
            super::super::types::automation::SoftwareUpdateConfigurationSchedule,
        >,
        /// A `target` blocks as defined below.
        #[builder(into, default)]
        pub target: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationTarget>,
        >,
        /// Specifies a list of Azure Resource IDs of azure virtual machines.
        #[builder(into, default)]
        pub virtual_machine_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `windows` block as defined below.
        ///
        /// > **NOTE:** One of `linux` or `windows` must be specified.
        #[builder(into, default)]
        pub windows: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationWindows>,
        >,
    }
    #[allow(dead_code)]
    pub struct SoftwareUpdateConfigurationResult {
        /// The ID of Automation Account to manage this Source Control. Changing this forces a new Automation Source Control to be created.
        pub automation_account_id: pulumi_wasm_rust::Output<String>,
        /// Maximum time allowed for the software update configuration run. using format `PT[n]H[n]M[n]S` as per ISO8601. Defaults to `PT2H`.
        pub duration: pulumi_wasm_rust::Output<Option<String>>,
        /// The Error code when failed.
        pub error_code: pulumi_wasm_rust::Output<String>,
        /// The Error message indicating why the operation failed.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// A `linux` block as defined below.
        pub linux: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationLinux>,
        >,
        /// The name which should be used for this Automation. Changing this forces a new Automation to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of names of non-Azure machines for the software update configuration.
        pub non_azure_computer_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `post_task` blocks as defined below.
        pub post_task: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationPostTask>,
        >,
        /// A `pre_task` blocks as defined below.
        pub pre_task: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationPreTask>,
        >,
        /// A `schedule` blocks as defined below.
        pub schedule: pulumi_wasm_rust::Output<
            super::super::types::automation::SoftwareUpdateConfigurationSchedule,
        >,
        /// A `target` blocks as defined below.
        pub target: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationTarget>,
        >,
        /// Specifies a list of Azure Resource IDs of azure virtual machines.
        pub virtual_machine_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `windows` block as defined below.
        ///
        /// > **NOTE:** One of `linux` or `windows` must be specified.
        pub windows: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::SoftwareUpdateConfigurationWindows>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SoftwareUpdateConfigurationArgs,
    ) -> SoftwareUpdateConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_id_binding = args.automation_account_id.get_inner();
        let duration_binding = args.duration.get_inner();
        let linux_binding = args.linux.get_inner();
        let name_binding = args.name.get_inner();
        let non_azure_computer_names_binding = args.non_azure_computer_names.get_inner();
        let post_task_binding = args.post_task.get_inner();
        let pre_task_binding = args.pre_task.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let target_binding = args.target.get_inner();
        let virtual_machine_ids_binding = args.virtual_machine_ids.get_inner();
        let windows_binding = args.windows.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/softwareUpdateConfiguration:SoftwareUpdateConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountId".into(),
                    value: &automation_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "linux".into(),
                    value: &linux_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nonAzureComputerNames".into(),
                    value: &non_azure_computer_names_binding,
                },
                register_interface::ObjectField {
                    name: "postTask".into(),
                    value: &post_task_binding,
                },
                register_interface::ObjectField {
                    name: "preTask".into(),
                    value: &pre_task_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineIds".into(),
                    value: &virtual_machine_ids_binding,
                },
                register_interface::ObjectField {
                    name: "windows".into(),
                    value: &windows_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountId".into(),
                },
                register_interface::ResultField {
                    name: "duration".into(),
                },
                register_interface::ResultField {
                    name: "errorCode".into(),
                },
                register_interface::ResultField {
                    name: "errorMessage".into(),
                },
                register_interface::ResultField {
                    name: "linux".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nonAzureComputerNames".into(),
                },
                register_interface::ResultField {
                    name: "postTask".into(),
                },
                register_interface::ResultField {
                    name: "preTask".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineIds".into(),
                },
                register_interface::ResultField {
                    name: "windows".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SoftwareUpdateConfigurationResult {
            automation_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountId").unwrap(),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duration").unwrap(),
            ),
            error_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorCode").unwrap(),
            ),
            error_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorMessage").unwrap(),
            ),
            linux: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linux").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            non_azure_computer_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonAzureComputerNames").unwrap(),
            ),
            post_task: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postTask").unwrap(),
            ),
            pre_task: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preTask").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            virtual_machine_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineIds").unwrap(),
            ),
            windows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windows").unwrap(),
            ),
        }
    }
}