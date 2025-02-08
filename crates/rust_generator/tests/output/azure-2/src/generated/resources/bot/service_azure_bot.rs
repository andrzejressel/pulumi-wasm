/// Manages an Azure Bot Service.
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
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: example-appinsights
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleApiKey:
///     type: azure:appinsights:ApiKey
///     name: example
///     properties:
///       name: example-appinsightsapikey
///       applicationInsightsId: ${exampleInsights.id}
///       readPermissions:
///         - aggregate
///         - api
///         - draft
///         - extendqueries
///         - search
///   exampleServiceAzureBot:
///     type: azure:bot:ServiceAzureBot
///     name: example
///     properties:
///       name: exampleazurebot
///       resourceGroupName: ${example.name}
///       location: global
///       microsoftAppId: ${current.clientId}
///       sku: F0
///       endpoint: https://example.com
///       developerAppInsightsApiKey: ${exampleApiKey.apiKey}
///       developerAppInsightsApplicationId: ${exampleInsights.appId}
///       tags:
///         environment: test
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Azure Bot Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/serviceAzureBot:ServiceAzureBot example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.BotService/botServices/botService1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_azure_bot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceAzureBotArgs {
        /// The CMK Key Vault Key URL that will be used to encrypt the Bot with the Customer Managed Encryption Key.
        ///
        /// > **Note:** In order to utilize CMEK, you must add the `Key Vault Crypto Service Encryption User` role to the Azure-defined `Bot Service CMEK Prod` Service Principal. You must also enable `soft_delete_enabled` and `purge_protection_enabled` on the `azure.keyvault.KeyVault` that `cmk_key_vault_key_url` refers to. [See Azure Documentation](https://learn.microsoft.com/en-us/azure/bot-service/bot-service-encryption?view=azure-bot-service-4.0#how-to-configure-your-azure-key-vault-instance)
        #[builder(into, default)]
        pub cmk_key_vault_key_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Application Insights API Key to associate with this Azure Bot Service.
        #[builder(into, default)]
        pub developer_app_insights_api_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The resource ID of the Application Insights instance to associate with this Azure Bot Service.
        #[builder(into, default)]
        pub developer_app_insights_application_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insight Key to associate with this Azure Bot Service.
        #[builder(into, default)]
        pub developer_app_insights_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name that the Azure Bot Service will be displayed as. This defaults to the value set for `name` if not specified.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Bot Service endpoint.
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Icon Url of the Azure Bot Service. Defaults to `https://docs.botframework.com/static/devportal/client/images/bot-framework-default.png`.
        #[builder(into, default)]
        pub icon_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is local authentication enabled? Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The supported Azure location where the Azure Bot Service should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of LUIS App IDs to associate with this Azure Bot Service.
        #[builder(into, default)]
        pub luis_app_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The LUIS key to associate with this Azure Bot Service.
        #[builder(into, default)]
        pub luis_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Microsoft Application ID for the Azure Bot Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub microsoft_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Microsoft App Managed Identity for this Azure Bot Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub microsoft_app_msi_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Tenant ID of the Microsoft App for this Azure Bot Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub microsoft_app_tenant_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Microsoft App Type for this Azure Bot Service. Possible values are `MultiTenant`, `SingleTenant` and `UserAssignedMSI`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub microsoft_app_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Azure Bot Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the Resource Group where the Azure Bot Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Azure Bot Service. Accepted values are `F0` or `S1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the streaming endpoint enabled for this Azure Bot Service. Defaults to `false`.
        #[builder(into, default)]
        pub streaming_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags which should be assigned to this Azure Bot Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceAzureBotResult {
        /// The CMK Key Vault Key URL that will be used to encrypt the Bot with the Customer Managed Encryption Key.
        ///
        /// > **Note:** In order to utilize CMEK, you must add the `Key Vault Crypto Service Encryption User` role to the Azure-defined `Bot Service CMEK Prod` Service Principal. You must also enable `soft_delete_enabled` and `purge_protection_enabled` on the `azure.keyvault.KeyVault` that `cmk_key_vault_key_url` refers to. [See Azure Documentation](https://learn.microsoft.com/en-us/azure/bot-service/bot-service-encryption?view=azure-bot-service-4.0#how-to-configure-your-azure-key-vault-instance)
        pub cmk_key_vault_key_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Application Insights API Key to associate with this Azure Bot Service.
        pub developer_app_insights_api_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the Application Insights instance to associate with this Azure Bot Service.
        pub developer_app_insights_application_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The Application Insight Key to associate with this Azure Bot Service.
        pub developer_app_insights_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name that the Azure Bot Service will be displayed as. This defaults to the value set for `name` if not specified.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Bot Service endpoint.
        pub endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Icon Url of the Azure Bot Service. Defaults to `https://docs.botframework.com/static/devportal/client/images/bot-framework-default.png`.
        pub icon_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is local authentication enabled? Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The supported Azure location where the Azure Bot Service should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A list of LUIS App IDs to associate with this Azure Bot Service.
        pub luis_app_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The LUIS key to associate with this Azure Bot Service.
        pub luis_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Microsoft Application ID for the Azure Bot Service. Changing this forces a new resource to be created.
        pub microsoft_app_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Microsoft App Managed Identity for this Azure Bot Service. Changing this forces a new resource to be created.
        pub microsoft_app_msi_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Tenant ID of the Microsoft App for this Azure Bot Service. Changing this forces a new resource to be created.
        pub microsoft_app_tenant_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Microsoft App Type for this Azure Bot Service. Possible values are `MultiTenant`, `SingleTenant` and `UserAssignedMSI`. Changing this forces a new resource to be created.
        pub microsoft_app_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Bot Service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is enabled. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Azure Bot Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Azure Bot Service. Accepted values are `F0` or `S1`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Is the streaming endpoint enabled for this Azure Bot Service. Defaults to `false`.
        pub streaming_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags which should be assigned to this Azure Bot Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceAzureBotArgs,
    ) -> ServiceAzureBotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cmk_key_vault_key_url_binding = args
            .cmk_key_vault_key_url
            .get_output(context)
            .get_inner();
        let developer_app_insights_api_key_binding = args
            .developer_app_insights_api_key
            .get_output(context)
            .get_inner();
        let developer_app_insights_application_id_binding = args
            .developer_app_insights_application_id
            .get_output(context)
            .get_inner();
        let developer_app_insights_key_binding = args
            .developer_app_insights_key
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let endpoint_binding = args.endpoint.get_output(context).get_inner();
        let icon_url_binding = args.icon_url.get_output(context).get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let luis_app_ids_binding = args.luis_app_ids.get_output(context).get_inner();
        let luis_key_binding = args.luis_key.get_output(context).get_inner();
        let microsoft_app_id_binding = args
            .microsoft_app_id
            .get_output(context)
            .get_inner();
        let microsoft_app_msi_id_binding = args
            .microsoft_app_msi_id
            .get_output(context)
            .get_inner();
        let microsoft_app_tenant_id_binding = args
            .microsoft_app_tenant_id
            .get_output(context)
            .get_inner();
        let microsoft_app_type_binding = args
            .microsoft_app_type
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let streaming_endpoint_enabled_binding = args
            .streaming_endpoint_enabled
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/serviceAzureBot:ServiceAzureBot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cmkKeyVaultKeyUrl".into(),
                    value: &cmk_key_vault_key_url_binding,
                },
                register_interface::ObjectField {
                    name: "developerAppInsightsApiKey".into(),
                    value: &developer_app_insights_api_key_binding,
                },
                register_interface::ObjectField {
                    name: "developerAppInsightsApplicationId".into(),
                    value: &developer_app_insights_application_id_binding,
                },
                register_interface::ObjectField {
                    name: "developerAppInsightsKey".into(),
                    value: &developer_app_insights_key_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "iconUrl".into(),
                    value: &icon_url_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "luisAppIds".into(),
                    value: &luis_app_ids_binding,
                },
                register_interface::ObjectField {
                    name: "luisKey".into(),
                    value: &luis_key_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftAppId".into(),
                    value: &microsoft_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftAppMsiId".into(),
                    value: &microsoft_app_msi_id_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftAppTenantId".into(),
                    value: &microsoft_app_tenant_id_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftAppType".into(),
                    value: &microsoft_app_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "streamingEndpointEnabled".into(),
                    value: &streaming_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceAzureBotResult {
            cmk_key_vault_key_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cmkKeyVaultKeyUrl"),
            ),
            developer_app_insights_api_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("developerAppInsightsApiKey"),
            ),
            developer_app_insights_application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("developerAppInsightsApplicationId"),
            ),
            developer_app_insights_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("developerAppInsightsKey"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            icon_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iconUrl"),
            ),
            local_authentication_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAuthenticationEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            luis_app_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("luisAppIds"),
            ),
            luis_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("luisKey"),
            ),
            microsoft_app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("microsoftAppId"),
            ),
            microsoft_app_msi_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("microsoftAppMsiId"),
            ),
            microsoft_app_tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("microsoftAppTenantId"),
            ),
            microsoft_app_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("microsoftAppType"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            streaming_endpoint_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamingEndpointEnabled"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
