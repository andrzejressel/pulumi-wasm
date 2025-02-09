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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_sms {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelSmsArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The phone number for the SMS Channel.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group where the SMS Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The account security identifier (SID) for the SMS Channel.
        #[builder(into)]
        pub sms_channel_account_security_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The authorization token for the SMS Channel.
        #[builder(into)]
        pub sms_channel_auth_token: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelSmsResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The phone number for the SMS Channel.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the SMS Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The account security identifier (SID) for the SMS Channel.
        pub sms_channel_account_security_id: pulumi_gestalt_rust::Output<String>,
        /// The authorization token for the SMS Channel.
        pub sms_channel_auth_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ChannelSmsArgs,
    ) -> ChannelSmsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bot_name_binding_1 = args.bot_name.get_output(context);
        let bot_name_binding = bot_name_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let phone_number_binding_1 = args.phone_number.get_output(context);
        let phone_number_binding = phone_number_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sms_channel_account_security_id_binding_1 = args
            .sms_channel_account_security_id
            .get_output(context);
        let sms_channel_account_security_id_binding = sms_channel_account_security_id_binding_1
            .get_inner();
        let sms_channel_auth_token_binding_1 = args
            .sms_channel_auth_token
            .get_output(context);
        let sms_channel_auth_token_binding = sms_channel_auth_token_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelSms:ChannelSms".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ChannelSmsResult {
            bot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            phone_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("phoneNumber"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sms_channel_account_security_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("smsChannelAccountSecurityId"),
            ),
            sms_channel_auth_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("smsChannelAuthToken"),
            ),
        }
    }
}
