//! The list of configurations.
//! API Version: 2020-12-01-preview.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListConfigurationsArgs {
    /// Holds details about product hierarchy information and filterable property.
    #[builder(into)]
    pub configuration_filters: pulumi_wasm_rust::Output<Vec<crate::types::ConfigurationFilters>>,
    /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
    #[builder(into, default)]
    pub customer_subscription_details: pulumi_wasm_rust::Output<Option<crate::types::CustomerSubscriptionDetails>>,
    /// $skipToken is supported on list of configurations, which provides the next page in the list of configurations.
    #[builder(into, default)]
    pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ListConfigurationsResult {
    /// Link for the next set of configurations.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    /// List of configurations.
    pub value: pulumi_wasm_rust::Output<Vec<crate::types::ConfigurationResponse>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: ListConfigurationsArgs
) -> ListConfigurationsResult {

    let result = crate::bindings::pulumi::myedgeorder::list_configurations::invoke(
        &crate::bindings::pulumi::myedgeorder::list_configurations::Args {
                configuration_filters: &args.configuration_filters.get_inner(),
                customer_subscription_details: &args.customer_subscription_details.get_inner(),
                skip_token: &args.skip_token.get_inner(),
        }
    );

    ListConfigurationsResult {
        next_link: crate::into_domain(result.next_link),
        value: crate::into_domain(result.value),
    }
}
