/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-exclusion:
///     type: gcp:logging:BillingAccountExclusion
///     properties:
///       name: my-instance-debug-exclusion
///       billingAccount: ABCDEF-012345-GHIJKL
///       description: Exclude GCE instance debug logs
///       filter: resource.type = gce_instance AND severity <= DEBUG
/// ```
///
/// ## Import
///
/// Billing account logging exclusions can be imported using their URI, e.g.
///
/// * `billingAccounts/{{billing_account}}/exclusions/{{name}}`
///
/// When using the `pulumi import` command, billing account logging exclusions can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/billingAccountExclusion:BillingAccountExclusion default billingAccounts/{{billing_account}}/exclusions/{{name}}
/// ```
///
pub mod billing_account_exclusion {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BillingAccountExclusionArgs {
        /// The billing account to create the exclusion for.
        #[builder(into)]
        pub billing_account: pulumi_wasm_rust::InputOrOutput<String>,
        /// A human-readable description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the logging exclusion.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BillingAccountExclusionResult {
        /// The billing account to create the exclusion for.
        pub billing_account: pulumi_wasm_rust::Output<String>,
        /// A human-readable description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The name of the logging exclusion.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BillingAccountExclusionArgs,
    ) -> BillingAccountExclusionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/billingAccountExclusion:BillingAccountExclusion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BillingAccountExclusionResult {
            billing_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("billingAccount"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
