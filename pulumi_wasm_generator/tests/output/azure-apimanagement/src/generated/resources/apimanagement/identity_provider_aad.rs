/// Manages an API Management AAD Identity Provider.
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
///     let exampleIdentityProviderAad = identity_provider_aad::create(
///         "exampleIdentityProviderAad",
///         IdentityProviderAadArgs::builder()
///             .allowed_tenants(vec!["00000000-0000-0000-0000-000000000000",])
///             .api_management_name("${exampleService.name}")
///             .client_id("00000000-0000-0000-0000-000000000000")
///             .client_secret("00000000000000000000000000000000")
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
/// API Management AAD Identity Provider can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/identityProviderAad:IdentityProviderAad example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/identityProviders/aad
/// ```
///
pub mod identity_provider_aad {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderAadArgs {
        /// List of allowed AAD Tenants.
        #[builder(into)]
        pub allowed_tenants: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Client Id of the Application in the AAD Identity Provider.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client library to be used in the AAD Identity Provider.
        #[builder(into, default)]
        pub client_library: pulumi_wasm_rust::Output<Option<String>>,
        /// Client secret of the Application in the AAD Identity Provider.
        #[builder(into)]
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The AAD Tenant to use instead of Common when logging into Active Directory.
        #[builder(into, default)]
        pub signin_tenant: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderAadResult {
        /// List of allowed AAD Tenants.
        pub allowed_tenants: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Client Id of the Application in the AAD Identity Provider.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client library to be used in the AAD Identity Provider.
        pub client_library: pulumi_wasm_rust::Output<Option<String>>,
        /// Client secret of the Application in the AAD Identity Provider.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The AAD Tenant to use instead of Common when logging into Active Directory.
        pub signin_tenant: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IdentityProviderAadArgs,
    ) -> IdentityProviderAadResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_tenants_binding = args.allowed_tenants.get_inner();
        let api_management_name_binding = args.api_management_name.get_inner();
        let client_id_binding = args.client_id.get_inner();
        let client_library_binding = args.client_library.get_inner();
        let client_secret_binding = args.client_secret.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let signin_tenant_binding = args.signin_tenant.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/identityProviderAad:IdentityProviderAad".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedTenants".into(),
                    value: &allowed_tenants_binding,
                },
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientLibrary".into(),
                    value: &client_library_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "signinTenant".into(),
                    value: &signin_tenant_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedTenants".into(),
                },
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientLibrary".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "signinTenant".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityProviderAadResult {
            allowed_tenants: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedTenants").unwrap(),
            ),
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_library: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientLibrary").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            signin_tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signinTenant").unwrap(),
            ),
        }
    }
}