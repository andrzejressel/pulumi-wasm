/// Manages a API Management API Release.
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
pub mod api_release {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiReleaseArgs {
        /// The ID of the API Management API. Changing this forces a new API Management API Release to be created.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this API Management API Release. Changing this forces a new API Management API Release to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Release Notes.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiReleaseResult {
        /// The ID of the API Management API. Changing this forces a new API Management API Release to be created.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this API Management API Release. Changing this forces a new API Management API Release to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Release Notes.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiReleaseArgs) -> ApiReleaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let name_binding = args.name.get_inner();
        let notes_binding = args.notes.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiRelease:ApiRelease".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiReleaseResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
        }
    }
}