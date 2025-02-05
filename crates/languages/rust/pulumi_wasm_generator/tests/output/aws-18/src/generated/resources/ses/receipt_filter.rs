/// Provides an SES receipt filter resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let filter = receipt_filter::create(
///         "filter",
///         ReceiptFilterArgs::builder()
///             .cidr("10.10.10.10")
///             .name("block-spammer")
///             .policy("Block")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES Receipt Filter using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/receiptFilter:ReceiptFilter test some-filter
/// ```
pub mod receipt_filter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReceiptFilterArgs {
        /// The IP address or address range to filter, in CIDR notation
        #[builder(into)]
        pub cidr: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the filter
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Block or Allow
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReceiptFilterResult {
        /// The SES receipt filter ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The IP address or address range to filter, in CIDR notation
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// The name of the filter
        pub name: pulumi_wasm_rust::Output<String>,
        /// Block or Allow
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReceiptFilterArgs,
    ) -> ReceiptFilterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_binding = args.cidr.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/receiptFilter:ReceiptFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReceiptFilterResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cidr: pulumi_wasm_rust::__private::into_domain(o.extract_field("cidr")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
        }
    }
}
