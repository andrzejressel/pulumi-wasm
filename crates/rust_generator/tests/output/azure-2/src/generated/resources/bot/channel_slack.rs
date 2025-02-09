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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_slack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelSlackArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Client ID that will be used to authenticate with Slack.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Client Secret that will be used to authenticate with Slack.
        #[builder(into)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Slack Landing Page URL.
        #[builder(into, default)]
        pub landing_page_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Signing Secret that will be used to sign the requests.
        #[builder(into, default)]
        pub signing_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Verification Token that will be used to authenticate with Slack.
        #[builder(into)]
        pub verification_token: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelSlackResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// The Client ID that will be used to authenticate with Slack.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The Client Secret that will be used to authenticate with Slack.
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// The Slack Landing Page URL.
        pub landing_page_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Signing Secret that will be used to sign the requests.
        pub signing_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Verification Token that will be used to authenticate with Slack.
        pub verification_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelSlackArgs,
    ) -> ChannelSlackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let landing_page_url_binding = args.landing_page_url.get_output(context);
        let location_binding = args.location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let signing_secret_binding = args.signing_secret.get_output(context);
        let verification_token_binding = args.verification_token.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelSlack:ChannelSlack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: client_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "landingPageUrl".into(),
                    value: landing_page_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingSecret".into(),
                    value: signing_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verificationToken".into(),
                    value: verification_token_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelSlackResult {
            bot_name: o.get_field("botName"),
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            landing_page_url: o.get_field("landingPageUrl"),
            location: o.get_field("location"),
            resource_group_name: o.get_field("resourceGroupName"),
            signing_secret: o.get_field("signingSecret"),
            verification_token: o.get_field("verificationToken"),
        }
    }
}
