/// Manages a Direct Line Speech integration for a Bot Channel
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
///     type: azure:cognitive:Account
///     name: example
///     properties:
///       name: example-cogacct
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       kind: SpeechServices
///       skuName: S0
///   exampleChannelsRegistration:
///     type: azure:bot:ChannelsRegistration
///     name: example
///     properties:
///       name: example-bcr
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelDirectLineSpeech:
///     type: azure:bot:ChannelDirectLineSpeech
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       cognitiveServiceLocation: ${exampleAccount.location}
///       cognitiveServiceAccessKey: ${exampleAccount.primaryAccessKey}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Direct Line Speech Channels can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelDirectLineSpeech:ChannelDirectLineSpeech example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.BotService/botServices/botService1/channels/DirectLineSpeechChannel
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_direct_line_speech {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelDirectLineSpeechArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Cognitive Account this Bot Channel should be associated with.
        #[builder(into, default)]
        pub cognitive_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The access key to access the Cognitive Service.
        #[builder(into)]
        pub cognitive_service_access_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the Cognitive Service resource exists.
        #[builder(into)]
        pub cognitive_service_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The custom speech model id for the Direct Line Speech Channel.
        #[builder(into, default)]
        pub custom_speech_model_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The custom voice deployment id for the Direct Line Speech Channel.
        #[builder(into, default)]
        pub custom_voice_deployment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the Direct Line Speech Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelDirectLineSpeechResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Cognitive Account this Bot Channel should be associated with.
        pub cognitive_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The access key to access the Cognitive Service.
        pub cognitive_service_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the Cognitive Service resource exists.
        pub cognitive_service_location: pulumi_gestalt_rust::Output<String>,
        /// The custom speech model id for the Direct Line Speech Channel.
        pub custom_speech_model_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The custom voice deployment id for the Direct Line Speech Channel.
        pub custom_voice_deployment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the Direct Line Speech Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ChannelDirectLineSpeechArgs,
    ) -> ChannelDirectLineSpeechResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bot_name_binding_1 = args.bot_name.get_output(context);
        let bot_name_binding = bot_name_binding_1.get_inner();
        let cognitive_account_id_binding_1 = args
            .cognitive_account_id
            .get_output(context);
        let cognitive_account_id_binding = cognitive_account_id_binding_1.get_inner();
        let cognitive_service_access_key_binding_1 = args
            .cognitive_service_access_key
            .get_output(context);
        let cognitive_service_access_key_binding = cognitive_service_access_key_binding_1
            .get_inner();
        let cognitive_service_location_binding_1 = args
            .cognitive_service_location
            .get_output(context);
        let cognitive_service_location_binding = cognitive_service_location_binding_1
            .get_inner();
        let custom_speech_model_id_binding_1 = args
            .custom_speech_model_id
            .get_output(context);
        let custom_speech_model_id_binding = custom_speech_model_id_binding_1
            .get_inner();
        let custom_voice_deployment_id_binding_1 = args
            .custom_voice_deployment_id
            .get_output(context);
        let custom_voice_deployment_id_binding = custom_voice_deployment_id_binding_1
            .get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelDirectLineSpeech:ChannelDirectLineSpeech".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "cognitiveAccountId".into(),
                    value: &cognitive_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "cognitiveServiceAccessKey".into(),
                    value: &cognitive_service_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "cognitiveServiceLocation".into(),
                    value: &cognitive_service_location_binding,
                },
                register_interface::ObjectField {
                    name: "customSpeechModelId".into(),
                    value: &custom_speech_model_id_binding,
                },
                register_interface::ObjectField {
                    name: "customVoiceDeploymentId".into(),
                    value: &custom_voice_deployment_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ChannelDirectLineSpeechResult {
            bot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botName"),
            ),
            cognitive_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitiveAccountId"),
            ),
            cognitive_service_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitiveServiceAccessKey"),
            ),
            cognitive_service_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitiveServiceLocation"),
            ),
            custom_speech_model_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customSpeechModelId"),
            ),
            custom_voice_deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customVoiceDeploymentId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
