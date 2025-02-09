/// Manages an API Management Product Policy
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleProductPolicy:
///     type: azure:apimanagement:ProductPolicy
///     name: example
///     properties:
///       productId: ${example.productId}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       xmlContent: |
///         <policies>
///           <inbound>
///             <find-and-replace from="xyz" to="abc" />
///           </inbound>
///         </policies>
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getProduct
///       arguments:
///         productId: my-product
///         apiManagementName: example-apim
///         resourceGroupName: search-service
/// ```
///
/// ## Import
///
/// API Management Product Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/productPolicy:ProductPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/products/product1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductPolicyArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The XML Content for this Policy.
        #[builder(into, default)]
        pub xml_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A link to a Policy XML Document, which must be publicly available.
        #[builder(into, default)]
        pub xml_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProductPolicyResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the API Management Product within the API Management Service. Changing this forces a new resource to be created.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The XML Content for this Policy.
        pub xml_content: pulumi_gestalt_rust::Output<String>,
        /// A link to a Policy XML Document, which must be publicly available.
        pub xml_link: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductPolicyArgs,
    ) -> ProductPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let product_id_binding = args.product_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let xml_content_binding = args.xml_content.get_output(context);
        let xml_link_binding = args.xml_link.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/productPolicy:ProductPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productId".into(),
                    value: product_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xmlContent".into(),
                    value: xml_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xmlLink".into(),
                    value: xml_link_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProductPolicyResult {
            api_management_name: o.get_field("apiManagementName"),
            product_id: o.get_field("productId"),
            resource_group_name: o.get_field("resourceGroupName"),
            xml_content: o.get_field("xmlContent"),
            xml_link: o.get_field("xmlLink"),
        }
    }
}
