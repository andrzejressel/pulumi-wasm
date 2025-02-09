/// Resource for managing an AWS Bedrock Guardrail Version.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod guardrail_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GuardrailVersionArgs {
        /// Description of the Guardrail version.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Guardrail ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub guardrail_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to retain the old version of a previously deployed Guardrail. Default is `false`
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailVersionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct GuardrailVersionResult {
        /// Description of the Guardrail version.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Guardrail ARN.
        ///
        /// The following arguments are optional:
        pub guardrail_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to retain the old version of a previously deployed Guardrail. Default is `false`
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::GuardrailVersionTimeouts>,
        >,
        /// Guardrail version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GuardrailVersionArgs,
    ) -> GuardrailVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let guardrail_arn_binding_1 = args.guardrail_arn.get_output(context);
        let guardrail_arn_binding = guardrail_arn_binding_1.get_inner();
        let skip_destroy_binding_1 = args.skip_destroy.get_output(context);
        let skip_destroy_binding = skip_destroy_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        GuardrailVersionResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            guardrail_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guardrailArn"),
            ),
            skip_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDestroy"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
