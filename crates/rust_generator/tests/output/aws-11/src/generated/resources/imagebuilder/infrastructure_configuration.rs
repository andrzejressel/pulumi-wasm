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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod infrastructure_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InfrastructureConfigurationArgs {
        /// Description for the configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances. Detailed below.
        #[builder(into, default)]
        pub instance_metadata_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::imagebuilder::InfrastructureConfigurationInstanceMetadataOptions,
            >,
        >,
        /// Name of IAM Instance Profile.
        #[builder(into)]
        pub instance_profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of EC2 Instance Types.
        #[builder(into, default)]
        pub instance_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of EC2 Key Pair.
        #[builder(into, default)]
        pub key_pair: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with logging settings. Detailed below.
        #[builder(into, default)]
        pub logging: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::InfrastructureConfigurationLogging>,
        >,
        /// Name for the configuration.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags to assign to infrastructure created by the configuration.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of EC2 Security Group identifiers.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of SNS Topic.
        #[builder(into, default)]
        pub sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EC2 Subnet identifier. Also requires `security_group_ids` argument.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags to assign to the configuration. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable if the instance should be terminated when the pipeline fails. Defaults to `false`.
        #[builder(into, default)]
        pub terminate_instance_on_failure: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct InfrastructureConfigurationResult {
        /// Amazon Resource Name (ARN) of the configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date when the configuration was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Date when the configuration was updated.
        pub date_updated: pulumi_gestalt_rust::Output<String>,
        /// Description for the configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block with instance metadata options for the HTTP requests that pipeline builds use to launch EC2 build and test instances. Detailed below.
        pub instance_metadata_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::imagebuilder::InfrastructureConfigurationInstanceMetadataOptions,
            >,
        >,
        /// Name of IAM Instance Profile.
        pub instance_profile_name: pulumi_gestalt_rust::Output<String>,
        /// Set of EC2 Instance Types.
        pub instance_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of EC2 Key Pair.
        pub key_pair: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block with logging settings. Detailed below.
        pub logging: pulumi_gestalt_rust::Output<
            Option<super::super::types::imagebuilder::InfrastructureConfigurationLogging>,
        >,
        /// Name for the configuration.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags to assign to infrastructure created by the configuration.
        pub resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of EC2 Security Group identifiers.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Name (ARN) of SNS Topic.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// EC2 Subnet identifier. Also requires `security_group_ids` argument.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags to assign to the configuration. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable if the instance should be terminated when the pipeline fails. Defaults to `false`.
        pub terminate_instance_on_failure: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InfrastructureConfigurationArgs,
    ) -> InfrastructureConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let instance_metadata_options_binding_1 = args
            .instance_metadata_options
            .get_output(context);
        let instance_metadata_options_binding = instance_metadata_options_binding_1
            .get_inner();
        let instance_profile_name_binding_1 = args
            .instance_profile_name
            .get_output(context);
        let instance_profile_name_binding = instance_profile_name_binding_1.get_inner();
        let instance_types_binding_1 = args.instance_types.get_output(context);
        let instance_types_binding = instance_types_binding_1.get_inner();
        let key_pair_binding_1 = args.key_pair.get_output(context);
        let key_pair_binding = key_pair_binding_1.get_inner();
        let logging_binding_1 = args.logging.get_output(context);
        let logging_binding = logging_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_tags_binding_1 = args.resource_tags.get_output(context);
        let resource_tags_binding = resource_tags_binding_1.get_inner();
        let security_group_ids_binding_1 = args.security_group_ids.get_output(context);
        let security_group_ids_binding = security_group_ids_binding_1.get_inner();
        let sns_topic_arn_binding_1 = args.sns_topic_arn.get_output(context);
        let sns_topic_arn_binding = sns_topic_arn_binding_1.get_inner();
        let subnet_id_binding_1 = args.subnet_id.get_output(context);
        let subnet_id_binding = subnet_id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let terminate_instance_on_failure_binding_1 = args
            .terminate_instance_on_failure
            .get_output(context);
        let terminate_instance_on_failure_binding = terminate_instance_on_failure_binding_1
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        InfrastructureConfigurationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            date_updated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateUpdated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            instance_metadata_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceMetadataOptions"),
            ),
            instance_profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceProfileName"),
            ),
            instance_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceTypes"),
            ),
            key_pair: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyPair"),
            ),
            logging: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logging"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            sns_topic_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snsTopicArn"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            terminate_instance_on_failure: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("terminateInstanceOnFailure"),
            ),
        }
    }
}
