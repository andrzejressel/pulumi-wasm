/// Manages an Image Builder Infrastructure Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:imagebuilder:InfrastructureConfiguration
///     properties:
///       description: example description
///       instanceProfileName: ${exampleAwsIamInstanceProfile.name}
///       instanceTypes:
///         - t2.nano
///         - t3.micro
///       keyPair: ${exampleAwsKeyPair.keyName}
///       name: example
///       securityGroupIds:
///         - ${exampleAwsSecurityGroup.id}
///       snsTopicArn: ${exampleAwsSnsTopic.arn}
///       subnetId: ${main.id}
///       terminateInstanceOnFailure: true
///       logging:
///         s3Logs:
///           s3BucketName: ${exampleAwsS3Bucket.bucket}
///           s3KeyPrefix: logs
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_infrastructure_configuration` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/infrastructureConfiguration:InfrastructureConfiguration example arn:aws:imagebuilder:us-east-1:123456789012:infrastructure-configuration/example
/// ```
pub mod infrastructure_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InfrastructureConfigurationArgs {
        /// Description for the configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances. Detailed below.
        #[builder(into, default)]
        pub instance_metadata_options: pulumi_wasm_rust::Output<
            Option<
                super::super::types::imagebuilder::InfrastructureConfigurationInstanceMetadataOptions,
            >,
        >,
        /// Name of IAM Instance Profile.
        #[builder(into)]
        pub instance_profile_name: pulumi_wasm_rust::Output<String>,
        /// Set of EC2 Instance Types.
        #[builder(into, default)]
        pub instance_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of EC2 Key Pair.
        #[builder(into, default)]
        pub key_pair: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with logging settings. Detailed below.
        #[builder(into, default)]
        pub logging: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::InfrastructureConfigurationLogging>,
        >,
        /// Name for the configuration.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags to assign to infrastructure created by the configuration.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of EC2 Security Group identifiers.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of SNS Topic.
        #[builder(into, default)]
        pub sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// EC2 Subnet identifier. Also requires `security_group_ids` argument.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags to assign to the configuration. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable if the instance should be terminated when the pipeline fails. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instance_on_failure: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct InfrastructureConfigurationResult {
        /// Amazon Resource Name (ARN) of the configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date when the configuration was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Date when the configuration was updated.
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description for the configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances. Detailed below.
        pub instance_metadata_options: pulumi_wasm_rust::Output<
            Option<
                super::super::types::imagebuilder::InfrastructureConfigurationInstanceMetadataOptions,
            >,
        >,
        /// Name of IAM Instance Profile.
        pub instance_profile_name: pulumi_wasm_rust::Output<String>,
        /// Set of EC2 Instance Types.
        pub instance_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of EC2 Key Pair.
        pub key_pair: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with logging settings. Detailed below.
        pub logging: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::InfrastructureConfigurationLogging>,
        >,
        /// Name for the configuration.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags to assign to infrastructure created by the configuration.
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of EC2 Security Group identifiers.
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of SNS Topic.
        pub sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// EC2 Subnet identifier. Also requires `security_group_ids` argument.
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags to assign to the configuration. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable if the instance should be terminated when the pipeline fails. Defaults to `false`.
        pub terminate_instance_on_failure: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InfrastructureConfigurationArgs,
    ) -> InfrastructureConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let instance_metadata_options_binding = args
            .instance_metadata_options
            .get_inner();
        let instance_profile_name_binding = args.instance_profile_name.get_inner();
        let instance_types_binding = args.instance_types.get_inner();
        let key_pair_binding = args.key_pair.get_inner();
        let logging_binding = args.logging.get_inner();
        let name_binding = args.name.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let sns_topic_arn_binding = args.sns_topic_arn.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let terminate_instance_on_failure_binding = args
            .terminate_instance_on_failure
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/infrastructureConfiguration:InfrastructureConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMetadataOptions".into(),
                    value: &instance_metadata_options_binding,
                },
                register_interface::ObjectField {
                    name: "instanceProfileName".into(),
                    value: &instance_profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTypes".into(),
                    value: &instance_types_binding,
                },
                register_interface::ObjectField {
                    name: "keyPair".into(),
                    value: &key_pair_binding,
                },
                register_interface::ObjectField {
                    name: "logging".into(),
                    value: &logging_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminateInstanceOnFailure".into(),
                    value: &terminate_instance_on_failure_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "dateUpdated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "instanceMetadataOptions".into(),
                },
                register_interface::ResultField {
                    name: "instanceProfileName".into(),
                },
                register_interface::ResultField {
                    name: "instanceTypes".into(),
                },
                register_interface::ResultField {
                    name: "keyPair".into(),
                },
                register_interface::ResultField {
                    name: "logging".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "terminateInstanceOnFailure".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InfrastructureConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            date_updated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateUpdated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            instance_metadata_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceMetadataOptions").unwrap(),
            ),
            instance_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceProfileName").unwrap(),
            ),
            instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTypes").unwrap(),
            ),
            key_pair: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPair").unwrap(),
            ),
            logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logging").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArn").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            terminate_instance_on_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminateInstanceOnFailure").unwrap(),
            ),
        }
    }
}
