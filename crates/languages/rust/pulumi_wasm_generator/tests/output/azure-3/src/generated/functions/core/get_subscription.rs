pub mod get_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionArgs {
        /// Specifies the ID of the subscription. If this argument is omitted, the subscription ID of the current Azure Resource Manager provider is used.
        #[builder(into, default)]
        pub subscription_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionResult {
        /// The subscription display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The subscription location placement ID.
        pub location_placement_id: pulumi_wasm_rust::Output<String>,
        /// The subscription quota ID.
        pub quota_id: pulumi_wasm_rust::Output<String>,
        /// The subscription spending limit.
        pub spending_limit: pulumi_wasm_rust::Output<String>,
        /// The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subscription GUID.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Subscription.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The subscription tenant ID.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubscriptionArgs,
    ) -> GetSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:core/getSubscription:getSubscription".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubscriptionResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location_placement_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationPlacementId"),
            ),
            quota_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quotaId"),
            ),
            spending_limit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("spendingLimit"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
