/// Provides an AppConfig Deployment resource for an `aws.appconfig.Application` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:Deployment
///     properties:
///       applicationId: ${exampleAwsAppconfigApplication.id}
///       configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
///       configurationVersion: ${exampleAwsAppconfigHostedConfigurationVersion.versionNumber}
///       deploymentStrategyId: ${exampleAwsAppconfigDeploymentStrategy.id}
///       description: My example deployment
///       environmentId: ${exampleAwsAppconfigEnvironment.environmentId}
///       kmsKeyIdentifier: ${exampleAwsKmsKey.arn}
///       tags:
///         Type: AppConfig Deployment
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Deployments using the application ID, environment ID, and deployment number separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/deployment:Deployment example 71abcde/11xxxxx/1
/// ```
pub mod deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Application ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Configuration profile ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub configuration_profile_id: pulumi_wasm_rust::Output<String>,
        /// Configuration version to deploy. Can be at most 1024 characters.
        #[builder(into)]
        pub configuration_version: pulumi_wasm_rust::Output<String>,
        /// Deployment strategy ID or name of a predefined deployment strategy. See [Predefined Deployment Strategies](https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-creating-deployment-strategy.html#appconfig-creating-deployment-strategy-predefined) for more details.
        #[builder(into)]
        pub deployment_strategy_id: pulumi_wasm_rust::Output<String>,
        /// Description of the deployment. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Environment ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this to encrypt the configuration data using a customer managed key.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Application ID. Must be between 4 and 7 characters in length.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the AppConfig Deployment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration profile ID. Must be between 4 and 7 characters in length.
        pub configuration_profile_id: pulumi_wasm_rust::Output<String>,
        /// Configuration version to deploy. Can be at most 1024 characters.
        pub configuration_version: pulumi_wasm_rust::Output<String>,
        /// Deployment number.
        pub deployment_number: pulumi_wasm_rust::Output<i32>,
        /// Deployment strategy ID or name of a predefined deployment strategy. See [Predefined Deployment Strategies](https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-creating-deployment-strategy.html#appconfig-creating-deployment-strategy-predefined) for more details.
        pub deployment_strategy_id: pulumi_wasm_rust::Output<String>,
        /// Description of the deployment. Can be at most 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Environment ID. Must be between 4 and 7 characters in length.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the KMS key used to encrypt configuration data.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this to encrypt the configuration data using a customer managed key.
        pub kms_key_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the deployment.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeploymentArgs) -> DeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let configuration_profile_id_binding = args.configuration_profile_id.get_inner();
        let configuration_version_binding = args.configuration_version.get_inner();
        let deployment_strategy_id_binding = args.deployment_strategy_id.get_inner();
        let description_binding = args.description.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let kms_key_identifier_binding = args.kms_key_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/deployment:Deployment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationProfileId".into(),
                    value: &configuration_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationVersion".into(),
                    value: &configuration_version_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentStrategyId".into(),
                    value: &deployment_strategy_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: &kms_key_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configurationProfileId".into(),
                },
                register_interface::ResultField {
                    name: "configurationVersion".into(),
                },
                register_interface::ResultField {
                    name: "deploymentNumber".into(),
                },
                register_interface::ResultField {
                    name: "deploymentStrategyId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        DeploymentResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationProfileId").unwrap(),
            ),
            configuration_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationVersion").unwrap(),
            ),
            deployment_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentNumber").unwrap(),
            ),
            deployment_strategy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentStrategyId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            kms_key_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyIdentifier").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
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