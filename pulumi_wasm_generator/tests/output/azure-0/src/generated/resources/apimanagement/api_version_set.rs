/// Manages an API Version Set within an API Management Service.
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
///     let exampleApiVersionSet = api_version_set::create(
///         "exampleApiVersionSet",
///         ApiVersionSetArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .display_name("ExampleAPIVersionSet")
///             .name("example-apimapi-1_0_0")
///             .resource_group_name("${example.name}")
///             .versioning_scheme("Segment")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("pub1@email.com")
///             .publisher_name("pub1")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Version Set can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiVersionSet:ApiVersionSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apiVersionSets/set1
/// ```
///
pub mod api_version_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiVersionSetArgs {
        /// The name of the API Management Service in which the API Version Set should exist. May only contain alphanumeric characters and dashes up to 50 characters in length. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The description of API Version Set.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of this API Version Set.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API Version Set. May only contain alphanumeric characters and dashes up to 80 characters in length. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the parent API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Header which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Header`.
        #[builder(into, default)]
        pub version_header_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Query String which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Query`.
        #[builder(into, default)]
        pub version_query_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies where in an Inbound HTTP Request that the API Version should be read from. Possible values are `Header`, `Query` and `Segment`.
        #[builder(into)]
        pub versioning_scheme: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiVersionSetResult {
        /// The name of the API Management Service in which the API Version Set should exist. May only contain alphanumeric characters and dashes up to 50 characters in length. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The description of API Version Set.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of this API Version Set.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API Version Set. May only contain alphanumeric characters and dashes up to 80 characters in length. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the parent API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Header which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Header`.
        pub version_header_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Query String which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Query`.
        pub version_query_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies where in an Inbound HTTP Request that the API Version should be read from. Possible values are `Header`, `Query` and `Segment`.
        pub versioning_scheme: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiVersionSetArgs) -> ApiVersionSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let version_header_name_binding = args.version_header_name.get_inner();
        let version_query_name_binding = args.version_query_name.get_inner();
        let versioning_scheme_binding = args.versioning_scheme.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiVersionSet:ApiVersionSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "versionHeaderName".into(),
                    value: &version_header_name_binding,
                },
                register_interface::ObjectField {
                    name: "versionQueryName".into(),
                    value: &version_query_name_binding,
                },
                register_interface::ObjectField {
                    name: "versioningScheme".into(),
                    value: &versioning_scheme_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "versionHeaderName".into(),
                },
                register_interface::ResultField {
                    name: "versionQueryName".into(),
                },
                register_interface::ResultField {
                    name: "versioningScheme".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiVersionSetResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            version_header_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionHeaderName").unwrap(),
            ),
            version_query_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionQueryName").unwrap(),
            ),
            versioning_scheme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versioningScheme").unwrap(),
            ),
        }
    }
}
