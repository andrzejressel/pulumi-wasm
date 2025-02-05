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
pub mod web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAppArgs {
        /// The Application Insights API Key to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_api_key: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Application ID to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_application_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Web App Bot.
        #[builder(into, default)]
        pub developer_app_insights_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Web App Bot will be displayed as. This defaults to `name` if not specified.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Web App Bot endpoint.
        #[builder(into, default)]
        pub endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of LUIS App IDs to associate with the Web App Bot.
        #[builder(into, default)]
        pub luis_app_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The LUIS key to associate with the Web App Bot.
        #[builder(into, default)]
        pub luis_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Microsoft Application ID for the Web App Bot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub microsoft_app_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Web App Bot. Changing this forces a new resource to be created. Must be globally unique.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Web App Bot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SKU of the Web App Bot. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAppResult {
        /// The Application Insights API Key to associate with the Web App Bot.
        pub developer_app_insights_api_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The Application Insights Application ID to associate with the Web App Bot.
        pub developer_app_insights_application_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The Application Insights Key to associate with the Web App Bot.
        pub developer_app_insights_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Web App Bot will be displayed as. This defaults to `name` if not specified.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Web App Bot endpoint.
        pub endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A list of LUIS App IDs to associate with the Web App Bot.
        pub luis_app_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The LUIS key to associate with the Web App Bot.
        pub luis_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The Microsoft Application ID for the Web App Bot. Changing this forces a new resource to be created.
        pub microsoft_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Web App Bot. Changing this forces a new resource to be created. Must be globally unique.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Web App Bot. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Web App Bot. Valid values include `F0` or `S1`. Changing this forces a new resource to be created.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebAppArgs,
    ) -> WebAppResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
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
        let location_binding = args.location.get_output(context).get_inner();
        let luis_app_ids_binding = args.luis_app_ids.get_output(context).get_inner();
        let luis_key_binding = args.luis_key.get_output(context).get_inner();
        let microsoft_app_id_binding = args
            .microsoft_app_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:bot/webApp:WebApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebAppResult {
            developer_app_insights_api_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("developerAppInsightsApiKey"),
            ),
            developer_app_insights_application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("developerAppInsightsApplicationId"),
            ),
            developer_app_insights_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("developerAppInsightsKey"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            luis_app_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("luisAppIds"),
            ),
            luis_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("luisKey"),
            ),
            microsoft_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("microsoftAppId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
