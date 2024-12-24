#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListConfigurationsArgs {
    /// Holds details about product hierarchy information and filterable property.
    #[builder(into)]
    pub configuration_filters: pulumi_wasm_rust::Output<
        Vec<super::super::types::ConfigurationFilters>,
    >,
    /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
    #[builder(into, default)]
    pub customer_subscription_details: pulumi_wasm_rust::Output<
        Option<super::super::types::CustomerSubscriptionDetails>,
    >,
    /// $skipToken is supported on list of configurations, which provides the next page in the list of configurations.
    #[builder(into, default)]
    pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ListConfigurationsResult {
    /// Link for the next set of configurations.
    pub next_link: pulumi_wasm_rust::Output<Option<String>>,
    /// List of configurations.
    pub value: pulumi_wasm_rust::Output<Vec<super::super::types::ConfigurationResponse>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: ListConfigurationsArgs) -> ListConfigurationsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let configuration_filters_binding = args.configuration_filters.get_inner();
    let customer_subscription_details_binding = args
        .customer_subscription_details
        .get_inner();
    let skip_token_binding = args.skip_token.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "myedgeorder::listConfigurations".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "configurationFilters".into(),
                value: &configuration_filters_binding,
            },
            register_interface::ObjectField {
                name: "customerSubscriptionDetails".into(),
                value: &customer_subscription_details_binding,
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
    ListConfigurationsResult {
        next_link: into_domain(hashmap.remove("nextLink").unwrap()),
        value: into_domain(hashmap.remove("value").unwrap()),
    }
}
