/// Resource for managing an AWS Shield Subscription.
///
/// > This resource creates a subscription to AWS Shield Advanced, which requires a 1 year subscription commitment with a monthly fee. Refer to the [AWS Shield Pricing](https://aws.amazon.com/shield/pricing/) page for more details.
///
/// > Destruction of this resource will set `auto_renew` to `DISABLED`. Automatic renewal can only be disabled during the last 30 days of a subscription. To unsubscribe outside of this window, you must contact AWS Support. Set `skip_destroy` to `true` to skip modifying the `auto_renew` argument during destruction.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription::create(
///         "example",
///         SubscriptionArgs::builder().auto_renew("ENABLED").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield Subscription using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:shield/subscription:Subscription example 123456789012
/// ```
pub mod subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// Toggle for automated renewal of the subscription. Valid values are `ENABLED` or `DISABLED`. Default is `ENABLED`.
        #[builder(into, default)]
        pub auto_renew: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Skip attempting to disable automated renewal upon destruction. If set to `true`, the `auto_renew` value will be left as-is and the resource will simply be removed from state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// Toggle for automated renewal of the subscription. Valid values are `ENABLED` or `DISABLED`. Default is `ENABLED`.
        pub auto_renew: pulumi_wasm_rust::Output<String>,
        /// Skip attempting to disable automated renewal upon destruction. If set to `true`, the `auto_renew` value will be left as-is and the resource will simply be removed from state.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubscriptionArgs,
    ) -> SubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_renew_binding = args.auto_renew.get_output(context).get_inner();
        let skip_destroy_binding = args.skip_destroy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/subscription:Subscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoRenew".into(),
                    value: &auto_renew_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionResult {
            auto_renew: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoRenew"),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipDestroy"),
            ),
        }
    }
}
