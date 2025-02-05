pub mod get_subscriptions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionsArgs {
        /// A case-insensitive value which must be contained within the `display_name` field, used to filter the results
        #[builder(into, default)]
        pub display_name_contains: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A case-insensitive prefix which can be used to filter on the `display_name` field
        #[builder(into, default)]
        pub display_name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubscriptionsArgs,
    ) -> GetSubscriptionsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_contains_binding = args
            .display_name_contains
            .get_output(context)
            .get_inner();
        let display_name_prefix_binding = args
            .display_name_prefix
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:core/getSubscriptions:getSubscriptions".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubscriptionsResult {
            display_name_contains: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayNameContains"),
            ),
            display_name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayNamePrefix"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            subscriptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptions"),
            ),
        }
    }
}
