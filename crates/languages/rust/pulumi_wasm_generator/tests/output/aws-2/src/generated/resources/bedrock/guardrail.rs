/// Resource for managing an Amazon Bedrock Guardrail.
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
///     let example = guardrail::create(
///         "example",
///         GuardrailArgs::builder()
///             .blocked_input_messaging("example")
///             .blocked_outputs_messaging("example")
///             .content_policy_config(
///                 GuardrailContentPolicyConfig::builder()
///                     .filtersConfigs(
///                         vec![
///                             GuardrailContentPolicyConfigFiltersConfig::builder()
///                             .inputStrength("MEDIUM").outputStrength("MEDIUM"). type
///                             ("HATE").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .description("example")
///             .name("example")
///             .sensitive_information_policy_config(
///                 GuardrailSensitiveInformationPolicyConfig::builder()
///                     .piiEntitiesConfigs(
///                         vec![
///                             GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig::builder()
///                             .action("BLOCK"). type ("NAME").build_struct(),
///                         ],
///                     )
///                     .regexesConfigs(
///                         vec![
///                             GuardrailSensitiveInformationPolicyConfigRegexesConfig::builder()
///                             .action("BLOCK").description("example regex")
///                             .name("regex_example").pattern("^\\d{3}-\\d{2}-\\d{4}$")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .topic_policy_config(
///                 GuardrailTopicPolicyConfig::builder()
///                     .topicsConfigs(
///                         vec![
///                             GuardrailTopicPolicyConfigTopicsConfig::builder()
///                             .definition("Investment advice refers to inquiries, guidance, or recommendations regarding the management or allocation of funds or assets with the goal of generating returns .")
///                             .examples(vec!["Where should I invest my money ?",])
///                             .name("investment_topic"). type ("DENY").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .word_policy_config(
///                 GuardrailWordPolicyConfig::builder()
///                     .managedWordListsConfigs(
///                         vec![
///                             GuardrailWordPolicyConfigManagedWordListsConfig::builder().
///                             type ("PROFANITY").build_struct(),
///                         ],
///                     )
///                     .wordsConfigs(
///                         vec![
///                             GuardrailWordPolicyConfigWordsConfig::builder().text("HATE")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Bedrock Guardrail using using a comma-delimited string of `guardrail_id` and `version`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/guardrail:Guardrail example guardrail-id-12345678,DRAFT
/// ```
pub mod guardrail {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GuardrailArgs {
        /// Message to return when the guardrail blocks a prompt.
        #[builder(into)]
        pub blocked_input_messaging: pulumi_wasm_rust::InputOrOutput<String>,
        /// Message to return when the guardrail blocks a model response.
        #[builder(into)]
        pub blocked_outputs_messaging: pulumi_wasm_rust::InputOrOutput<String>,
        /// Content policy config for a guardrail. See Content Policy Config for more information.
        #[builder(into, default)]
        pub content_policy_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailContentPolicyConfig>,
        >,
        /// Contextual grounding policy config for a guardrail. See Contextual Grounding Policy Config for more information.
        #[builder(into, default)]
        pub contextual_grounding_policy_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::GuardrailContextualGroundingPolicyConfig,
            >,
        >,
        /// Description of the guardrail or its version.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The KMS key with which the guardrail was encrypted at rest.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the guardrail.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sensitive information policy config for a guardrail. See Sensitive Information Policy Config for more information.
        #[builder(into, default)]
        pub sensitive_information_policy_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfig,
            >,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailTimeouts>,
        >,
        /// Topic policy config for a guardrail. See Topic Policy Config for more information.
        #[builder(into, default)]
        pub topic_policy_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailTopicPolicyConfig>,
        >,
        /// Word policy config for a guardrail. See Word Policy Config for more information.
        #[builder(into, default)]
        pub word_policy_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bedrock::GuardrailWordPolicyConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct GuardrailResult {
        /// Message to return when the guardrail blocks a prompt.
        pub blocked_input_messaging: pulumi_wasm_rust::Output<String>,
        /// Message to return when the guardrail blocks a model response.
        pub blocked_outputs_messaging: pulumi_wasm_rust::Output<String>,
        /// Content policy config for a guardrail. See Content Policy Config for more information.
        pub content_policy_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::GuardrailContentPolicyConfig>,
        >,
        /// Contextual grounding policy config for a guardrail. See Contextual Grounding Policy Config for more information.
        pub contextual_grounding_policy_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::GuardrailContextualGroundingPolicyConfig,
            >,
        >,
        /// Unix epoch timestamp in seconds for when the Guardrail was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Description of the guardrail or its version.
        pub description: pulumi_wasm_rust::Output<String>,
        /// ARN of the Guardrail.
        pub guardrail_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Guardrail.
        pub guardrail_id: pulumi_wasm_rust::Output<String>,
        /// The KMS key with which the guardrail was encrypted at rest.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the guardrail.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Sensitive information policy config for a guardrail. See Sensitive Information Policy Config for more information.
        pub sensitive_information_policy_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfig,
            >,
        >,
        /// Status of the Bedrock Guardrail. One of `READY`, `FAILED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::GuardrailTimeouts>,
        >,
        /// Topic policy config for a guardrail. See Topic Policy Config for more information.
        pub topic_policy_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::GuardrailTopicPolicyConfig>,
        >,
        /// Version of the Guardrail.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Word policy config for a guardrail. See Word Policy Config for more information.
        pub word_policy_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::GuardrailWordPolicyConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GuardrailArgs,
    ) -> GuardrailResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blocked_input_messaging_binding = args
            .blocked_input_messaging
            .get_output(context)
            .get_inner();
        let blocked_outputs_messaging_binding = args
            .blocked_outputs_messaging
            .get_output(context)
            .get_inner();
        let content_policy_config_binding = args
            .content_policy_config
            .get_output(context)
            .get_inner();
        let contextual_grounding_policy_config_binding = args
            .contextual_grounding_policy_config
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sensitive_information_policy_config_binding = args
            .sensitive_information_policy_config
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let topic_policy_config_binding = args
            .topic_policy_config
            .get_output(context)
            .get_inner();
        let word_policy_config_binding = args
            .word_policy_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/guardrail:Guardrail".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockedInputMessaging".into(),
                    value: &blocked_input_messaging_binding,
                },
                register_interface::ObjectField {
                    name: "blockedOutputsMessaging".into(),
                    value: &blocked_outputs_messaging_binding,
                },
                register_interface::ObjectField {
                    name: "contentPolicyConfig".into(),
                    value: &content_policy_config_binding,
                },
                register_interface::ObjectField {
                    name: "contextualGroundingPolicyConfig".into(),
                    value: &contextual_grounding_policy_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sensitiveInformationPolicyConfig".into(),
                    value: &sensitive_information_policy_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "topicPolicyConfig".into(),
                    value: &topic_policy_config_binding,
                },
                register_interface::ObjectField {
                    name: "wordPolicyConfig".into(),
                    value: &word_policy_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GuardrailResult {
            blocked_input_messaging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blockedInputMessaging"),
            ),
            blocked_outputs_messaging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blockedOutputsMessaging"),
            ),
            content_policy_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentPolicyConfig"),
            ),
            contextual_grounding_policy_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contextualGroundingPolicyConfig"),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            guardrail_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guardrailArn"),
            ),
            guardrail_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("guardrailId"),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            sensitive_information_policy_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sensitiveInformationPolicyConfig"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            topic_policy_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("topicPolicyConfig"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            word_policy_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("wordPolicyConfig"),
            ),
        }
    }
}
