pub mod get_subscriptions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionsArgs {
        /// A case-insensitive value which must be contained within the `display_name` field, used to filter the results
        #[builder(into, default)]
        pub display_name_contains: pulumi_wasm_rust::Output<Option<String>>,
        /// A case-insensitive prefix which can be used to filter on the `display_name` field
        #[builder(into, default)]
        pub display_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionsResult {
        pub display_name_contains: pulumi_wasm_rust::Output<Option<String>>,
        pub display_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `subscription` blocks as defined below.
        pub subscriptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::core::GetSubscriptionsSubscription>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSubscriptionsArgs) -> GetSubscriptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_contains_binding = args.display_name_contains.get_inner();
        let display_name_prefix_binding = args.display_name_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:core/getSubscriptions:getSubscriptions".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayNameContains".into(),
                    value: &display_name_contains_binding,
                },
                register_interface::ObjectField {
                    name: "displayNamePrefix".into(),
                    value: &display_name_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayNameContains".into(),
                },
                register_interface::ResultField {
                    name: "displayNamePrefix".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "subscriptions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSubscriptionsResult {
            display_name_contains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayNameContains").unwrap(),
            ),
            display_name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayNamePrefix").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            subscriptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptions").unwrap(),
            ),
        }
    }
}