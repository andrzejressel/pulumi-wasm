/// Manages an API Management Facebook Identity Provider.
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
///     let exampleIdentityProviderFacebook = identity_provider_facebook::create(
///         "exampleIdentityProviderFacebook",
///         IdentityProviderFacebookArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .app_id("00000000000000000000000000000000")
///             .app_secret("00000000000000000000000000000000")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@mycompany.io")
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
/// API Management Facebook Identity Provider can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/identityProviderFacebook:IdentityProviderFacebook example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/identityProviders/facebook
/// ```
///
pub mod identity_provider_facebook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderFacebookArgs {
        /// The Name of the API Management Service where this Facebook Identity Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// App ID for Facebook.
        #[builder(into)]
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// App Secret for Facebook.
        #[builder(into)]
        pub app_secret: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderFacebookResult {
        /// The Name of the API Management Service where this Facebook Identity Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// App ID for Facebook.
        pub app_id: pulumi_wasm_rust::Output<String>,
        /// App Secret for Facebook.
        pub app_secret: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IdentityProviderFacebookArgs,
    ) -> IdentityProviderFacebookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let app_id_binding = args.app_id.get_inner();
        let app_secret_binding = args.app_secret.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/identityProviderFacebook:IdentityProviderFacebook"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "appSecret".into(),
                    value: &app_secret_binding,
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
                    name: "appId".into(),
                },
                register_interface::ResultField {
                    name: "appSecret".into(),
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
        IdentityProviderFacebookResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appId").unwrap(),
            ),
            app_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSecret").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}