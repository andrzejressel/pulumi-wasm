/// Manages the Assignment of an API Management API Tag to an API.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleApi:
///     type: azure:apimanagement:Api
///     name: example
///     properties:
///       name: example-api
///       resourceGroupName: ${exampleResourceGroup.name}
///       apiManagementName: ${example.name}
///       revision: '1'
///   exampleTag:
///     type: azure:apimanagement:Tag
///     name: example
///     properties:
///       apiManagementId: ${example.id}
///       name: example-tag
///   exampleApiTag:
///     type: azure:apimanagement:ApiTag
///     name: example
///     properties:
///       apiId: ${exampleApi.id}
///       name: ${exampleTag.name}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-apim
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// API Management API Tags can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiTag:ApiTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/api1/tags/tag1
/// ```
///
pub mod api_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiTagArgs {
        /// The ID of the API Management API. Changing this forces a new API Management API Tag to be created.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The name of the tag. It must be known in the API Management instance. Changing this forces a new API Management API Tag to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiTagResult {
        /// The ID of the API Management API. Changing this forces a new API Management API Tag to be created.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The name of the tag. It must be known in the API Management instance. Changing this forces a new API Management API Tag to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiTagArgs) -> ApiTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiTag:ApiTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
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
        ApiTagResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
