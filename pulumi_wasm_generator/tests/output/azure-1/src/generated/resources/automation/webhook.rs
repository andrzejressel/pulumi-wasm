/// Manages an Automation Runbook's Webhook.
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
///   exampleWebhook:
///     type: azure:automation:Webhook
///     name: example
///     properties:
///       name: TestRunbook_webhook
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       expiryTime: 2021-12-31T00:00:00Z
///       enabled: true
///       runbookName: ${exampleRunBook.name}
///       parameters:
///         input: parameter
/// ```
///
/// ## Import
///
/// Automation Webhooks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/webhook:Webhook TestRunbook_webhook /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/webHooks/TestRunbook_webhook
/// ```
///
pub mod webhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// The name of the automation account in which the Webhook is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Controls if Webhook is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Timestamp when the webhook expires. Changing this forces a new resource to be created.
        #[builder(into)]
        pub expiry_time: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Webhook. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of input parameters passed to runbook.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the resource group in which the Webhook is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the hybrid worker group the Webhook job will run on.
        #[builder(into, default)]
        pub run_on_worker_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the Automation Runbook to execute by Webhook.
        #[builder(into)]
        pub runbook_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// URI to initiate the webhook. Can be generated using [Generate URI API](https://docs.microsoft.com/rest/api/automation/webhook/generate-uri). By default, new URI is generated on each new resource creation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// The name of the automation account in which the Webhook is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// Controls if Webhook is enabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Timestamp when the webhook expires. Changing this forces a new resource to be created.
        pub expiry_time: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Webhook. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of input parameters passed to runbook.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the resource group in which the Webhook is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Name of the hybrid worker group the Webhook job will run on.
        pub run_on_worker_group: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Automation Runbook to execute by Webhook.
        pub runbook_name: pulumi_wasm_rust::Output<String>,
        /// URI to initiate the webhook. Can be generated using [Generate URI API](https://docs.microsoft.com/rest/api/automation/webhook/generate-uri). By default, new URI is generated on each new resource creation. Changing this forces a new resource to be created.
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebhookArgs,
    ) -> WebhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let expiry_time_binding = args.expiry_time.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let run_on_worker_group_binding = args
            .run_on_worker_group
            .get_output(context)
            .get_inner();
        let runbook_name_binding = args.runbook_name.get_output(context).get_inner();
        let uri_binding = args.uri.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/webhook:Webhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "expiryTime".into(),
                    value: &expiry_time_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "runOnWorkerGroup".into(),
                    value: &run_on_worker_group_binding,
                },
                register_interface::ObjectField {
                    name: "runbookName".into(),
                    value: &runbook_name_binding,
                },
                register_interface::ObjectField {
                    name: "uri".into(),
                    value: &uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "expiryTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "runOnWorkerGroup".into(),
                },
                register_interface::ResultField {
                    name: "runbookName".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebhookResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            expiry_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiryTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            run_on_worker_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runOnWorkerGroup").unwrap(),
            ),
            runbook_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runbookName").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uri").unwrap()),
        }
    }
}
