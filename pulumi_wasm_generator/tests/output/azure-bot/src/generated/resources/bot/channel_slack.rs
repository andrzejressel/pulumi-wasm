/// Manages a Slack integration for a Bot Channel
///
/// > **Note** A bot can only have a single Slack Channel associated with it.
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
///       name: example
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelSlack:
///     type: azure:bot:ChannelSlack
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       clientId: exampleId
///       clientSecret: exampleSecret
///       verificationToken: exampleVerificationToken
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The Slack Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelSlack:ChannelSlack example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.BotService/botServices/example/channels/SlackChannel
/// ```
///
pub mod channel_slack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelSlackArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The Client ID that will be used to authenticate with Slack.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The Client Secret that will be used to authenticate with Slack.
        #[builder(into)]
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// The Slack Landing Page URL.
        #[builder(into, default)]
        pub landing_page_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Signing Secret that will be used to sign the requests.
        #[builder(into, default)]
        pub signing_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The Verification Token that will be used to authenticate with Slack.
        #[builder(into)]
        pub verification_token: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelSlackResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The Client ID that will be used to authenticate with Slack.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The Client Secret that will be used to authenticate with Slack.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// The Slack Landing Page URL.
        pub landing_page_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Signing Secret that will be used to sign the requests.
        pub signing_secret: pulumi_wasm_rust::Output<Option<String>>,
        /// The Verification Token that will be used to authenticate with Slack.
        pub verification_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ChannelSlackArgs) -> ChannelSlackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let client_id_binding = args.client_id.get_inner();
        let client_secret_binding = args.client_secret.get_inner();
        let landing_page_url_binding = args.landing_page_url.get_inner();
        let location_binding = args.location.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let signing_secret_binding = args.signing_secret.get_inner();
        let verification_token_binding = args.verification_token.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelSlack:ChannelSlack".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "landingPageUrl".into(),
                    value: &landing_page_url_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "signingSecret".into(),
                    value: &signing_secret_binding,
                },
                register_interface::ObjectField {
                    name: "verificationToken".into(),
                    value: &verification_token_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "botName".into(),
                },
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "landingPageUrl".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "signingSecret".into(),
                },
                register_interface::ResultField {
                    name: "verificationToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ChannelSlackResult {
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            landing_page_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("landingPageUrl").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            signing_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingSecret").unwrap(),
            ),
            verification_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verificationToken").unwrap(),
            ),
        }
    }
}