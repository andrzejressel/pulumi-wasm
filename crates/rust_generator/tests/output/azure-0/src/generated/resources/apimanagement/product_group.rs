/// Manages an API Management Product Assignment to a Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleProductGroup:
///     type: azure:apimanagement:ProductGroup
///     name: example
///     properties:
///       productId: ${exampleGetProduct.productId}
///       groupName: ${exampleGetGroup.name}
///       apiManagementName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-api
///         resourceGroupName: example-resources
///   exampleGetProduct:
///     fn::invoke:
///       function: azure:apimanagement:getProduct
///       arguments:
///         productId: my-product
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
///   exampleGetGroup:
///     fn::invoke:
///       function: azure:apimanagement:getGroup
///       arguments:
///         name: my-group
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
/// ```
///
/// ## Import
///
/// API Management Product Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/productGroup:ProductGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/products/exampleId/groups/groupId
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductGroupArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProductGroupResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProductGroupArgs,
    ) -> ProductGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding_1 = args.api_management_name.get_output(context);
        let api_management_name_binding = api_management_name_binding_1.get_inner();
        let group_name_binding_1 = args.group_name.get_output(context);
        let group_name_binding = group_name_binding_1.get_inner();
        let product_id_binding_1 = args.product_id.get_output(context);
        let product_id_binding = product_id_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/productGroup:ProductGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
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
        ProductGroupResult {
            api_management_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupName"),
            ),
            product_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
