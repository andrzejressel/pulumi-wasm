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
pub mod code_signing_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeSigningConfigArgs {
        /// A configuration block of allowed publishers as signing profiles for this code signing configuration. Detailed below.
        #[builder(into)]
        pub allowed_publishers: pulumi_wasm_rust::Output<
            super::super::types::lambda::CodeSigningConfigAllowedPublishers,
        >,
        /// Descriptive name for this code signing configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A configuration block of code signing policies that define the actions to take if the validation checks fail. Detailed below.
        #[builder(into, default)]
        pub policies: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::CodeSigningConfigPolicies>,
        >,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CodeSigningConfigResult {
        /// A configuration block of allowed publishers as signing profiles for this code signing configuration. Detailed below.
        pub allowed_publishers: pulumi_wasm_rust::Output<
            super::super::types::lambda::CodeSigningConfigAllowedPublishers,
        >,
        /// The Amazon Resource Name (ARN) of the code signing configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the code signing configuration.
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// Descriptive name for this code signing configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The date and time that the code signing configuration was last modified.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// A configuration block of code signing policies that define the actions to take if the validation checks fail. Detailed below.
        pub policies: pulumi_wasm_rust::Output<
            super::super::types::lambda::CodeSigningConfigPolicies,
        >,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CodeSigningConfigArgs) -> CodeSigningConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_publishers_binding = args.allowed_publishers.get_inner();
        let description_binding = args.description.get_inner();
        let policies_binding = args.policies.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/codeSigningConfig:CodeSigningConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedPublishers".into(),
                    value: &allowed_publishers_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "policies".into(),
                    value: &policies_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowedPublishers".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "policies".into(),
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
        CodeSigningConfigResult {
            allowed_publishers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedPublishers").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            config_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policies").unwrap(),
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