/// Manages an Action Group within Azure Monitor.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: monitoring-resources
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: workspace-01
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: CriticalAlertsAction
///       resourceGroupName: ${example.name}
///       shortName: p0action
///       armRoleReceivers:
///         - name: armroleaction
///           roleId: de139f84-1756-47ae-9be6-808fbbe84772
///           useCommonAlertSchema: true
///       automationRunbookReceivers:
///         - name: action_name_1
///           automationAccountId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg-runbooks/providers/Microsoft.Automation/automationAccounts/aaa001
///           runbookName: my runbook
///           webhookResourceId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg-runbooks/providers/Microsoft.Automation/automationAccounts/aaa001/webHooks/webhook_alert
///           isGlobalRunbook: true
///           serviceUri: https://s13events.azure-automation.net/webhooks?token=randomtoken
///           useCommonAlertSchema: true
///       azureAppPushReceivers:
///         - name: pushtoadmin
///           emailAddress: admin@contoso.com
///       azureFunctionReceivers:
///         - name: funcaction
///           functionAppResourceId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg-funcapp/providers/Microsoft.Web/sites/funcapp
///           functionName: myfunc
///           httpTriggerUrl: https://example.com/trigger
///           useCommonAlertSchema: true
///       emailReceivers:
///         - name: sendtoadmin
///           emailAddress: admin@contoso.com
///         - name: sendtodevops
///           emailAddress: devops@contoso.com
///           useCommonAlertSchema: true
///       eventHubReceivers:
///         - name: sendtoeventhub
///           eventHubNamespace: eventhubnamespace
///           eventHubName: eventhub1
///           subscriptionId: 00000000-0000-0000-0000-000000000000
///           useCommonAlertSchema: false
///       itsmReceivers:
///         - name: createorupdateticket
///           workspaceId: ${current.subscriptionId}|${exampleAnalyticsWorkspace.workspaceId}
///           connectionId: 53de6956-42b4-41ba-be3c-b154cdf17b13
///           ticketConfiguration: '{"PayloadRevision":0,"WorkItemType":"Incident","UseTemplate":false,"WorkItemData":"{}","CreateOneWIPerCI":false}'
///           region: southcentralus
///       logicAppReceivers:
///         - name: logicappaction
///           resourceId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg-logicapp/providers/Microsoft.Logic/workflows/logicapp
///           callbackUrl: https://logicapptriggerurl/...
///           useCommonAlertSchema: true
///       smsReceivers:
///         - name: oncallmsg
///           countryCode: '1'
///           phoneNumber: '1231231234'
///       voiceReceivers:
///         - name: remotesupport
///           countryCode: '86'
///           phoneNumber: '13888888888'
///       webhookReceivers:
///         - name: callmyapiaswell
///           serviceUri: http://example.com/alert
///           useCommonAlertSchema: true
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Action Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/actionGroup:ActionGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/actionGroups/myagname
/// ```
///
pub mod action_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionGroupArgs {
        /// One or more `arm_role_receiver` blocks as defined below.
        #[builder(into, default)]
        pub arm_role_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupArmRoleReceiver>>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        #[builder(into, default)]
        pub automation_runbook_receivers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::monitoring::ActionGroupAutomationRunbookReceiver,
                >,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        #[builder(into, default)]
        pub azure_app_push_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupAzureAppPushReceiver>>,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        #[builder(into, default)]
        pub azure_function_receivers: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::monitoring::ActionGroupAzureFunctionReceiver>,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        #[builder(into, default)]
        pub email_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEmailReceiver>>,
        >,
        /// Whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `event_hub_receiver` blocks as defined below.
        #[builder(into, default)]
        pub event_hub_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEventHubReceiver>>,
        >,
        /// One or more `itsm_receiver` blocks as defined below.
        #[builder(into, default)]
        pub itsm_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupItsmReceiver>>,
        >,
        /// The Azure Region where the Action Group should exist. Changing this forces a new Action Group to be created. Defaults to `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `logic_app_receiver` blocks as defined below.
        #[builder(into, default)]
        pub logic_app_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupLogicAppReceiver>>,
        >,
        /// The name of the Action Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Action Group instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The short name of the action group. This will be used in SMS messages.
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// One or more `sms_receiver` blocks as defined below.
        #[builder(into, default)]
        pub sms_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupSmsReceiver>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        #[builder(into, default)]
        pub voice_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupVoiceReceiver>>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        #[builder(into, default)]
        pub webhook_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupWebhookReceiver>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActionGroupResult {
        /// One or more `arm_role_receiver` blocks as defined below.
        pub arm_role_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupArmRoleReceiver>>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        pub automation_runbook_receivers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::monitoring::ActionGroupAutomationRunbookReceiver,
                >,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        pub azure_app_push_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupAzureAppPushReceiver>>,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        pub azure_function_receivers: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::monitoring::ActionGroupAzureFunctionReceiver>,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        pub email_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEmailReceiver>>,
        >,
        /// Whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `event_hub_receiver` blocks as defined below.
        pub event_hub_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEventHubReceiver>>,
        >,
        /// One or more `itsm_receiver` blocks as defined below.
        pub itsm_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupItsmReceiver>>,
        >,
        /// The Azure Region where the Action Group should exist. Changing this forces a new Action Group to be created. Defaults to `global`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// One or more `logic_app_receiver` blocks as defined below.
        pub logic_app_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupLogicAppReceiver>>,
        >,
        /// The name of the Action Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Action Group instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The short name of the action group. This will be used in SMS messages.
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// One or more `sms_receiver` blocks as defined below.
        pub sms_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupSmsReceiver>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        pub voice_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupVoiceReceiver>>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        pub webhook_receivers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupWebhookReceiver>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ActionGroupArgs) -> ActionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arm_role_receivers_binding = args.arm_role_receivers.get_inner();
        let automation_runbook_receivers_binding = args
            .automation_runbook_receivers
            .get_inner();
        let azure_app_push_receivers_binding = args.azure_app_push_receivers.get_inner();
        let azure_function_receivers_binding = args.azure_function_receivers.get_inner();
        let email_receivers_binding = args.email_receivers.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let event_hub_receivers_binding = args.event_hub_receivers.get_inner();
        let itsm_receivers_binding = args.itsm_receivers.get_inner();
        let location_binding = args.location.get_inner();
        let logic_app_receivers_binding = args.logic_app_receivers.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let short_name_binding = args.short_name.get_inner();
        let sms_receivers_binding = args.sms_receivers.get_inner();
        let tags_binding = args.tags.get_inner();
        let voice_receivers_binding = args.voice_receivers.get_inner();
        let webhook_receivers_binding = args.webhook_receivers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/actionGroup:ActionGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "armRoleReceivers".into(),
                    value: &arm_role_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "automationRunbookReceivers".into(),
                    value: &automation_runbook_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "azureAppPushReceivers".into(),
                    value: &azure_app_push_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "azureFunctionReceivers".into(),
                    value: &azure_function_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "emailReceivers".into(),
                    value: &email_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "eventHubReceivers".into(),
                    value: &event_hub_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "itsmReceivers".into(),
                    value: &itsm_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logicAppReceivers".into(),
                    value: &logic_app_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
                register_interface::ObjectField {
                    name: "smsReceivers".into(),
                    value: &sms_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "voiceReceivers".into(),
                    value: &voice_receivers_binding,
                },
                register_interface::ObjectField {
                    name: "webhookReceivers".into(),
                    value: &webhook_receivers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "armRoleReceivers".into(),
                },
                register_interface::ResultField {
                    name: "automationRunbookReceivers".into(),
                },
                register_interface::ResultField {
                    name: "azureAppPushReceivers".into(),
                },
                register_interface::ResultField {
                    name: "azureFunctionReceivers".into(),
                },
                register_interface::ResultField {
                    name: "emailReceivers".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "eventHubReceivers".into(),
                },
                register_interface::ResultField {
                    name: "itsmReceivers".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logicAppReceivers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "smsReceivers".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "voiceReceivers".into(),
                },
                register_interface::ResultField {
                    name: "webhookReceivers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ActionGroupResult {
            arm_role_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("armRoleReceivers").unwrap(),
            ),
            automation_runbook_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationRunbookReceivers").unwrap(),
            ),
            azure_app_push_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureAppPushReceivers").unwrap(),
            ),
            azure_function_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureFunctionReceivers").unwrap(),
            ),
            email_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailReceivers").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            event_hub_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubReceivers").unwrap(),
            ),
            itsm_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("itsmReceivers").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logic_app_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicAppReceivers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            sms_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsReceivers").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            voice_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceReceivers").unwrap(),
            ),
            webhook_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webhookReceivers").unwrap(),
            ),
        }
    }
}