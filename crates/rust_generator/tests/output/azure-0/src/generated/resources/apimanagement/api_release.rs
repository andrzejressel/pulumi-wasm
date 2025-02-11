/// Manages a API Management API Release.
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
///                         "https://raw.githubusercontent.com/hashicorp/terraform-provider-azurerm/refs/heads/main/internal/services/apimanagement/testdata/api_management_api_swagger.json",
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
///     let exampleApiRelease = api_release::create(
///         "exampleApiRelease",
///         ApiReleaseArgs::builder()
///             .api_id("${exampleApi.id}")
///             .name("example-Api-Release")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@terraform.io")
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
/// API Management API Releases can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiRelease:ApiRelease example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/api1/releases/release1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_release {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiReleaseArgs {
        /// The ID of the API Management API. Changing this forces a new API Management API Release to be created.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this API Management API Release. Changing this forces a new API Management API Release to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Release Notes.
        #[builder(into, default)]
        pub notes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiReleaseResult {
        /// The ID of the API Management API. Changing this forces a new API Management API Release to be created.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this API Management API Release. Changing this forces a new API Management API Release to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Release Notes.
        pub notes: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiReleaseArgs,
    ) -> ApiReleaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let notes_binding = args.notes.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiRelease:ApiRelease".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notes".into(),
                    value: &notes_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiReleaseResult {
            api_id: o.get_field("apiId"),
            name: o.get_field("name"),
            notes: o.get_field("notes"),
        }
    }
}
