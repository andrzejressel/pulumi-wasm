/// Manages an API Management Product.
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
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
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
/// API Management Products can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/product:Product example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/products/myproduct
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod product {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        ///
        /// > **NOTE:** `approval_required` can only be set when `subscription_required` is set to `true`.
        #[builder(into, default)]
        pub approval_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of this Product, which may include HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this API Management Product.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for this Product, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is this Product Published?
        #[builder(into)]
        pub published: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is a Subscription required to access API's included in this Product? Defaults to `true`.
        #[builder(into, default)]
        pub subscription_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of subscriptions a user can have to this Product at the same time.
        ///
        /// > **NOTE:** `subscriptions_limit` can only be set when `subscription_required` is set to `true`.
        #[builder(into, default)]
        pub subscriptions_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        #[builder(into, default)]
        pub terms: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProductResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        ///
        /// > **NOTE:** `approval_required` can only be set when `subscription_required` is set to `true`.
        pub approval_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description of this Product, which may include HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Display Name for this API Management Product.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The Identifier for this Product, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Is this Product Published?
        pub published: pulumi_gestalt_rust::Output<bool>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is a Subscription required to access API's included in this Product? Defaults to `true`.
        pub subscription_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of subscriptions a user can have to this Product at the same time.
        ///
        /// > **NOTE:** `subscriptions_limit` can only be set when `subscription_required` is set to `true`.
        pub subscriptions_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        pub terms: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProductArgs,
    ) -> ProductResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let approval_required_binding = args.approval_required.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let product_id_binding = args.product_id.get_output(context);
        let published_binding = args.published.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subscription_required_binding = args
            .subscription_required
            .get_output(context);
        let subscriptions_limit_binding = args.subscriptions_limit.get_output(context);
        let terms_binding = args.terms.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/product:Product".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalRequired".into(),
                    value: &approval_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productId".into(),
                    value: &product_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "published".into(),
                    value: &published_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionRequired".into(),
                    value: &subscription_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionsLimit".into(),
                    value: &subscriptions_limit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terms".into(),
                    value: &terms_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProductResult {
            api_management_name: o.get_field("apiManagementName"),
            approval_required: o.get_field("approvalRequired"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            product_id: o.get_field("productId"),
            published: o.get_field("published"),
            resource_group_name: o.get_field("resourceGroupName"),
            subscription_required: o.get_field("subscriptionRequired"),
            subscriptions_limit: o.get_field("subscriptionsLimit"),
            terms: o.get_field("terms"),
        }
    }
}
