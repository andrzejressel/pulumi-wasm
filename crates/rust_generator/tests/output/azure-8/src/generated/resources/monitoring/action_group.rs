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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod action_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionGroupArgs {
        /// One or more `arm_role_receiver` blocks as defined below.
        #[builder(into, default)]
        pub arm_role_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupArmRoleReceiver>>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        #[builder(into, default)]
        pub automation_runbook_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::monitoring::ActionGroupAutomationRunbookReceiver,
                >,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        #[builder(into, default)]
        pub azure_app_push_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupAzureAppPushReceiver>>,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        #[builder(into, default)]
        pub azure_function_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::monitoring::ActionGroupAzureFunctionReceiver>,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        #[builder(into, default)]
        pub email_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupEmailReceiver>>,
        >,
        /// Whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more `event_hub_receiver` blocks as defined below.
        #[builder(into, default)]
        pub event_hub_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupEventHubReceiver>>,
        >,
        /// One or more `itsm_receiver` blocks as defined below.
        #[builder(into, default)]
        pub itsm_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupItsmReceiver>>,
        >,
        /// The Azure Region where the Action Group should exist. Changing this forces a new Action Group to be created. Defaults to `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `logic_app_receiver` blocks as defined below.
        #[builder(into, default)]
        pub logic_app_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupLogicAppReceiver>>,
        >,
        /// The name of the Action Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Action Group instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The short name of the action group. This will be used in SMS messages.
        #[builder(into)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `sms_receiver` blocks as defined below.
        #[builder(into, default)]
        pub sms_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupSmsReceiver>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        #[builder(into, default)]
        pub voice_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupVoiceReceiver>>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        #[builder(into, default)]
        pub webhook_receivers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::monitoring::ActionGroupWebhookReceiver>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActionGroupResult {
        /// One or more `arm_role_receiver` blocks as defined below.
        pub arm_role_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupArmRoleReceiver>>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        pub automation_runbook_receivers: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::monitoring::ActionGroupAutomationRunbookReceiver,
                >,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        pub azure_app_push_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupAzureAppPushReceiver>>,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        pub azure_function_receivers: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::monitoring::ActionGroupAzureFunctionReceiver>,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        pub email_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEmailReceiver>>,
        >,
        /// Whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more `event_hub_receiver` blocks as defined below.
        pub event_hub_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupEventHubReceiver>>,
        >,
        /// One or more `itsm_receiver` blocks as defined below.
        pub itsm_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupItsmReceiver>>,
        >,
        /// The Azure Region where the Action Group should exist. Changing this forces a new Action Group to be created. Defaults to `global`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// One or more `logic_app_receiver` blocks as defined below.
        pub logic_app_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupLogicAppReceiver>>,
        >,
        /// The name of the Action Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Action Group instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The short name of the action group. This will be used in SMS messages.
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `sms_receiver` blocks as defined below.
        pub sms_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupSmsReceiver>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        pub voice_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupVoiceReceiver>>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        pub webhook_receivers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::monitoring::ActionGroupWebhookReceiver>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActionGroupArgs,
    ) -> ActionGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arm_role_receivers_binding = args.arm_role_receivers.get_output(context);
        let automation_runbook_receivers_binding = args
            .automation_runbook_receivers
            .get_output(context);
        let azure_app_push_receivers_binding = args
            .azure_app_push_receivers
            .get_output(context);
        let azure_function_receivers_binding = args
            .azure_function_receivers
            .get_output(context);
        let email_receivers_binding = args.email_receivers.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let event_hub_receivers_binding = args.event_hub_receivers.get_output(context);
        let itsm_receivers_binding = args.itsm_receivers.get_output(context);
        let location_binding = args.location.get_output(context);
        let logic_app_receivers_binding = args.logic_app_receivers.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let short_name_binding = args.short_name.get_output(context);
        let sms_receivers_binding = args.sms_receivers.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let voice_receivers_binding = args.voice_receivers.get_output(context);
        let webhook_receivers_binding = args.webhook_receivers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/actionGroup:ActionGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "armRoleReceivers".into(),
                    value: &arm_role_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationRunbookReceivers".into(),
                    value: &automation_runbook_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureAppPushReceivers".into(),
                    value: &azure_app_push_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureFunctionReceivers".into(),
                    value: &azure_function_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailReceivers".into(),
                    value: &email_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventHubReceivers".into(),
                    value: &event_hub_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "itsmReceivers".into(),
                    value: &itsm_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicAppReceivers".into(),
                    value: &logic_app_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsReceivers".into(),
                    value: &sms_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceReceivers".into(),
                    value: &voice_receivers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webhookReceivers".into(),
                    value: &webhook_receivers_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActionGroupResult {
            arm_role_receivers: o.get_field("armRoleReceivers"),
            automation_runbook_receivers: o.get_field("automationRunbookReceivers"),
            azure_app_push_receivers: o.get_field("azureAppPushReceivers"),
            azure_function_receivers: o.get_field("azureFunctionReceivers"),
            email_receivers: o.get_field("emailReceivers"),
            enabled: o.get_field("enabled"),
            event_hub_receivers: o.get_field("eventHubReceivers"),
            itsm_receivers: o.get_field("itsmReceivers"),
            location: o.get_field("location"),
            logic_app_receivers: o.get_field("logicAppReceivers"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            short_name: o.get_field("shortName"),
            sms_receivers: o.get_field("smsReceivers"),
            tags: o.get_field("tags"),
            voice_receivers: o.get_field("voiceReceivers"),
            webhook_receivers: o.get_field("webhookReceivers"),
        }
    }
}
