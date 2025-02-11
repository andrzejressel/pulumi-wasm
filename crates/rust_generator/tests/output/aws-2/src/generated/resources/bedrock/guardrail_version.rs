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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GuardrailVersionArgs,
    ) -> GuardrailVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let guardrail_arn_binding = args.guardrail_arn.get_output(context);
        let skip_destroy_binding = args.skip_destroy.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/guardrailVersion:GuardrailVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guardrailArn".into(),
                    value: &guardrail_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GuardrailVersionResult {
            description: o.get_field("description"),
            guardrail_arn: o.get_field("guardrailArn"),
            skip_destroy: o.get_field("skipDestroy"),
            timeouts: o.get_field("timeouts"),
            version: o.get_field("version"),
        }
    }
}
