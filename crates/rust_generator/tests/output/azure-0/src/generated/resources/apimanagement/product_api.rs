/// Manages an API Management API Assignment to a Product.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleProductApi:
///     type: azure:apimanagement:ProductApi
///     name: example
///     properties:
///       apiName: ${exampleGetApi.name}
///       productId: ${exampleGetProduct.productId}
///       apiManagementName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-api
///         resourceGroupName: example-resources
///   exampleGetApi:
///     fn::invoke:
///       function: azure:apimanagement:getApi
///       arguments:
///         name: search-api
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
///         revision: '2'
///   exampleGetProduct:
///     fn::invoke:
///       function: azure:apimanagement:getProduct
///       arguments:
///         productId: my-product
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
/// ```
///
/// ## Import
///
/// API Management Product API's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/productApi:ProductApi example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/products/exampleId/apis/apiId
/// ```
///
pub mod product_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductApiArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProductApiResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProductApiArgs,
    ) -> ProductApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let api_name_binding = args.api_name.get_output(context).get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/productApi:ProductApi".into(),
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
                    name: "productId".into(),
                    value: &product_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProductApiResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            api_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiName"),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
