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
pub mod channel_facebook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelFacebookArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The Facebook Application ID for the Facebook Channel.
        #[builder(into)]
        pub facebook_application_id: pulumi_wasm_rust::Output<String>,
        /// The Facebook Application Secret for the Facebook Channel.
        #[builder(into)]
        pub facebook_application_secret: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `page` blocks as defined below.
        #[builder(into)]
        pub pages: pulumi_wasm_rust::Output<
            Vec<super::super::types::bot::ChannelFacebookPage>,
        >,
        /// The name of the resource group where the Facebook Channel should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelFacebookResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The Facebook Application ID for the Facebook Channel.
        pub facebook_application_id: pulumi_wasm_rust::Output<String>,
        /// The Facebook Application Secret for the Facebook Channel.
        pub facebook_application_secret: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// One or more `page` blocks as defined below.
        pub pages: pulumi_wasm_rust::Output<
            Vec<super::super::types::bot::ChannelFacebookPage>,
        >,
        /// The name of the resource group where the Facebook Channel should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ChannelFacebookArgs) -> ChannelFacebookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let facebook_application_id_binding = args.facebook_application_id.get_inner();
        let facebook_application_secret_binding = args
            .facebook_application_secret
            .get_inner();
        let location_binding = args.location.get_inner();
        let pages_binding = args.pages.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/channelFacebook:ChannelFacebook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "facebookApplicationId".into(),
                    value: &facebook_application_id_binding,
                },
                register_interface::ObjectField {
                    name: "facebookApplicationSecret".into(),
                    value: &facebook_application_secret_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "pages".into(),
                    value: &pages_binding,
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
                    name: "facebookApplicationId".into(),
                },
                register_interface::ResultField {
                    name: "facebookApplicationSecret".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "pages".into(),
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
        ChannelFacebookResult {
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            facebook_application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("facebookApplicationId").unwrap(),
            ),
            facebook_application_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("facebookApplicationSecret").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            pages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pages").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
