/// Manages a SMS integration for a Bot Channel
///
/// > **Note** A bot can only have a single SMS Channel associated with it.
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
///   exampleChannelsRegistration:
///     type: azure:bot:ChannelsRegistration
///     name: example
///     properties:
///       name: example-bcr
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelSms:
///     type: azure:bot:ChannelSms
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       smsChannelAccountSecurityId: BG61f7cf5157f439b084e98256409c2815
///       smsChannelAuthToken: jh8980432610052ed4e29565c5e232f
///       phoneNumber: '+12313803556'
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The SMS Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelSms:ChannelSms example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.BotService/botServices/botService1/channels/SmsChannel
/// ```
///
pub mod channel_sms {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelSmsArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The phone number for the SMS Channel.
        #[builder(into)]
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the SMS Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The account security identifier (SID) for the SMS Channel.
        #[builder(into)]
        pub sms_channel_account_security_id: pulumi_wasm_rust::Output<String>,
        /// The authorization token for the SMS Channel.
        #[builder(into)]
        pub sms_channel_auth_token: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelSmsResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The phone number for the SMS Channel.
        pub phone_number: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the SMS Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The account security identifier (SID) for the SMS Channel.
        pub sms_channel_account_security_id: pulumi_wasm_rust::Output<String>,
        /// The authorization token for the SMS Channel.
        pub sms_channel_auth_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ChannelSmsArgs) -> ChannelSmsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let location_binding = args.location.get_inner();
        let phone_number_binding = args.phone_number.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sms_channel_account_security_id_binding = args
            .sms_channel_account_security_id
            .get_inner();
        let sms_channel_auth_token_binding = args.sms_channel_auth_token.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelSms:ChannelSms".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "smsChannelAccountSecurityId".into(),
                    value: &sms_channel_account_security_id_binding,
                },
                register_interface::ObjectField {
                    name: "smsChannelAuthToken".into(),
                    value: &sms_channel_auth_token_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "botName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumber".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "smsChannelAccountSecurityId".into(),
                },
                register_interface::ResultField {
                    name: "smsChannelAuthToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ChannelSmsResult {
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            phone_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumber").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sms_channel_account_security_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsChannelAccountSecurityId").unwrap(),
            ),
            sms_channel_auth_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsChannelAuthToken").unwrap(),
            ),
        }
    }
}
