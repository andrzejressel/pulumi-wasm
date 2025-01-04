pub mod get_budget_resource_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetResourceGroupArgs {
        /// The name of this Consumption Budget.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the subscription.
        #[builder(into)]
        pub resource_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBudgetResourceGroupResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_wasm_rust::Output<f64>,
        /// A `filter` block as defined below.
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetResourceGroupFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the tag used for the filter.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `notification` block as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetResourceGroupNotification,
            >,
        >,
        pub resource_group_id: pulumi_wasm_rust::Output<String>,
        /// The time covered by a budget.
        pub time_grain: pulumi_wasm_rust::Output<String>,
        /// A `time_period` block as defined below.
        pub time_periods: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetResourceGroupTimePeriod,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBudgetResourceGroupArgs) -> GetBudgetResourceGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_id_binding = args.resource_group_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:consumption/getBudgetResourceGroup:getBudgetResourceGroup"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding,
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
                    name: "resourceGroupId".into(),
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
        GetBudgetResourceGroupResult {
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
            resource_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupId").unwrap(),
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
