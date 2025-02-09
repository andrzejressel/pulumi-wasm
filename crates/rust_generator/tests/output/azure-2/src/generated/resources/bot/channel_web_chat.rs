/// Manages a Web Chat integration for a Bot Channel
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
///   exampleChannelWebChat:
///     type: azure:bot:ChannelWebChat
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
///       sites:
///         - name: TestSite
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Web Chat Channels can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelWebChat:ChannelWebChat example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.BotService/botServices/botService1/channels/WebChatChannel
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_web_chat {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelWebChatArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the Web Chat Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A site represents a client application that you want to connect to your bot. One or more `site` blocks as defined below.
        #[builder(into, default)]
        pub sites: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bot::ChannelWebChatSite>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ChannelWebChatResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the Web Chat Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A site represents a client application that you want to connect to your bot. One or more `site` blocks as defined below.
        pub sites: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::bot::ChannelWebChatSite>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelWebChatArgs,
    ) -> ChannelWebChatResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sites_binding = args.sites.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelWebChat:ChannelWebChat".into(),
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
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sites".into(),
                    value: sites_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelWebChatResult {
            bot_name: o.get_field("botName"),
            location: o.get_field("location"),
            resource_group_name: o.get_field("resourceGroupName"),
            sites: o.get_field("sites"),
        }
    }
}
