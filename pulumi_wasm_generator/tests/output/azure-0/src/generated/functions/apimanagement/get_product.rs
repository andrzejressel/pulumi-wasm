pub mod get_product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// The Name of the API Management Service in which this Product exists.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Identifier for the API Management Product.
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        pub approval_required: pulumi_wasm_rust::Output<bool>,
        /// The description of this Product, which may include HTML formatting tags.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The Display Name for this API Management Product.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Is this Product Published?
        pub published: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is a Subscription required to access API's included in this Product?
        pub subscription_required: pulumi_wasm_rust::Output<bool>,
        /// The number of subscriptions a user can have to this Product at the same time.
        pub subscriptions_limit: pulumi_wasm_rust::Output<i32>,
        /// Any Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        pub terms: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetProductArgs,
    ) -> GetProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getProduct:getProduct".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProductResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
