/// Manages a Facebook integration for a Bot Channel
///
/// > **Note** A bot can only have a single Facebook Channel associated with it.
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
///   exampleChannelFacebook:
///     type: azure:bot:ChannelFacebook
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       facebookApplicationId: '563490254873576'
///       facebookApplicationSecret: 8976d2536445ad5b976dee8437b9beb0
///       pages:
///         - id: '876248795081953'
///           accessToken: CGGCec3UAFPMBAKwK3Ft8SEpO8ZCuvpNBI5DClaJCDfqJj2BgEHCKxcY0FDarmUQap6XxpZC9GWCW4nZCzjcKosAZAP7SO44X8Q8gAntbDIXgYUBGp9xtS8wUkwgKPobUePcOOVFkvClxvYZByuiQxoTiK9fQ9jZCPEorbmZCsKDZAx4VLnrNwCTZAPUwXxO61gfq4ZD
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The Facebook Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelFacebook:ChannelFacebook example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.BotService/botServices/botService1/channels/FacebookChannel
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_facebook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelFacebookArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Facebook Application ID for the Facebook Channel.
        #[builder(into)]
        pub facebook_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Facebook Application Secret for the Facebook Channel.
        #[builder(into)]
        pub facebook_application_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `page` blocks as defined below.
        #[builder(into)]
        pub pages: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::bot::ChannelFacebookPage>,
        >,
        /// The name of the resource group where the Facebook Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelFacebookResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// The Facebook Application ID for the Facebook Channel.
        pub facebook_application_id: pulumi_gestalt_rust::Output<String>,
        /// The Facebook Application Secret for the Facebook Channel.
        pub facebook_application_secret: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// One or more `page` blocks as defined below.
        pub pages: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bot::ChannelFacebookPage>,
        >,
        /// The name of the resource group where the Facebook Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelFacebookArgs,
    ) -> ChannelFacebookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let facebook_application_id_binding = args
            .facebook_application_id
            .get_output(context);
        let facebook_application_secret_binding = args
            .facebook_application_secret
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let pages_binding = args.pages.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelFacebook:ChannelFacebook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "facebookApplicationId".into(),
                    value: facebook_application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "facebookApplicationSecret".into(),
                    value: facebook_application_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pages".into(),
                    value: pages_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelFacebookResult {
            bot_name: o.get_field("botName"),
            facebook_application_id: o.get_field("facebookApplicationId"),
            facebook_application_secret: o.get_field("facebookApplicationSecret"),
            location: o.get_field("location"),
            pages: o.get_field("pages"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
