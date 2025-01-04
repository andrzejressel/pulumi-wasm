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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceVoiceProfileDomainArgs {
        /// Description of Voice Profile Domain.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of Voice Profile Domain.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for server side encryption.
        #[builder(into)]
        pub server_side_encryption_configuration: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
        name: &str,
        args: SdkvoiceVoiceProfileDomainArgs,
    ) -> SdkvoiceVoiceProfileDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomain"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SdkvoiceVoiceProfileDomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_side_encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryptionConfiguration").unwrap(),
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
