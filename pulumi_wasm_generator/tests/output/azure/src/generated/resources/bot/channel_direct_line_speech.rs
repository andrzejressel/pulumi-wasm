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
pub mod channel_direct_line_speech {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelDirectLineSpeechArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Cognitive Account this Bot Channel should be associated with.
        #[builder(into, default)]
        pub cognitive_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The access key to access the Cognitive Service.
        #[builder(into)]
        pub cognitive_service_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the Cognitive Service resource exists.
        #[builder(into)]
        pub cognitive_service_location: pulumi_wasm_rust::Output<String>,
        /// The custom speech model id for the Direct Line Speech Channel.
        #[builder(into, default)]
        pub custom_speech_model_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The custom voice deployment id for the Direct Line Speech Channel.
        #[builder(into, default)]
        pub custom_voice_deployment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group where the Direct Line Speech Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelDirectLineSpeechResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Cognitive Account this Bot Channel should be associated with.
        pub cognitive_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The access key to access the Cognitive Service.
        pub cognitive_service_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the Cognitive Service resource exists.
        pub cognitive_service_location: pulumi_wasm_rust::Output<String>,
        /// The custom speech model id for the Direct Line Speech Channel.
        pub custom_speech_model_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The custom voice deployment id for the Direct Line Speech Channel.
        pub custom_voice_deployment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the Direct Line Speech Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ChannelDirectLineSpeechArgs,
    ) -> ChannelDirectLineSpeechResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let cognitive_account_id_binding = args.cognitive_account_id.get_inner();
        let cognitive_service_access_key_binding = args
            .cognitive_service_access_key
            .get_inner();
        let cognitive_service_location_binding = args
            .cognitive_service_location
            .get_inner();
        let custom_speech_model_id_binding = args.custom_speech_model_id.get_inner();
        let custom_voice_deployment_id_binding = args
            .custom_voice_deployment_id
            .get_inner();
        let location_binding = args.location.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelDirectLineSpeech:ChannelDirectLineSpeech".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "botName".into(),
                },
                register_interface::ResultField {
                    name: "cognitiveAccountId".into(),
                },
                register_interface::ResultField {
                    name: "cognitiveServiceAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "cognitiveServiceLocation".into(),
                },
                register_interface::ResultField {
                    name: "customSpeechModelId".into(),
                },
                register_interface::ResultField {
                    name: "customVoiceDeploymentId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ChannelDirectLineSpeechResult {
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            cognitive_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitiveAccountId").unwrap(),
            ),
            cognitive_service_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitiveServiceAccessKey").unwrap(),
            ),
            cognitive_service_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitiveServiceLocation").unwrap(),
            ),
            custom_speech_model_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSpeechModelId").unwrap(),
            ),
            custom_voice_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customVoiceDeploymentId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
