/// Manages a Bot Channels Registration.
///
/// > **Note:** Bot Channels Registration has been [deprecated by Azure](https://learn.microsoft.com/en-us/azure/bot-service/bot-service-resources-faq-azure?view=azure-bot-service-4.0#why-are-web-app-bot-and-bot-channel-registration-being-deprecated). New implementations should use the `azure.bot.ServiceAzureBot` resource.
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
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Bot Channels Registration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelsRegistration:ChannelsRegistration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.BotService/botServices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channels_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelsRegistrationArgs {
        /// The CMK Key Vault Key URL to encrypt the Bot Channels Registration with the Customer Managed Encryption Key.
        ///
        /// > **Note:** It has to add the Key Vault Access Policy for the `Bot Service CMEK Prod` Service Principal and the `soft_delete_enabled` and the `purge_protection_enabled` is enabled on the `azure.keyvault.KeyVault` resource while using `cmk_key_vault_url`.
        ///
        /// > **Note:** It has to turn off the CMK feature before revoking Key Vault Access Policy. For more information, please refer to [Revoke access to customer-managed keys](https://docs.microsoft.com/azure/bot-service/bot-service-encryption?view=azure-bot-service-4.0&WT.mc_id=Portal-Microsoft_Azure_BotService#revoke-access-to-customer-managed-keys).
        #[builder(into, default)]
        pub cmk_key_vault_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the Bot Channels Registration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Application Insights API Key to associate with the Bot Channels Registration.
        #[builder(into, default)]
        pub developer_app_insights_api_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Application ID to associate with the Bot Channels Registration.
        #[builder(into, default)]
        pub developer_app_insights_application_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Bot Channels Registration.
        #[builder(into, default)]
        pub developer_app_insights_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Bot Channels Registration will be displayed as. This defaults to `name` if not specified.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Bot Channels Registration endpoint.
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The icon URL to visually identify the Bot Channels Registration. Defaults to `https://docs.botframework.com/static/devportal/client/images/bot-framework-default.png`.
        #[builder(into, default)]
        pub icon_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Microsoft Application ID for the Bot Channels Registration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub microsoft_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Bot Channels Registration. Changing this forces a new resource to be created. Must be globally unique.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the Bot Channels Registration in an isolated network?
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Bot Channels Registration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Bot Channels Registration. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the streaming endpoint enabled for the Bot Channels Registration. Defaults to `false`.
        #[builder(into, default)]
        pub streaming_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ChannelsRegistrationResult {
        /// The CMK Key Vault Key URL to encrypt the Bot Channels Registration with the Customer Managed Encryption Key.
        ///
        /// > **Note:** It has to add the Key Vault Access Policy for the `Bot Service CMEK Prod` Service Principal and the `soft_delete_enabled` and the `purge_protection_enabled` is enabled on the `azure.keyvault.KeyVault` resource while using `cmk_key_vault_url`.
        ///
        /// > **Note:** It has to turn off the CMK feature before revoking Key Vault Access Policy. For more information, please refer to [Revoke access to customer-managed keys](https://docs.microsoft.com/azure/bot-service/bot-service-encryption?view=azure-bot-service-4.0&WT.mc_id=Portal-Microsoft_Azure_BotService#revoke-access-to-customer-managed-keys).
        pub cmk_key_vault_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the Bot Channels Registration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Application Insights API Key to associate with the Bot Channels Registration.
        pub developer_app_insights_api_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Application Insights Application ID to associate with the Bot Channels Registration.
        pub developer_app_insights_application_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Bot Channels Registration.
        pub developer_app_insights_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Bot Channels Registration will be displayed as. This defaults to `name` if not specified.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Bot Channels Registration endpoint.
        pub endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The icon URL to visually identify the Bot Channels Registration. Defaults to `https://docs.botframework.com/static/devportal/client/images/bot-framework-default.png`.
        pub icon_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Microsoft Application ID for the Bot Channels Registration. Changing this forces a new resource to be created.
        pub microsoft_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Bot Channels Registration. Changing this forces a new resource to be created. Must be globally unique.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is the Bot Channels Registration in an isolated network?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Bot Channels Registration. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Bot Channels Registration. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Is the streaming endpoint enabled for the Bot Channels Registration. Defaults to `false`.
        pub streaming_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelsRegistrationArgs,
    ) -> ChannelsRegistrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cmk_key_vault_url_binding = args.cmk_key_vault_url.get_output(context);
        let description_binding = args.description.get_output(context);
        let developer_app_insights_api_key_binding = args
            .developer_app_insights_api_key
            .get_output(context);
        let developer_app_insights_application_id_binding = args
            .developer_app_insights_application_id
            .get_output(context);
        let developer_app_insights_key_binding = args
            .developer_app_insights_key
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let endpoint_binding = args.endpoint.get_output(context);
        let icon_url_binding = args.icon_url.get_output(context);
        let location_binding = args.location.get_output(context);
        let microsoft_app_id_binding = args.microsoft_app_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let streaming_endpoint_enabled_binding = args
            .streaming_endpoint_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelsRegistration:ChannelsRegistration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cmkKeyVaultUrl".into(),
                    value: &cmk_key_vault_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsApiKey".into(),
                    value: &developer_app_insights_api_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsApplicationId".into(),
                    value: &developer_app_insights_application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsKey".into(),
                    value: &developer_app_insights_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iconUrl".into(),
                    value: &icon_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "microsoftAppId".into(),
                    value: &microsoft_app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingEndpointEnabled".into(),
                    value: &streaming_endpoint_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelsRegistrationResult {
            cmk_key_vault_url: o.get_field("cmkKeyVaultUrl"),
            description: o.get_field("description"),
            developer_app_insights_api_key: o.get_field("developerAppInsightsApiKey"),
            developer_app_insights_application_id: o
                .get_field("developerAppInsightsApplicationId"),
            developer_app_insights_key: o.get_field("developerAppInsightsKey"),
            display_name: o.get_field("displayName"),
            endpoint: o.get_field("endpoint"),
            icon_url: o.get_field("iconUrl"),
            location: o.get_field("location"),
            microsoft_app_id: o.get_field("microsoftAppId"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            streaming_endpoint_enabled: o.get_field("streamingEndpointEnabled"),
            tags: o.get_field("tags"),
        }
    }
}
