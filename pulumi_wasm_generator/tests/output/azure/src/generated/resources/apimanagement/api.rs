/// Manages an API within an API Management Service.
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
///     let exampleApi = api::create(
///         "exampleApi",
///         ApiArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .display_name("Example API")
///             .import(
///                 ApiImport::builder()
///                     .contentFormat("swagger-link-json")
///                     .contentValue(
///                         "https://raw.githubusercontent.com/hashicorp/terraform-provider-azurerm/refs/heads/main/internal/services/apimanagement/testdata/api_management_api_schema_swagger.json",
///                     )
///                     .build_struct(),
///             )
///             .name("example-api")
///             .path("example")
///             .protocols(vec!["https",])
///             .resource_group_name("${example.name}")
///             .revision("1")
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
/// API Management API's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/api:Api example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/apis/api1;rev=1
/// ```
///
pub mod api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// The Name of the API Management Service where this API should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Type of API. Possible values are `graphql`, `http`, `soap`, and `websocket`. Defaults to `http`.
        #[builder(into, default)]
        pub api_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A `contact` block as documented below.
        #[builder(into, default)]
        pub contact: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiContact>,
        >,
        /// A description of the API Management API, which may include HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the API.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `import` block as documented below.
        #[builder(into, default)]
        pub import: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiImport>,
        >,
        /// A `license` block as documented below.
        #[builder(into, default)]
        pub license: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiLicense>,
        >,
        /// The name of the API Management API. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `oauth2_authorization` block as documented below.
        #[builder(into, default)]
        pub oauth2_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiOauth2Authorization>,
        >,
        /// An `openid_authentication` block as documented below.
        #[builder(into, default)]
        pub openid_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiOpenidAuthentication>,
        >,
        /// The Path for this API Management API, which is a relative URL which uniquely identifies this API and all of its resource paths within the API Management Service.
        #[builder(into, default)]
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of protocols the operations in this API can be invoked. Possible values are `http`, `https`, `ws`, and `wss`.
        ///
        /// > **NOTE:** `display_name`, `path` and `protocols` are required when `source_api_id` is not set.
        #[builder(into, default)]
        pub protocols: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Name of the Resource Group where the API Management API exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Revision which used for this API. Changing this forces a new resource to be created.
        #[builder(into)]
        pub revision: pulumi_wasm_rust::Output<String>,
        /// The description of the API Revision of the API Management API.
        #[builder(into, default)]
        pub revision_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Absolute URL of the backend service implementing this API.
        #[builder(into, default)]
        pub service_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The API id of the source API, which could be in format `azurerm_api_management_api.example.id` or in format `azurerm_api_management_api.example.id;rev=1`
        #[builder(into, default)]
        pub source_api_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `subscription_key_parameter_names` block as documented below.
        #[builder(into, default)]
        pub subscription_key_parameter_names: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiSubscriptionKeyParameterNames>,
        >,
        /// Should this API require a subscription key? Defaults to `true`.
        #[builder(into, default)]
        pub subscription_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Absolute URL of the Terms of Service for the API.
        #[builder(into, default)]
        pub terms_of_service_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The Version number of this API, if this API is versioned.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the API Version of the API Management API.
        #[builder(into, default)]
        pub version_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Version Set which this API is associated with.
        ///
        /// > **NOTE:** When `version` is set, `version_set_id` must also be specified
        #[builder(into, default)]
        pub version_set_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// The Name of the API Management Service where this API should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Type of API. Possible values are `graphql`, `http`, `soap`, and `websocket`. Defaults to `http`.
        pub api_type: pulumi_wasm_rust::Output<String>,
        /// A `contact` block as documented below.
        pub contact: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiContact>,
        >,
        /// A description of the API Management API, which may include HTML formatting tags.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the API.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// A `import` block as documented below.
        pub import: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiImport>,
        >,
        /// Is this the current API Revision?
        pub is_current: pulumi_wasm_rust::Output<bool>,
        /// Is this API Revision online/accessible via the Gateway?
        pub is_online: pulumi_wasm_rust::Output<bool>,
        /// A `license` block as documented below.
        pub license: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiLicense>,
        >,
        /// The name of the API Management API. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An `oauth2_authorization` block as documented below.
        pub oauth2_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiOauth2Authorization>,
        >,
        /// An `openid_authentication` block as documented below.
        pub openid_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::ApiOpenidAuthentication>,
        >,
        /// The Path for this API Management API, which is a relative URL which uniquely identifies this API and all of its resource paths within the API Management Service.
        pub path: pulumi_wasm_rust::Output<String>,
        /// A list of protocols the operations in this API can be invoked. Possible values are `http`, `https`, `ws`, and `wss`.
        ///
        /// > **NOTE:** `display_name`, `path` and `protocols` are required when `source_api_id` is not set.
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Name of the Resource Group where the API Management API exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Revision which used for this API. Changing this forces a new resource to be created.
        pub revision: pulumi_wasm_rust::Output<String>,
        /// The description of the API Revision of the API Management API.
        pub revision_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Absolute URL of the backend service implementing this API.
        pub service_url: pulumi_wasm_rust::Output<String>,
        /// The API id of the source API, which could be in format `azurerm_api_management_api.example.id` or in format `azurerm_api_management_api.example.id;rev=1`
        pub source_api_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `subscription_key_parameter_names` block as documented below.
        pub subscription_key_parameter_names: pulumi_wasm_rust::Output<
            super::super::types::apimanagement::ApiSubscriptionKeyParameterNames,
        >,
        /// Should this API require a subscription key? Defaults to `true`.
        pub subscription_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Absolute URL of the Terms of Service for the API.
        pub terms_of_service_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The Version number of this API, if this API is versioned.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The description of the API Version of the API Management API.
        pub version_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Version Set which this API is associated with.
        ///
        /// > **NOTE:** When `version` is set, `version_set_id` must also be specified
        pub version_set_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiArgs) -> ApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let api_type_binding = args.api_type.get_inner();
        let contact_binding = args.contact.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let import_binding = args.import.get_inner();
        let license_binding = args.license.get_inner();
        let name_binding = args.name.get_inner();
        let oauth2_authorization_binding = args.oauth2_authorization.get_inner();
        let openid_authentication_binding = args.openid_authentication.get_inner();
        let path_binding = args.path.get_inner();
        let protocols_binding = args.protocols.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let revision_binding = args.revision.get_inner();
        let revision_description_binding = args.revision_description.get_inner();
        let service_url_binding = args.service_url.get_inner();
        let source_api_id_binding = args.source_api_id.get_inner();
        let subscription_key_parameter_names_binding = args
            .subscription_key_parameter_names
            .get_inner();
        let subscription_required_binding = args.subscription_required.get_inner();
        let terms_of_service_url_binding = args.terms_of_service_url.get_inner();
        let version_binding = args.version.get_inner();
        let version_description_binding = args.version_description.get_inner();
        let version_set_id_binding = args.version_set_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/api:Api".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "apiType".into(),
                    value: &api_type_binding,
                },
                register_interface::ObjectField {
                    name: "contact".into(),
                    value: &contact_binding,
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
                    name: "import".into(),
                    value: &import_binding,
                },
                register_interface::ObjectField {
                    name: "license".into(),
                    value: &license_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "oauth2Authorization".into(),
                    value: &oauth2_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "openidAuthentication".into(),
                    value: &openid_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "revision".into(),
                    value: &revision_binding,
                },
                register_interface::ObjectField {
                    name: "revisionDescription".into(),
                    value: &revision_description_binding,
                },
                register_interface::ObjectField {
                    name: "serviceUrl".into(),
                    value: &service_url_binding,
                },
                register_interface::ObjectField {
                    name: "sourceApiId".into(),
                    value: &source_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionKeyParameterNames".into(),
                    value: &subscription_key_parameter_names_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionRequired".into(),
                    value: &subscription_required_binding,
                },
                register_interface::ObjectField {
                    name: "termsOfServiceUrl".into(),
                    value: &terms_of_service_url_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
                register_interface::ObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding,
                },
                register_interface::ObjectField {
                    name: "versionSetId".into(),
                    value: &version_set_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "apiType".into(),
                },
                register_interface::ResultField {
                    name: "contact".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "import".into(),
                },
                register_interface::ResultField {
                    name: "isCurrent".into(),
                },
                register_interface::ResultField {
                    name: "isOnline".into(),
                },
                register_interface::ResultField {
                    name: "license".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oauth2Authorization".into(),
                },
                register_interface::ResultField {
                    name: "openidAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "revisionDescription".into(),
                },
                register_interface::ResultField {
                    name: "serviceUrl".into(),
                },
                register_interface::ResultField {
                    name: "sourceApiId".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionKeyParameterNames".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionRequired".into(),
                },
                register_interface::ResultField {
                    name: "termsOfServiceUrl".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionDescription".into(),
                },
                register_interface::ResultField {
                    name: "versionSetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            api_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiType").unwrap(),
            ),
            contact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contact").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            import: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("import").unwrap(),
            ),
            is_current: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isCurrent").unwrap(),
            ),
            is_online: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isOnline").unwrap(),
            ),
            license: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("license").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oauth2_authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oauth2Authorization").unwrap(),
            ),
            openid_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openidAuthentication").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            revision_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionDescription").unwrap(),
            ),
            service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUrl").unwrap(),
            ),
            source_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceApiId").unwrap(),
            ),
            subscription_key_parameter_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionKeyParameterNames").unwrap(),
            ),
            subscription_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionRequired").unwrap(),
            ),
            terms_of_service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("termsOfServiceUrl").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionDescription").unwrap(),
            ),
            version_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionSetId").unwrap(),
            ),
        }
    }
}