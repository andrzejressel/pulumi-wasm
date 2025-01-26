/// Resource for managing an AWS Bedrock Guardrail Version.
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
///     let example = guardrail_version::create(
///         "example",
///         GuardrailVersionArgs::builder()
///             .description("example")
///             .guardrail_arn("${test.guardrailArn}")
///             .skip_destroy(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Bedrock Guardrail Version using using a comma-delimited string of `guardrail_arn` and `version`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/guardrailVersion:GuardrailVersion example arn:aws:bedrock:us-west-2:123456789012:guardrail-id-12345678,1
/// ```
pub mod guardrail_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GuardrailVersionArgs {
        /// Description of the Guardrail version.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Guardrail ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub guardrail_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether to retain the old version of a previously deployed Guardrail. Default is `false`
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailVersionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct GuardrailVersionResult {
        /// Description of the Guardrail version.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Guardrail ARN.
        ///
        /// The following arguments are optional:
        pub guardrail_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to retain the old version of a previously deployed Guardrail. Default is `false`
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::GuardrailVersionTimeouts>,
        >,
        /// Guardrail version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GuardrailVersionArgs,
    ) -> GuardrailVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let guardrail_arn_binding = args.guardrail_arn.get_output(context).get_inner();
        let skip_destroy_binding = args.skip_destroy.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/guardrailVersion:GuardrailVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "guardrailArn".into(),
                    value: &guardrail_arn_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "guardrailArn".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GuardrailVersionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            guardrail_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guardrailArn").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
