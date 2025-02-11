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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_provider_aadb_2_c {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderAadb2cArgs {
        /// The allowed AAD tenant, usually your B2C tenant domain.
        #[builder(into)]
        pub allowed_tenant: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// OpenID Connect discovery endpoint hostname, usually your b2clogin.com domain.
        #[builder(into)]
        pub authority: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Client ID of the Application in your B2C tenant.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client library to be used in the Azure AD B2C Identity Provider.
        #[builder(into, default)]
        pub client_library: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Client secret of the Application in your B2C tenant.
        #[builder(into)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password reset Policy Name.
        #[builder(into, default)]
        pub password_reset_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Profile editing Policy Name.
        #[builder(into, default)]
        pub profile_editing_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Signin Policy Name.
        #[builder(into)]
        pub signin_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The tenant to use instead of Common when logging into Active Directory, usually your B2C tenant domain.
        #[builder(into)]
        pub signin_tenant: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Signup Policy Name.
        #[builder(into)]
        pub signup_policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderAadb2cResult {
        /// The allowed AAD tenant, usually your B2C tenant domain.
        pub allowed_tenant: pulumi_gestalt_rust::Output<String>,
        /// The Name of the API Management Service where this AAD Identity Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// OpenID Connect discovery endpoint hostname, usually your b2clogin.com domain.
        pub authority: pulumi_gestalt_rust::Output<String>,
        /// Client ID of the Application in your B2C tenant.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The client library to be used in the Azure AD B2C Identity Provider.
        pub client_library: pulumi_gestalt_rust::Output<Option<String>>,
        /// Client secret of the Application in your B2C tenant.
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// Password reset Policy Name.
        pub password_reset_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Profile editing Policy Name.
        pub profile_editing_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Signin Policy Name.
        pub signin_policy: pulumi_gestalt_rust::Output<String>,
        /// The tenant to use instead of Common when logging into Active Directory, usually your B2C tenant domain.
        pub signin_tenant: pulumi_gestalt_rust::Output<String>,
        /// Signup Policy Name.
        pub signup_policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityProviderAadb2cArgs,
    ) -> IdentityProviderAadb2cResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_tenant_binding = args.allowed_tenant.get_output(context);
        let api_management_name_binding = args.api_management_name.get_output(context);
        let authority_binding = args.authority.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_library_binding = args.client_library.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let password_reset_policy_binding = args
            .password_reset_policy
            .get_output(context);
        let profile_editing_policy_binding = args
            .profile_editing_policy
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let signin_policy_binding = args.signin_policy.get_output(context);
        let signin_tenant_binding = args.signin_tenant.get_output(context);
        let signup_policy_binding = args.signup_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/identityProviderAadb2c:IdentityProviderAadb2c"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedTenant".into(),
                    value: &allowed_tenant_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authority".into(),
                    value: &authority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientLibrary".into(),
                    value: &client_library_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwordResetPolicy".into(),
                    value: &password_reset_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileEditingPolicy".into(),
                    value: &profile_editing_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signinPolicy".into(),
                    value: &signin_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signinTenant".into(),
                    value: &signin_tenant_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signupPolicy".into(),
                    value: &signup_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityProviderAadb2cResult {
            allowed_tenant: o.get_field("allowedTenant"),
            api_management_name: o.get_field("apiManagementName"),
            authority: o.get_field("authority"),
            client_id: o.get_field("clientId"),
            client_library: o.get_field("clientLibrary"),
            client_secret: o.get_field("clientSecret"),
            password_reset_policy: o.get_field("passwordResetPolicy"),
            profile_editing_policy: o.get_field("profileEditingPolicy"),
            resource_group_name: o.get_field("resourceGroupName"),
            signin_policy: o.get_field("signinPolicy"),
            signin_tenant: o.get_field("signinTenant"),
            signup_policy: o.get_field("signupPolicy"),
        }
    }
}
