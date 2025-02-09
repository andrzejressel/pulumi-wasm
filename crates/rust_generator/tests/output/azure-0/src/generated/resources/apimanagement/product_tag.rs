/// Manages an API Management Product tag
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
///     let exampleProduct = product::create(
///         "exampleProduct",
///         ProductArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .approval_required(true)
///             .display_name("Test Product")
///             .product_id("test-product")
///             .published(true)
///             .resource_group_name("${example.name}")
///             .subscription_required(true)
///             .build_struct(),
///     );
///     let exampleProductTag = product_tag::create(
///         "exampleProductTag",
///         ProductTagArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .api_management_product_id("${exampleProduct.productId}")
///             .name("${exampleTag.name}")
///             .resource_group_name("${example.name}")
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
///     let exampleTag = tag::create(
///         "exampleTag",
///         TagArgs::builder()
///             .api_management_id("${exampleService.id}")
///             .name("example-tag")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Products can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/productTag:ProductTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/products/myproduct/tags/mytag
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductTagArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management product. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this API Management Tag. Changing this forces a new API Management Tag to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProductTagResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management product. Changing this forces a new resource to be created.
        pub api_management_product_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this API Management Tag. Changing this forces a new API Management Tag to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductTagArgs,
    ) -> ProductTagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let api_management_product_id_binding = args
            .api_management_product_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/productTag:ProductTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementProductId".into(),
                    value: api_management_product_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProductTagResult {
            api_management_name: o.get_field("apiManagementName"),
            api_management_product_id: o.get_field("apiManagementProductId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
