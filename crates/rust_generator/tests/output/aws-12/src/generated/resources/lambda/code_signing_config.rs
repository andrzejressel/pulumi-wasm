/// Provides a Lambda Code Signing Config resource. A code signing configuration defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail).
///
/// For information about Lambda code signing configurations and how to use them, see [configuring code signing for Lambda functions](https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   newCsc:
///     type: aws:lambda:CodeSigningConfig
///     name: new_csc
///     properties:
///       allowedPublishers:
///         signingProfileVersionArns:
///           - ${example1.arn}
///           - ${example2.arn}
///       policies:
///         untrustedArtifactOnDeployment: Warn
///       description: My awesome code signing config.
///       tags:
///         Name: dynamodb
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Code Signing Configs using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/codeSigningConfig:CodeSigningConfig imported_csc arn:aws:lambda:us-west-2:123456789012:code-signing-config:csc-0f6c334abcdea4d8b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod code_signing_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeSigningConfigArgs {
        /// A configuration block of allowed publishers as signing profiles for this code signing configuration. Detailed below.
        #[builder(into)]
        pub allowed_publishers: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::lambda::CodeSigningConfigAllowedPublishers,
        >,
        /// Descriptive name for this code signing configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block of code signing policies that define the actions to take if the validation checks fail. Detailed below.
        #[builder(into, default)]
        pub policies: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lambda::CodeSigningConfigPolicies>,
        >,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CodeSigningConfigResult {
        /// A configuration block of allowed publishers as signing profiles for this code signing configuration. Detailed below.
        pub allowed_publishers: pulumi_gestalt_rust::Output<
            super::super::types::lambda::CodeSigningConfigAllowedPublishers,
        >,
        /// The Amazon Resource Name (ARN) of the code signing configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the code signing configuration.
        pub config_id: pulumi_gestalt_rust::Output<String>,
        /// Descriptive name for this code signing configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date and time that the code signing configuration was last modified.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// A configuration block of code signing policies that define the actions to take if the validation checks fail. Detailed below.
        pub policies: pulumi_gestalt_rust::Output<
            super::super::types::lambda::CodeSigningConfigPolicies,
        >,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: CodeSigningConfigArgs,
    ) -> CodeSigningConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_publishers_binding = args.allowed_publishers.get_output(context);
        let description_binding = args.description.get_output(context);
        let policies_binding = args.policies.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lambda/codeSigningConfig:CodeSigningConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedPublishers".into(),
                    value: allowed_publishers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CodeSigningConfigResult {
            allowed_publishers: o.get_field("allowedPublishers"),
            arn: o.get_field("arn"),
            config_id: o.get_field("configId"),
            description: o.get_field("description"),
            last_modified: o.get_field("lastModified"),
            policies: o.get_field("policies"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
