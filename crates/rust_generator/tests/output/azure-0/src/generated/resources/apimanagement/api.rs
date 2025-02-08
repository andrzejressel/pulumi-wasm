/// Manages an API within an API Management Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation)]
pub mod api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// The Name of the API Management Service where this API should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of API. Possible values are `graphql`, `http`, `soap`, and `websocket`. Defaults to `http`.
        #[builder(into, default)]
        pub api_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `contact` block as documented below.
        #[builder(into, default)]
        pub contact: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiContact>,
        >,
        /// A description of the API Management API, which may include HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the API.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `import` block as documented below.
        #[builder(into, default)]
        pub import: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiImport>,
        >,
        /// A `license` block as documented below.
        #[builder(into, default)]
        pub license: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiLicense>,
        >,
        /// The name of the API Management API. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `oauth2_authorization` block as documented below.
        #[builder(into, default)]
        pub oauth2_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiOauth2Authorization>,
        >,
        /// An `openid_authentication` block as documented below.
        #[builder(into, default)]
        pub openid_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiOpenidAuthentication>,
        >,
        /// The Path for this API Management API, which is a relative URL which uniquely identifies this API and all of its resource paths within the API Management Service.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of protocols the operations in this API can be invoked. Possible values are `http`, `https`, `ws`, and `wss`.
        ///
        /// > **NOTE:** `display_name`, `path` and `protocols` are required when `source_api_id` is not set.
        #[builder(into, default)]
        pub protocols: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Name of the Resource Group where the API Management API exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Revision which used for this API. Changing this forces a new resource to be created.
        #[builder(into)]
        pub revision: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the API Revision of the API Management API.
        #[builder(into, default)]
        pub revision_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Absolute URL of the backend service implementing this API.
        #[builder(into, default)]
        pub service_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The API id of the source API, which could be in format `azurerm_api_management_api.example.id` or in format `azurerm_api_management_api.example.id;rev=1`
        #[builder(into, default)]
        pub source_api_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `subscription_key_parameter_names` block as documented below.
        #[builder(into, default)]
        pub subscription_key_parameter_names: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiSubscriptionKeyParameterNames>,
        >,
        /// Should this API require a subscription key? Defaults to `true`.
        #[builder(into, default)]
        pub subscription_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Absolute URL of the Terms of Service for the API.
        #[builder(into, default)]
        pub terms_of_service_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Version number of this API, if this API is versioned.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the API Version of the API Management API.
        #[builder(into, default)]
        pub version_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Version Set which this API is associated with.
        ///
        /// > **NOTE:** When `version` is set, `version_set_id` must also be specified
        #[builder(into, default)]
        pub version_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// The Name of the API Management Service where this API should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// Type of API. Possible values are `graphql`, `http`, `soap`, and `websocket`. Defaults to `http`.
        pub api_type: pulumi_gestalt_rust::Output<String>,
        /// A `contact` block as documented below.
        pub contact: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ApiContact>,
        >,
        /// A description of the API Management API, which may include HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the API.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// A `import` block as documented below.
        pub import: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ApiImport>,
        >,
        /// Is this the current API Revision?
        pub is_current: pulumi_gestalt_rust::Output<bool>,
        /// Is this API Revision online/accessible via the Gateway?
        pub is_online: pulumi_gestalt_rust::Output<bool>,
        /// A `license` block as documented below.
        pub license: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ApiLicense>,
        >,
        /// The name of the API Management API. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `oauth2_authorization` block as documented below.
        pub oauth2_authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ApiOauth2Authorization>,
        >,
        /// An `openid_authentication` block as documented below.
        pub openid_authentication: pulumi_gestalt_rust::Output<
            Option<super::super::types::apimanagement::ApiOpenidAuthentication>,
        >,
        /// The Path for this API Management API, which is a relative URL which uniquely identifies this API and all of its resource paths within the API Management Service.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// A list of protocols the operations in this API can be invoked. Possible values are `http`, `https`, `ws`, and `wss`.
        ///
        /// > **NOTE:** `display_name`, `path` and `protocols` are required when `source_api_id` is not set.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Name of the Resource Group where the API Management API exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Revision which used for this API. Changing this forces a new resource to be created.
        pub revision: pulumi_gestalt_rust::Output<String>,
        /// The description of the API Revision of the API Management API.
        pub revision_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Absolute URL of the backend service implementing this API.
        pub service_url: pulumi_gestalt_rust::Output<String>,
        /// The API id of the source API, which could be in format `azurerm_api_management_api.example.id` or in format `azurerm_api_management_api.example.id;rev=1`
        pub source_api_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `subscription_key_parameter_names` block as documented below.
        pub subscription_key_parameter_names: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ApiSubscriptionKeyParameterNames,
        >,
        /// Should this API require a subscription key? Defaults to `true`.
        pub subscription_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Absolute URL of the Terms of Service for the API.
        pub terms_of_service_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Version number of this API, if this API is versioned.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The description of the API Version of the API Management API.
        pub version_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Version Set which this API is associated with.
        ///
        /// > **NOTE:** When `version` is set, `version_set_id` must also be specified
        pub version_set_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiArgs,
    ) -> ApiResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let api_type_binding = args.api_type.get_output(context).get_inner();
        let contact_binding = args.contact.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let import_binding = args.import.get_output(context).get_inner();
        let license_binding = args.license.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let oauth2_authorization_binding = args
            .oauth2_authorization
            .get_output(context)
            .get_inner();
        let openid_authentication_binding = args
            .openid_authentication
            .get_output(context)
            .get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let protocols_binding = args.protocols.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let revision_binding = args.revision.get_output(context).get_inner();
        let revision_description_binding = args
            .revision_description
            .get_output(context)
            .get_inner();
        let service_url_binding = args.service_url.get_output(context).get_inner();
        let source_api_id_binding = args.source_api_id.get_output(context).get_inner();
        let subscription_key_parameter_names_binding = args
            .subscription_key_parameter_names
            .get_output(context)
            .get_inner();
        let subscription_required_binding = args
            .subscription_required
            .get_output(context)
            .get_inner();
        let terms_of_service_url_binding = args
            .terms_of_service_url
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let version_description_binding = args
            .version_description
            .get_output(context)
            .get_inner();
        let version_set_id_binding = args.version_set_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/api:Api".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiResult {
            api_management_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            api_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiType"),
            ),
            contact: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contact"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            import: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("import"),
            ),
            is_current: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isCurrent"),
            ),
            is_online: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isOnline"),
            ),
            license: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("license"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            oauth2_authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oauth2Authorization"),
            ),
            openid_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("openidAuthentication"),
            ),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            revision: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revision"),
            ),
            revision_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revisionDescription"),
            ),
            service_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceUrl"),
            ),
            source_api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceApiId"),
            ),
            subscription_key_parameter_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionKeyParameterNames"),
            ),
            subscription_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionRequired"),
            ),
            terms_of_service_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("termsOfServiceUrl"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            version_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionDescription"),
            ),
            version_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionSetId"),
            ),
        }
    }
}
