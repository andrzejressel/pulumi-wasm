/// Manages an OpenID Connect Provider within a API Management Service.
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
///     let exampleOpenIdConnectProvider = open_id_connect_provider::create(
///         "exampleOpenIdConnectProvider",
///         OpenIdConnectProviderArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .client_id("00001111-2222-3333-4444-555566667777")
///             .client_secret("00001111-423egvwdcsjx-00001111")
///             .display_name("Example Provider")
///             .metadata_endpoint("https://example.com/example")
///             .name("example-provider")
///             .resource_group_name("${example.name}")
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
/// API Management OpenID Connect Providers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/openIdConnectProvider:OpenIdConnectProvider example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/openidConnectProviders/provider1
/// ```
///
pub mod open_id_connect_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderArgs {
        /// The name of the API Management Service in which this OpenID Connect Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Client ID used for the Client Application.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Client Secret used for the Client Application.
        #[builder(into)]
        pub client_secret: pulumi_wasm_rust::InputOrOutput<String>,
        /// A description of this OpenID Connect Provider.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A user-friendly name for this OpenID Connect Provider.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The URI of the Metadata endpoint.
        #[builder(into)]
        pub metadata_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// the Name of the OpenID Connect Provider which should be created within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderResult {
        /// The name of the API Management Service in which this OpenID Connect Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The Client ID used for the Client Application.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The Client Secret used for the Client Application.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// A description of this OpenID Connect Provider.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A user-friendly name for this OpenID Connect Provider.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The URI of the Metadata endpoint.
        pub metadata_endpoint: pulumi_wasm_rust::Output<String>,
        /// the Name of the OpenID Connect Provider which should be created within the API Management Service. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OpenIdConnectProviderArgs,
    ) -> OpenIdConnectProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let client_secret_binding = args.client_secret.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let metadata_endpoint_binding = args
            .metadata_endpoint
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/openIdConnectProvider:OpenIdConnectProvider"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadataEndpoint".into(),
                    value: &metadata_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "metadataEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OpenIdConnectProviderResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            metadata_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataEndpoint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
