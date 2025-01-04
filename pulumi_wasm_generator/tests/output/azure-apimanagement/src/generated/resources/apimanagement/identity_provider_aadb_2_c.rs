/// Manages an API Management Azure AD B2C Identity Provider.
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
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       publisherName: My Company
///       publisherEmail: company@terraform.io
///       skuName: Developer_1
///   exampleApplication:
///     type: azuread:Application
///     name: example
///     properties:
///       displayName: acctestam-example
///   exampleApplicationPassword:
///     type: azuread:ApplicationPassword
///     name: example
///     properties:
///       applicationObjectId: ${exampleApplication.objectId}
///       endDateRelative: 36h
///   exampleIdentityProviderAadb2c:
///     type: azure:apimanagement:IdentityProviderAadb2c
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       apiManagementName: ${exampleService.name}
///       clientId: ${exampleApplication.applicationId}
///       clientSecret: P@55w0rD!
///       allowedTenant: myb2ctenant.onmicrosoft.com
///       signinTenant: myb2ctenant.onmicrosoft.com
///       authority: myb2ctenant.b2clogin.com
///       signinPolicy: B2C_1_Login
///       signupPolicy: B2C_1_Signup
///     options:
///       dependsOn:
///         - ${exampleApplicationPassword}
/// ```
///
/// ## Import
///
/// API Management Azure AD B2C Identity Providers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/identityProviderAadb2c:IdentityProviderAadb2c example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/service1/identityProviders/aadB2C
/// ```
///
pub mod identity_provider_aadb_2_c {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderAadb2cArgs {
        /// The allowed AAD tenant, usually your B2C tenant domain.
        #[builder(into)]
        pub allowed_tenant: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// OpenID Connect discovery endpoint hostname, usually your b2clogin.com domain.
        #[builder(into)]
        pub authority: pulumi_wasm_rust::Output<String>,
        /// Client ID of the Application in your B2C tenant.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client library to be used in the Azure AD B2C Identity Provider.
        #[builder(into, default)]
        pub client_library: pulumi_wasm_rust::Output<Option<String>>,
        /// Client secret of the Application in your B2C tenant.
        #[builder(into)]
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// Password reset Policy Name.
        #[builder(into, default)]
        pub password_reset_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Profile editing Policy Name.
        #[builder(into, default)]
        pub profile_editing_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Signin Policy Name.
        #[builder(into)]
        pub signin_policy: pulumi_wasm_rust::Output<String>,
        /// The tenant to use instead of Common when logging into Active Directory, usually your B2C tenant domain.
        #[builder(into)]
        pub signin_tenant: pulumi_wasm_rust::Output<String>,
        /// Signup Policy Name.
        #[builder(into)]
        pub signup_policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderAadb2cResult {
        /// The allowed AAD tenant, usually your B2C tenant domain.
        pub allowed_tenant: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// OpenID Connect discovery endpoint hostname, usually your b2clogin.com domain.
        pub authority: pulumi_wasm_rust::Output<String>,
        /// Client ID of the Application in your B2C tenant.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// The client library to be used in the Azure AD B2C Identity Provider.
        pub client_library: pulumi_wasm_rust::Output<Option<String>>,
        /// Client secret of the Application in your B2C tenant.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// Password reset Policy Name.
        pub password_reset_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Profile editing Policy Name.
        pub profile_editing_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Signin Policy Name.
        pub signin_policy: pulumi_wasm_rust::Output<String>,
        /// The tenant to use instead of Common when logging into Active Directory, usually your B2C tenant domain.
        pub signin_tenant: pulumi_wasm_rust::Output<String>,
        /// Signup Policy Name.
        pub signup_policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IdentityProviderAadb2cArgs,
    ) -> IdentityProviderAadb2cResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_tenant_binding = args.allowed_tenant.get_inner();
        let api_management_name_binding = args.api_management_name.get_inner();
        let authority_binding = args.authority.get_inner();
        let client_id_binding = args.client_id.get_inner();
        let client_library_binding = args.client_library.get_inner();
        let client_secret_binding = args.client_secret.get_inner();
        let password_reset_policy_binding = args.password_reset_policy.get_inner();
        let profile_editing_policy_binding = args.profile_editing_policy.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let signin_policy_binding = args.signin_policy.get_inner();
        let signin_tenant_binding = args.signin_tenant.get_inner();
        let signup_policy_binding = args.signup_policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/identityProviderAadb2c:IdentityProviderAadb2c"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedTenant".into(),
                    value: &allowed_tenant_binding,
                },
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "authority".into(),
                    value: &authority_binding,
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
                    name: "passwordResetPolicy".into(),
                    value: &password_reset_policy_binding,
                },
                register_interface::ObjectField {
                    name: "profileEditingPolicy".into(),
                    value: &profile_editing_policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "signinPolicy".into(),
                    value: &signin_policy_binding,
                },
                register_interface::ObjectField {
                    name: "signinTenant".into(),
                    value: &signin_tenant_binding,
                },
                register_interface::ObjectField {
                    name: "signupPolicy".into(),
                    value: &signup_policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedTenant".into(),
                },
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "authority".into(),
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
                    name: "passwordResetPolicy".into(),
                },
                register_interface::ResultField {
                    name: "profileEditingPolicy".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "signinPolicy".into(),
                },
                register_interface::ResultField {
                    name: "signinTenant".into(),
                },
                register_interface::ResultField {
                    name: "signupPolicy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityProviderAadb2cResult {
            allowed_tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedTenant").unwrap(),
            ),
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            authority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authority").unwrap(),
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
            password_reset_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwordResetPolicy").unwrap(),
            ),
            profile_editing_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileEditingPolicy").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            signin_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signinPolicy").unwrap(),
            ),
            signin_tenant: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signinTenant").unwrap(),
            ),
            signup_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signupPolicy").unwrap(),
            ),
        }
    }
}
