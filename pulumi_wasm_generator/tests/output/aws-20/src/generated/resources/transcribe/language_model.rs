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
pub mod language_model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LanguageModelArgs {
        /// Name of reference base model.
        #[builder(into)]
        pub base_model_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The input data config for the LanguageModel. See Input Data Config for more details.
        #[builder(into)]
        pub input_data_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::transcribe::LanguageModelInputDataConfig,
        >,
        /// The language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        #[builder(into)]
        pub language_code: pulumi_wasm_rust::InputOrOutput<String>,
        /// The model name.
        #[builder(into)]
        pub model_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LanguageModelResult {
        /// ARN of the LanguageModel.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of reference base model.
        pub base_model_name: pulumi_wasm_rust::Output<String>,
        /// The input data config for the LanguageModel. See Input Data Config for more details.
        pub input_data_config: pulumi_wasm_rust::Output<
            super::super::types::transcribe::LanguageModelInputDataConfig,
        >,
        /// The language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// The model name.
        pub model_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LanguageModelArgs,
    ) -> LanguageModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_model_name_binding = args
            .base_model_name
            .get_output(context)
            .get_inner();
        let input_data_config_binding = args
            .input_data_config
            .get_output(context)
            .get_inner();
        let language_code_binding = args.language_code.get_output(context).get_inner();
        let model_name_binding = args.model_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transcribe/languageModel:LanguageModel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baseModelName".into(),
                    value: &base_model_name_binding,
                },
                register_interface::ObjectField {
                    name: "inputDataConfig".into(),
                    value: &input_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "modelName".into(),
                    value: &model_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseModelName".into(),
                },
                register_interface::ResultField {
                    name: "inputDataConfig".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "modelName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LanguageModelResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_model_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseModelName").unwrap(),
            ),
            input_data_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputDataConfig").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            model_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
