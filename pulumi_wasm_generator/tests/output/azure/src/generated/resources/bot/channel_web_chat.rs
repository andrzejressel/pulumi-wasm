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
pub mod channel_web_chat {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelWebChatArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group where the Web Chat Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A site represents a client application that you want to connect to your bot. One or more `site` blocks as defined below.
        #[builder(into, default)]
        pub sites: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::bot::ChannelWebChatSite>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ChannelWebChatResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the Web Chat Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A site represents a client application that you want to connect to your bot. One or more `site` blocks as defined below.
        pub sites: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::bot::ChannelWebChatSite>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ChannelWebChatArgs) -> ChannelWebChatResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let location_binding = args.location.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sites_binding = args.sites.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelWebChat:ChannelWebChat".into(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sites".into(),
                    value: &sites_binding,
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sites".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ChannelWebChatResult {
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sites: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sites").unwrap(),
            ),
        }
    }
}