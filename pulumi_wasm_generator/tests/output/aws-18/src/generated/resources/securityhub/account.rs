/// Enables Security Hub for this AWS account.
///
/// > **NOTE:** Destroying this resource will disable Security Hub for this AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub enabled account using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/account:Account example 123456789012
/// ```
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Whether to automatically enable new controls when they are added to standards that are enabled. By default, this is set to true, and new controls are enabled automatically. To not automatically enable new controls, set this to false.
        #[builder(into, default)]
        pub auto_enable_controls: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Updates whether the calling account has consolidated control findings turned on. If the value for this field is set to `SECURITY_CONTROL`, Security Hub generates a single finding for a control check even when the check applies to multiple enabled standards. If the value for this field is set to `STANDARD_CONTROL`, Security Hub generates separate findings for a control check when the check applies to multiple enabled standards. For accounts that are part of an organization, this value can only be updated in the administrator account.
        #[builder(into, default)]
        pub control_finding_generator: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to enable the security standards that Security Hub has designated as automatically enabled including: ` AWS Foundational Security Best Practices v1.0.0` and `CIS AWS Foundations Benchmark v1.2.0`. Defaults to `true`.
        #[builder(into, default)]
        pub enable_default_standards: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// ARN of the SecurityHub Hub created in the account.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to automatically enable new controls when they are added to standards that are enabled. By default, this is set to true, and new controls are enabled automatically. To not automatically enable new controls, set this to false.
        pub auto_enable_controls: pulumi_wasm_rust::Output<Option<bool>>,
        /// Updates whether the calling account has consolidated control findings turned on. If the value for this field is set to `SECURITY_CONTROL`, Security Hub generates a single finding for a control check even when the check applies to multiple enabled standards. If the value for this field is set to `STANDARD_CONTROL`, Security Hub generates separate findings for a control check when the check applies to multiple enabled standards. For accounts that are part of an organization, this value can only be updated in the administrator account.
        pub control_finding_generator: pulumi_wasm_rust::Output<String>,
        /// Whether to enable the security standards that Security Hub has designated as automatically enabled including: ` AWS Foundational Security Best Practices v1.0.0` and `CIS AWS Foundations Benchmark v1.2.0`. Defaults to `true`.
        pub enable_default_standards: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_enable_controls_binding = args
            .auto_enable_controls
            .get_output(context)
            .get_inner();
        let control_finding_generator_binding = args
            .control_finding_generator
            .get_output(context)
            .get_inner();
        let enable_default_standards_binding = args
            .enable_default_standards
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoEnableControls".into(),
                    value: &auto_enable_controls_binding,
                },
                register_interface::ObjectField {
                    name: "controlFindingGenerator".into(),
                    value: &control_finding_generator_binding,
                },
                register_interface::ObjectField {
                    name: "enableDefaultStandards".into(),
                    value: &enable_default_standards_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_enable_controls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoEnableControls"),
            ),
            control_finding_generator: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("controlFindingGenerator"),
            ),
            enable_default_standards: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableDefaultStandards"),
            ),
        }
    }
}
