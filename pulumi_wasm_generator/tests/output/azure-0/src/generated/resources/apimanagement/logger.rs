/// Manages a Logger within an API Management Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("other")
///             .location("${example.location}")
///             .name("example-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLogger = logger::create(
///         "exampleLogger",
///         LoggerArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .application_insights(
///                 LoggerApplicationInsights::builder()
///                     .instrumentationKey("${exampleInsights.instrumentationKey}")
///                     .build_struct(),
///             )
///             .name("example-logger")
///             .resource_group_name("${example.name}")
///             .resource_id("${exampleInsights.id}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Loggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/logger:Logger example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.ApiManagement/service/example-apim/loggers/example-logger
/// ```
///
pub mod logger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggerArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// An `application_insights` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub application_insights: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::LoggerApplicationInsights>,
        >,
        /// Specifies whether records should be buffered in the Logger prior to publishing. Defaults to `true`.
        #[builder(into, default)]
        pub buffered: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A description of this Logger.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `eventhub` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eventhub: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::LoggerEventhub>,
        >,
        /// The name of this Logger, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The target resource id which will be linked in the API-Management portal page. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub resource_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoggerResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// An `application_insights` block as documented below. Changing this forces a new resource to be created.
        pub application_insights: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::LoggerApplicationInsights>,
        >,
        /// Specifies whether records should be buffered in the Logger prior to publishing. Defaults to `true`.
        pub buffered: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of this Logger.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An `eventhub` block as documented below. Changing this forces a new resource to be created.
        pub eventhub: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::LoggerEventhub>,
        >,
        /// The name of this Logger, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The target resource id which will be linked in the API-Management portal page. Changing this forces a new resource to be created.
        pub resource_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoggerArgs,
    ) -> LoggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let application_insights_binding = args
            .application_insights
            .get_output(context)
            .get_inner();
        let buffered_binding = args.buffered.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let eventhub_binding = args.eventhub.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/logger:Logger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "applicationInsights".into(),
                    value: &application_insights_binding,
                },
                register_interface::ObjectField {
                    name: "buffered".into(),
                    value: &buffered_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventhub".into(),
                    value: &eventhub_binding,
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
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "applicationInsights".into(),
                },
                register_interface::ResultField {
                    name: "buffered".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventhub".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoggerResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            application_insights: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationInsights").unwrap(),
            ),
            buffered: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buffered").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            eventhub: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhub").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}
