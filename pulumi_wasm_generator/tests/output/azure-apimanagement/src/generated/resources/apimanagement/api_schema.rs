/// Manages an API Schema within an API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApiSchema:
///     type: azure:apimanagement:ApiSchema
///     name: example
///     properties:
///       apiName: ${example.name}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       schemaId: example-schema
///       contentType: application/vnd.ms-azure-apim.xsd+xml
///       value:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: api_management_api_schema.xml
///           return: result
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getApi
///       arguments:
///         name: search-api
///         apiManagementName: search-api-management
///         resourceGroupName: search-service
///         revision: '2'
/// ```
///
/// ## Import
///
/// API Management API Schema's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiSchema:ApiSchema example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/apis/api1/schemas/schema1
/// ```
///
pub mod api_schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiSchemaArgs {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API within the API Management Service where this API Schema should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v2/v3 schemas only.
        #[builder(into, default)]
        pub components: pulumi_wasm_rust::Output<Option<String>>,
        /// The content type of the API Schema.
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v1 schemas only.
        #[builder(into, default)]
        pub definitions: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for this API Schema. Changing this forces a new resource to be created.
        #[builder(into)]
        pub schema_id: pulumi_wasm_rust::Output<String>,
        /// The JSON escaped string defining the document representing the Schema.
        #[builder(into, default)]
        pub value: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiSchemaResult {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API within the API Management Service where this API Schema should be created. Changing this forces a new resource to be created.
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v2/v3 schemas only.
        pub components: pulumi_wasm_rust::Output<Option<String>>,
        /// The content type of the API Schema.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v1 schemas only.
        pub definitions: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for this API Schema. Changing this forces a new resource to be created.
        pub schema_id: pulumi_wasm_rust::Output<String>,
        /// The JSON escaped string defining the document representing the Schema.
        pub value: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiSchemaArgs) -> ApiSchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let api_name_binding = args.api_name.get_inner();
        let components_binding = args.components.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let definitions_binding = args.definitions.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let schema_id_binding = args.schema_id.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiSchema:ApiSchema".into(),
            name: name.to_string(),
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
                    name: "components".into(),
                    value: &components_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "definitions".into(),
                    value: &definitions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "schemaId".into(),
                    value: &schema_id_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
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
                    name: "components".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "definitions".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "schemaId".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiSchemaResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            api_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiName").unwrap(),
            ),
            components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("components").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definitions").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            schema_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaId").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}