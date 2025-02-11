/// Resource for managing an AWS Transcribe LanguageModel.
///
/// > This resource can take a significant amount of time to provision. See Language Model [FAQ](https://aws.amazon.com/transcribe/faqs/) for more details.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${example.json}
///   testPolicy:
///     type: aws:iam:RolePolicy
///     name: test_policy
///     properties:
///       name: example
///       role: ${exampleRole.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - s3:GetObject
///                 - s3:ListBucket
///               Effect: Allow
///               Resource:
///                 - '*'
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example-transcribe
///       forceDestroy: true
///   object:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${exampleBucketV2.id}
///       key: transcribe/test1.txt
///       source:
///         fn::FileAsset: test1.txt
///   exampleLanguageModel:
///     type: aws:transcribe:LanguageModel
///     name: example
///     properties:
///       modelName: example
///       baseModelName: NarrowBand
///       inputDataConfig:
///         dataAccessRoleArn: ${exampleRole.arn}
///         s3Uri: s3://${exampleBucketV2.id}/transcribe/
///       languageCode: en-US
///       tags:
///         ENVIRONMENT: development
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - transcribe.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transcribe LanguageModel using the `model_name`. For example:
///
/// ```sh
/// $ pulumi import aws:transcribe/languageModel:LanguageModel example example-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod language_model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LanguageModelArgs {
        /// Name of reference base model.
        #[builder(into)]
        pub base_model_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The input data config for the LanguageModel. See Input Data Config for more details.
        #[builder(into)]
        pub input_data_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::transcribe::LanguageModelInputDataConfig,
        >,
        /// The language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        #[builder(into)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The model name.
        #[builder(into)]
        pub model_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LanguageModelResult {
        /// ARN of the LanguageModel.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of reference base model.
        pub base_model_name: pulumi_gestalt_rust::Output<String>,
        /// The input data config for the LanguageModel. See Input Data Config for more details.
        pub input_data_config: pulumi_gestalt_rust::Output<
            super::super::types::transcribe::LanguageModelInputDataConfig,
        >,
        /// The language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// The model name.
        pub model_name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LanguageModelArgs,
    ) -> LanguageModelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_model_name_binding = args.base_model_name.get_output(context);
        let input_data_config_binding = args.input_data_config.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let model_name_binding = args.model_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transcribe/languageModel:LanguageModel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseModelName".into(),
                    value: &base_model_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputDataConfig".into(),
                    value: &input_data_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelName".into(),
                    value: &model_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LanguageModelResult {
            arn: o.get_field("arn"),
            base_model_name: o.get_field("baseModelName"),
            input_data_config: o.get_field("inputDataConfig"),
            language_code: o.get_field("languageCode"),
            model_name: o.get_field("modelName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
