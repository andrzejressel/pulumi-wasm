/// Provides an AppConfig Configuration Profile resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:ConfigurationProfile
///     properties:
///       applicationId: ${exampleAwsAppconfigApplication.id}
///       description: Example Configuration Profile
///       name: example-configuration-profile-tf
///       locationUri: hosted
///       validators:
///         - content: ${exampleAwsLambdaFunction.arn}
///           type: LAMBDA
///       tags:
///         Type: AppConfig Configuration Profile
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Configuration Profiles using the configuration profile ID and application ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/configurationProfile:ConfigurationProfile example 71abcde:11xxxxx
/// ```
pub mod configuration_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationProfileArgs {
        /// Application ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the configuration profile. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
        #[builder(into)]
        pub location_uri: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name for the configuration profile. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
        #[builder(into, default)]
        pub retrieval_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set of methods for validating the configuration. Maximum of 2. See Validator below for more details.
        #[builder(into, default)]
        pub validators: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::appconfig::ConfigurationProfileValidator>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationProfileResult {
        /// Application ID. Must be between 4 and 7 characters in length.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the AppConfig Configuration Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration profile ID.
        pub configuration_profile_id: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration profile. Can be at most 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
        pub location_uri: pulumi_wasm_rust::Output<String>,
        /// Name for the configuration profile. Must be between 1 and 128 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
        pub retrieval_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of methods for validating the configuration. Maximum of 2. See Validator below for more details.
        pub validators: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfig::ConfigurationProfileValidator>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConfigurationProfileArgs,
    ) -> ConfigurationProfileResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kms_key_identifier_binding = args
            .kms_key_identifier
            .get_output(context)
            .get_inner();
        let location_uri_binding = args.location_uri.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let retrieval_role_arn_binding = args
            .retrieval_role_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let validators_binding = args.validators.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/configurationProfile:ConfigurationProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: &kms_key_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "locationUri".into(),
                    value: &location_uri_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "retrievalRoleArn".into(),
                    value: &retrieval_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "validators".into(),
                    value: &validators_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationProfileResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configuration_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationProfileId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kms_key_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyIdentifier"),
            ),
            location_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationUri"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            retrieval_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retrievalRoleArn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            validators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validators"),
            ),
        }
    }
}
