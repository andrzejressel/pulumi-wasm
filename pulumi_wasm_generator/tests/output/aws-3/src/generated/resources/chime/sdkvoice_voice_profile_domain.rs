/// Resource for managing an AWS Chime SDK Voice Profile Domain.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: KMS Key for Voice Profile Domain
///       deletionWindowInDays: 7
///   exampleSdkvoiceVoiceProfileDomain:
///     type: aws:chime:SdkvoiceVoiceProfileDomain
///     name: example
///     properties:
///       name: ExampleVoiceProfileDomain
///       serverSideEncryptionConfiguration:
///         kmsKeyArn: ${example.arn}
///       description: My Voice Profile Domain
///       tags:
///         key1: value1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Chime SDK Voice Profile Domain using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomain example abcdef123456
/// ```
pub mod sdkvoice_voice_profile_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceVoiceProfileDomainArgs {
        /// Description of Voice Profile Domain.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of Voice Profile Domain.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration for server side encryption.
        #[builder(into)]
        pub server_side_encryption_configuration: pulumi_wasm_rust::InputOrOutput<
            super::super::types::chime::SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceVoiceProfileDomainResult {
        /// ARN of the Voice Profile Domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of Voice Profile Domain.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of Voice Profile Domain.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration for server side encryption.
        pub server_side_encryption_configuration: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration,
        >,
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
        args: SdkvoiceVoiceProfileDomainArgs,
    ) -> SdkvoiceVoiceProfileDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryptionConfiguration".into(),
                    value: &server_side_encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SdkvoiceVoiceProfileDomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            server_side_encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverSideEncryptionConfiguration"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
