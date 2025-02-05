/// ## Example Usage
///
/// ### Creating A New Alias And Subscription For An Enrollment Account
///
/// ```yaml
/// resources:
///   exampleSubscription:
///     type: azure:core:Subscription
///     name: example
///     properties:
///       subscriptionName: My Example EA Subscription
///       billingScopeId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:billing:getEnrollmentAccountScope
///       arguments:
///         billingAccountName: '1234567890'
///         enrollmentAccountName: '0123456'
/// ```
///
/// ### Creating A New Alias And Subscription For A Microsoft Customer Account
///
/// ```yaml
/// resources:
///   exampleSubscription:
///     type: azure:core:Subscription
///     name: example
///     properties:
///       subscriptionName: My Example MCA Subscription
///       billingScopeId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:billing:getMcaAccountScope
///       arguments:
///         billingAccountName: e879cf0f-2b4d-5431-109a-f72fc9868693:024cabf4-7321-4cf9-be59-df0c77ca51de_2019-05-31
///         billingProfileName: PE2Q-NOIT-BG7-TGB
///         invoiceSectionName: MTT4-OBS7-PJA-TGB
/// ```
///
/// ### Creating A New Alias And Subscription For A Microsoft Partner Account
///
/// ```yaml
/// resources:
///   exampleSubscription:
///     type: azure:core:Subscription
///     name: example
///     properties:
///       subscriptionName: My Example MPA Subscription
///       billingScopeId: ${example.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:billing:getMpaAccountScope
///       arguments:
///         billingAccountName: e879cf0f-2b4d-5431-109a-f72fc9868693:024cabf4-7321-4cf9-be59-df0c77ca51de_2019-05-31
///         customerName: 2281f543-7321-4cf9-1e23-edb4Oc31a31c
/// ```
///
/// ### Adding An Alias To An Existing Subscription
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription::create(
///         "example",
///         SubscriptionArgs::builder()
///             .alias("examplesub")
///             .subscription_id("12345678-12234-5678-9012-123456789012")
///             .subscription_name("My Example Subscription")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Subscriptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/subscription:Subscription example "/providers/Microsoft.Subscription/aliases/subscription1"
/// ```
///
/// In this scenario, the `subscription_id` property can be completed and the provider will assume control of the existing subscription by creating an Alias. See the `adding an Alias to an existing Subscription` above. This provider requires an alias to correctly manage Subscription resources due to Azure Subscription API design.
///
pub mod subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// The Alias name for the subscription. This provider will generate a new GUID if this is not supplied. Changing this forces a new Subscription to be created.
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Azure Billing Scope ID. Can be a Microsoft Customer Account Billing Scope ID, a Microsoft Partner Account Billing Scope ID or an Enrollment Billing Scope ID.
        #[builder(into, default)]
        pub billing_scope_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Subscription. Changing this forces a new Subscription to be created.
        ///
        /// > **NOTE:** This value can be specified only for adopting control of an existing Subscription, it cannot be used to provide a custom Subscription ID.
        ///
        /// > **NOTE:** Either `billing_scope_id` or `subscription_id` has to be specified.
        #[builder(into, default)]
        pub subscription_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Name of the Subscription. This is the Display Name in the portal.
        #[builder(into)]
        pub subscription_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Subscription.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The workload type of the Subscription. Possible values are `Production` (default) and `DevTest`. Changing this forces a new Subscription to be created.
        #[builder(into, default)]
        pub workload: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// The Alias name for the subscription. This provider will generate a new GUID if this is not supplied. Changing this forces a new Subscription to be created.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// The Azure Billing Scope ID. Can be a Microsoft Customer Account Billing Scope ID, a Microsoft Partner Account Billing Scope ID or an Enrollment Billing Scope ID.
        pub billing_scope_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Subscription. Changing this forces a new Subscription to be created.
        ///
        /// > **NOTE:** This value can be specified only for adopting control of an existing Subscription, it cannot be used to provide a custom Subscription ID.
        ///
        /// > **NOTE:** Either `billing_scope_id` or `subscription_id` has to be specified.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The Name of the Subscription. This is the Display Name in the portal.
        pub subscription_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Subscription.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Tenant to which the subscription belongs.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        /// The workload type of the Subscription. Possible values are `Production` (default) and `DevTest`. Changing this forces a new Subscription to be created.
        pub workload: pulumi_wasm_rust::Output<Option<String>>,
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
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_output(context).get_inner();
        let billing_scope_id_binding = args
            .billing_scope_id
            .get_output(context)
            .get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let subscription_name_binding = args
            .subscription_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workload_binding = args.workload.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/subscription:Subscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "billingScopeId".into(),
                    value: &billing_scope_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionName".into(),
                    value: &subscription_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workload".into(),
                    value: &workload_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionResult {
            alias: pulumi_wasm_rust::__private::into_domain(o.extract_field("alias")),
            billing_scope_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("billingScopeId"),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
            subscription_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptionName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
            workload: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workload"),
            ),
        }
    }
}
