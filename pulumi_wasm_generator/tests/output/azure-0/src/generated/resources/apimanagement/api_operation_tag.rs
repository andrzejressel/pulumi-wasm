/// Manages a API Management API Operation Tag.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApiOperation:
///     type: azure:apimanagement:ApiOperation
///     name: example
///     properties:
///       operationId: user-delete
///       apiName: ${example.name}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       displayName: Delete User Operation
///       method: DELETE
///       urlTemplate: /users/{id}/delete
///       description: This can only be done by the logged in user.
///       templateParameters:
///         - name: id
///           type: number
///           required: true
///       responses:
///         - statusCode: 200
///   exampleApiOperationTag:
///     type: azure:apimanagement:ApiOperationTag
///     name: example
///     properties:
///       name: example-Tag
///       apiOperationId: ${exampleApiOperation.id}
///       displayName: example-Tag
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
/// API Management API Operation Tags can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiOperationTag:ApiOperationTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/api1/operations/operation1/tags/tag1
/// ```
///
pub mod api_operation_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiOperationTagArgs {
        /// The ID of the API Management API Operation. Changing this forces a new API Management API Operation Tag to be created.
        #[builder(into)]
        pub api_operation_id: pulumi_wasm_rust::Output<String>,
        /// The display name of the API Management API Operation Tag.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this API Management API Operation Tag. Changing this forces a new API Management API Operation Tag to be created. The name must be unique in the API Management Service.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiOperationTagResult {
        /// The ID of the API Management API Operation. Changing this forces a new API Management API Operation Tag to be created.
        pub api_operation_id: pulumi_wasm_rust::Output<String>,
        /// The display name of the API Management API Operation Tag.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this API Management API Operation Tag. Changing this forces a new API Management API Operation Tag to be created. The name must be unique in the API Management Service.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiOperationTagArgs) -> ApiOperationTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_operation_id_binding = args.api_operation_id.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiOperationTag:ApiOperationTag".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiOperationId".into(),
                    value: &api_operation_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiOperationId".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiOperationTagResult {
            api_operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiOperationId").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
