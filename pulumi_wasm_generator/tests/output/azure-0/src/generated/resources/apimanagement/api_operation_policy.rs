/// Manages an API Management API Operation Policy
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
///             .name("example-api")
///             .resource_group_name("${example.name}")
///             .revision("1")
///             .build_struct(),
///     );
///     let exampleApiOperation = api_operation::create(
///         "exampleApiOperation",
///         ApiOperationArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .api_name("${exampleApi.name}")
///             .display_name("DELETE Resource")
///             .method("DELETE")
///             .operation_id("acctest-operation")
///             .resource_group_name("${example.name}")
///             .url_template("/resource")
///             .build_struct(),
///     );
///     let exampleApiOperationPolicy = api_operation_policy::create(
///         "exampleApiOperationPolicy",
///         ApiOperationPolicyArgs::builder()
///             .api_management_name("${exampleApiOperation.apiManagementName}")
///             .api_name("${exampleApiOperation.apiName}")
///             .operation_id("${exampleApiOperation.operationId}")
///             .resource_group_name("${exampleApiOperation.resourceGroupName}")
///             .xml_content(
///                 "<policies>\n  <inbound>\n    <find-and-replace from=\"xyz\" to=\"abc\" />\n  </inbound>\n</policies>",
///             )
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
/// API Management API Operation Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiOperationPolicy:ApiOperationPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/instance1/apis/api1/operations/operation1
/// ```
///
pub mod api_operation_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiOperationPolicyArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API within the API Management Service where the Operation exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// The operation identifier within an API. Must be unique in the current API Management service instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub operation_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The XML Content for this Policy.
        #[builder(into, default)]
        pub xml_content: pulumi_wasm_rust::Output<Option<String>>,
        /// A link to a Policy XML Document, which must be publicly available.
        #[builder(into, default)]
        pub xml_link: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiOperationPolicyResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API within the API Management Service where the Operation exists. Changing this forces a new resource to be created.
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// The operation identifier within an API. Must be unique in the current API Management service instance. Changing this forces a new resource to be created.
        pub operation_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The XML Content for this Policy.
        pub xml_content: pulumi_wasm_rust::Output<String>,
        /// A link to a Policy XML Document, which must be publicly available.
        pub xml_link: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiOperationPolicyArgs) -> ApiOperationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let api_name_binding = args.api_name.get_inner();
        let operation_id_binding = args.operation_id.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let xml_content_binding = args.xml_content.get_inner();
        let xml_link_binding = args.xml_link.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiOperationPolicy:ApiOperationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "apiName".into(),
                    value: &api_name_binding,
                },
                register_interface::ObjectField {
                    name: "operationId".into(),
                    value: &operation_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "xmlContent".into(),
                    value: &xml_content_binding,
                },
                register_interface::ObjectField {
                    name: "xmlLink".into(),
                    value: &xml_link_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "apiName".into(),
                },
                register_interface::ResultField {
                    name: "operationId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "xmlContent".into(),
                },
                register_interface::ResultField {
                    name: "xmlLink".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiOperationPolicyResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            api_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiName").unwrap(),
            ),
            operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            xml_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xmlContent").unwrap(),
            ),
            xml_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xmlLink").unwrap(),
            ),
        }
    }
}
