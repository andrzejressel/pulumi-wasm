/// Manages a Bot Web App.
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
///   exampleWebApp:
///     type: azure:bot:WebApp
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
/// Bot Web App's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/webApp:WebApp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.BotService/botServices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAppArgs {
        /// The Application Insights API Key to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_api_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Application ID to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_application_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_key: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Web App Bot will be displayed as. This defaults to `name` if not specified.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Web App Bot endpoint.
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of LUIS App IDs to associate with the Web App Bot.
        #[builder(into, default)]
        pub luis_app_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The LUIS key to associate with the Web App Bot.
        #[builder(into, default)]
        pub luis_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Microsoft Application ID for the Web App Bot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub microsoft_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Web App Bot. Changing this forces a new resource to be created. Must be globally unique.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Web App Bot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Web App Bot. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAppResult {
        /// The Application Insights API Key to associate with the Web App Bot.
        pub developer_app_insights_api_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Application Insights Application ID to associate with the Web App Bot.
        pub developer_app_insights_application_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Web App Bot.
        pub developer_app_insights_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Web App Bot will be displayed as. This defaults to `name` if not specified.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Web App Bot endpoint.
        pub endpoint: pulumi_gestalt_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A list of LUIS App IDs to associate with the Web App Bot.
        pub luis_app_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The LUIS key to associate with the Web App Bot.
        pub luis_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Microsoft Application ID for the Web App Bot. Changing this forces a new resource to be created.
        pub microsoft_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Web App Bot. Changing this forces a new resource to be created. Must be globally unique.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Web App Bot. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Web App Bot. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<String>,
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
        args: WebAppArgs,
    ) -> WebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
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
        let location_binding = args.location.get_output(context);
        let luis_app_ids_binding = args.luis_app_ids.get_output(context);
        let luis_key_binding = args.luis_key.get_output(context);
        let microsoft_app_id_binding = args.microsoft_app_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/webApp:WebApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsApiKey".into(),
                    value: developer_app_insights_api_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsApplicationId".into(),
                    value: developer_app_insights_application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerAppInsightsKey".into(),
                    value: developer_app_insights_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "luisAppIds".into(),
                    value: luis_app_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "luisKey".into(),
                    value: luis_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "microsoftAppId".into(),
                    value: microsoft_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAppResult {
            developer_app_insights_api_key: o.get_field("developerAppInsightsApiKey"),
            developer_app_insights_application_id: o
                .get_field("developerAppInsightsApplicationId"),
            developer_app_insights_key: o.get_field("developerAppInsightsKey"),
            display_name: o.get_field("displayName"),
            endpoint: o.get_field("endpoint"),
            location: o.get_field("location"),
            luis_app_ids: o.get_field("luisAppIds"),
            luis_key: o.get_field("luisKey"),
            microsoft_app_id: o.get_field("microsoftAppId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
