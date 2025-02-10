/// Manages an API Management Google Identity Provider.
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
///     let exampleIdentityProviderGoogle = identity_provider_google::create(
///         "exampleIdentityProviderGoogle",
///         IdentityProviderGoogleArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .client_id("00000000.apps.googleusercontent.com")
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
/// API Management Google Identity Provider can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/identityProviderGoogle:IdentityProviderGoogle example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/identityProviders/google
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_provider_google {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderGoogleArgs {
        /// The Name of the API Management Service where this Google Identity Provider should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Client Id for Google Sign-in.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Client secret for Google Sign-in.
        #[builder(into)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderGoogleResult {
        /// The Name of the API Management Service where this Google Identity Provider should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// Client Id for Google Sign-in.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// Client secret for Google Sign-in.
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityProviderGoogleArgs,
    ) -> IdentityProviderGoogleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/identityProviderGoogle:IdentityProviderGoogle"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: client_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityProviderGoogleResult {
            api_management_name: o.get_field("apiManagementName"),
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
