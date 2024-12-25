#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListProductFamiliesArgs {
    /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
    #[builder(into, default)]
    pub customer_subscription_details: pulumi_wasm_rust::Output<
        Option<super::super::types::CustomerSubscriptionDetails>,
    >,
    /// $expand is supported on configurations parameter for product, which provides details on the configurations for the product.
    #[builder(into, default)]
    pub expand: pulumi_wasm_rust::Output<Option<String>>,
    /// Dictionary of filterable properties on product family.
    #[builder(into)]
    pub filterable_properties: pulumi_wasm_rust::Output<
        std::collections::HashMap<String, Vec<super::super::types::FilterableProperty>>,
    >,
    /// $skipToken is supported on list of product families, which provides the next page in the list of product families.
    #[builder(into, default)]
    pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ListProductFamiliesResult {
    /// Link for the next set of product families.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    /// List of product families.
    pub value: pulumi_wasm_rust::Output<Vec<super::super::types::ProductFamilyResponse>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: ListProductFamiliesArgs) -> ListProductFamiliesResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let customer_subscription_details_binding = args
        .customer_subscription_details
        .get_inner();
    let expand_binding = args.expand.get_inner();
    let filterable_properties_binding = args.filterable_properties.get_inner();
    let skip_token_binding = args.skip_token.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "myedgeorder::listProductFamilies".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "customerSubscriptionDetails".into(),
                value: &customer_subscription_details_binding,
            },
            register_interface::ObjectField {
                name: "expand".into(),
                value: &expand_binding,
            },
            register_interface::ObjectField {
                name: "filterableProperties".into(),
                value: &filterable_properties_binding,
            },
            register_interface::ObjectField {
                name: "skipToken".into(),
                value: &skip_token_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "nextLink".into() },
            register_interface::ResultField { name : "value".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ListProductFamiliesResult {
        next_link: into_domain(hashmap.remove("nextLink").unwrap()),
        value: into_domain(hashmap.remove("value").unwrap()),
    }
}
