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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sdkvoice_voice_profile_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceVoiceProfileDomainArgs {
        /// Description of Voice Profile Domain.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of Voice Profile Domain.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for server side encryption.
        #[builder(into)]
        pub server_side_encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::chime::SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceVoiceProfileDomainResult {
        /// ARN of the Voice Profile Domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of Voice Profile Domain.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of Voice Profile Domain.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for server side encryption.
        pub server_side_encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::chime::SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration,
        >,
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
        args: SdkvoiceVoiceProfileDomainArgs,
    ) -> SdkvoiceVoiceProfileDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverSideEncryptionConfiguration".into(),
                    value: server_side_encryption_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SdkvoiceVoiceProfileDomainResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            server_side_encryption_configuration: o
                .get_field("serverSideEncryptionConfiguration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
