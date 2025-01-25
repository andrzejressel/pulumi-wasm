pub mod get_action_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActionGroupArgs {
        /// Specifies the name of the Action Group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Action Group is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetActionGroupResult {
        /// One or more `arm_role_receiver` blocks as defined below.
        pub arm_role_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupArmRoleReceiver>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        pub automation_runbook_receivers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAutomationRunbookReceiver,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        pub azure_app_push_receivers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAzureAppPushReceiver,
            >,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        pub azure_function_receivers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAzureFunctionReceiver,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        pub email_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupEmailReceiver>,
        >,
        /// Whether this action group is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// One or more `event_hub_receiver` blocks as defined below.
        pub event_hub_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupEventHubReceiver>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `itsm_receiver` blocks as defined below.
        pub itsm_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupItsmReceiver>,
        >,
        /// One or more `logic_app_receiver` blocks as defined below.
        pub logic_app_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupLogicAppReceiver>,
        >,
        /// The name of the webhook receiver.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The short name of the action group.
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// One or more `sms_receiver` blocks as defined below.
        pub sms_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupSmsReceiver>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        pub voice_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupVoiceReceiver>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        pub webhook_receivers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupWebhookReceiver>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetActionGroupArgs,
    ) -> GetActionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getActionGroup:getActionGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "itsmReceivers".into(),
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
                    name: "voiceReceivers".into(),
                },
                register_interface::ResultField {
                    name: "webhookReceivers".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetActionGroupResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            itsm_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("itsmReceivers").unwrap(),
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
            voice_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceReceivers").unwrap(),
            ),
            webhook_receivers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webhookReceivers").unwrap(),
            ),
        }
    }
}
