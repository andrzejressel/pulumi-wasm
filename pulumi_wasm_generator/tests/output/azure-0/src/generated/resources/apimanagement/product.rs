/// Manages an API Management Product.
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
pub mod product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProductArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        ///
        /// > **NOTE:** `approval_required` can only be set when `subscription_required` is set to `true`.
        #[builder(into, default)]
        pub approval_required: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A description of this Product, which may include HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this API Management Product.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Identifier for this Product, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Is this Product Published?
        #[builder(into)]
        pub published: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Is a Subscription required to access API's included in this Product? Defaults to `true`.
        #[builder(into, default)]
        pub subscription_required: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The number of subscriptions a user can have to this Product at the same time.
        ///
        /// > **NOTE:** `subscriptions_limit` can only be set when `subscription_required` is set to `true`.
        #[builder(into, default)]
        pub subscriptions_limit: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        #[builder(into, default)]
        pub terms: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProductResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        ///
        /// > **NOTE:** `approval_required` can only be set when `subscription_required` is set to `true`.
        pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of this Product, which may include HTML formatting tags.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Display Name for this API Management Product.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The Identifier for this Product, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Is this Product Published?
        pub published: pulumi_wasm_rust::Output<bool>,
        /// The name of the Resource Group in which the API Management Service should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is a Subscription required to access API's included in this Product? Defaults to `true`.
        pub subscription_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// The number of subscriptions a user can have to this Product at the same time.
        ///
        /// > **NOTE:** `subscriptions_limit` can only be set when `subscription_required` is set to `true`.
        pub subscriptions_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        pub terms: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProductArgs,
    ) -> ProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let approval_required_binding = args
            .approval_required
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let published_binding = args.published.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let subscription_required_binding = args
            .subscription_required
            .get_output(context)
            .get_inner();
        let subscriptions_limit_binding = args
            .subscriptions_limit
            .get_output(context)
            .get_inner();
        let terms_binding = args.terms.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/product:Product".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "approvalRequired".into(),
                    value: &approval_required_binding,
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
                    name: "productId".into(),
                    value: &product_id_binding,
                },
                register_interface::ObjectField {
                    name: "published".into(),
                    value: &published_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionRequired".into(),
                    value: &subscription_required_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionsLimit".into(),
                    value: &subscriptions_limit_binding,
                },
                register_interface::ObjectField {
                    name: "terms".into(),
                    value: &terms_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "approvalRequired".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "published".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionRequired".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionsLimit".into(),
                },
                register_interface::ResultField {
                    name: "terms".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProductResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            approval_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalRequired").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            published: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("published").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subscription_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionRequired").unwrap(),
            ),
            subscriptions_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionsLimit").unwrap(),
            ),
            terms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terms").unwrap(),
            ),
        }
    }
}
