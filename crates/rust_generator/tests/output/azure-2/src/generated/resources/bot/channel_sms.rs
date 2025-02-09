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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelSmsArgs,
    ) -> ChannelSmsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let phone_number_binding = args.phone_number.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sms_channel_account_security_id_binding = args
            .sms_channel_account_security_id
            .get_output(context);
        let sms_channel_auth_token_binding = args
            .sms_channel_auth_token
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelSms:ChannelSms".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: phone_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsChannelAccountSecurityId".into(),
                    value: sms_channel_account_security_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsChannelAuthToken".into(),
                    value: sms_channel_auth_token_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelSmsResult {
            bot_name: o.get_field("botName"),
            location: o.get_field("location"),
            phone_number: o.get_field("phoneNumber"),
            resource_group_name: o.get_field("resourceGroupName"),
            sms_channel_account_security_id: o.get_field("smsChannelAccountSecurityId"),
            sms_channel_auth_token: o.get_field("smsChannelAuthToken"),
        }
    }
}
