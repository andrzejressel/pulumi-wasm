pub mod get_budget_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetSubscriptionArgs {
        /// The name of this Consumption Budget.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the subscription.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBudgetSubscriptionResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_wasm_rust::Output<f64>,
        /// A `filter` block as defined below.
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetSubscriptionFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the tag to use for the filter.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notification` block as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetSubscriptionNotification,
            >,
        >,
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The time covered by a budget.
        pub time_grain: pulumi_wasm_rust::Output<String>,
        /// A `time_period` block as defined below.
        pub time_periods: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetSubscriptionTimePeriod>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBudgetSubscriptionArgs) -> GetBudgetSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let subscription_id_binding = args.subscription_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:consumption/getBudgetSubscription:getBudgetSubscription"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "amount".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notifications".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionId".into(),
                },
                register_interface::ResultField {
                    name: "timeGrain".into(),
                },
                register_interface::ResultField {
                    name: "timePeriods".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBudgetSubscriptionResult {
            amount: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amount").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionId").unwrap(),
            ),
            time_grain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeGrain").unwrap(),
            ),
            time_periods: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timePeriods").unwrap(),
            ),
        }
    }
}