//! The list of product families.
//! API Version: 2020-12-01-preview.

    #[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListProductFamiliesArgs {
    /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub customer_subscription_details: pulumi_wasm_rust::Output<Option<crate::types::CustomerSubscriptionDetails>>,
    /// $expand is supported on configurations parameter for product, which provides details on the configurations for the product.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub expand: pulumi_wasm_rust::Output<Option<String>>,
    /// Dictionary of filterable properties on product family.
    #[builder(into)]
    pub filterable_properties: pulumi_wasm_rust::Output<std::collections::HashMap<String, Vec<crate::types::FilterableProperty>>>,
    /// $skipToken is supported on list of product families, which provides the next page in the list of product families.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ListProductFamiliesResult {
    /// Link for the next set of product families.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    /// List of product families.
    pub value: pulumi_wasm_rust::Output<Vec<crate::types::ProductFamilyResponse>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: ListProductFamiliesArgs
) -> ListProductFamiliesResult {

    let result = crate::bindings::pulumi::myedgeorder::list_product_families::invoke(
        &crate::bindings::pulumi::myedgeorder::list_product_families::Args {
                customer_subscription_details: &args.customer_subscription_details.get_inner(),
                expand: &args.expand.get_inner(),
                filterable_properties: &args.filterable_properties.get_inner(),
                skip_token: &args.skip_token.get_inner(),
        }
    );

    ListProductFamiliesResult {
        next_link: crate::into_domain(result.next_link),
        value: crate::into_domain(result.value),
    }
}
