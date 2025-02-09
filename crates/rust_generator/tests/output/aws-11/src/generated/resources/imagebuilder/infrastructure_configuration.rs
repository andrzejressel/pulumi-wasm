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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InfrastructureConfigurationArgs,
    ) -> InfrastructureConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_metadata_options_binding = args
            .instance_metadata_options
            .get_output(context);
        let instance_profile_name_binding = args
            .instance_profile_name
            .get_output(context);
        let instance_types_binding = args.instance_types.get_output(context);
        let key_pair_binding = args.key_pair.get_output(context);
        let logging_binding = args.logging.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_tags_binding = args.resource_tags.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let terminate_instance_on_failure_binding = args
            .terminate_instance_on_failure
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/infrastructureConfiguration:InfrastructureConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceMetadataOptions".into(),
                    value: instance_metadata_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceProfileName".into(),
                    value: instance_profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceTypes".into(),
                    value: instance_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyPair".into(),
                    value: key_pair_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logging".into(),
                    value: logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTags".into(),
                    value: resource_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsTopicArn".into(),
                    value: sns_topic_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terminateInstanceOnFailure".into(),
                    value: terminate_instance_on_failure_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InfrastructureConfigurationResult {
            arn: o.get_field("arn"),
            date_created: o.get_field("dateCreated"),
            date_updated: o.get_field("dateUpdated"),
            description: o.get_field("description"),
            instance_metadata_options: o.get_field("instanceMetadataOptions"),
            instance_profile_name: o.get_field("instanceProfileName"),
            instance_types: o.get_field("instanceTypes"),
            key_pair: o.get_field("keyPair"),
            logging: o.get_field("logging"),
            name: o.get_field("name"),
            resource_tags: o.get_field("resourceTags"),
            security_group_ids: o.get_field("securityGroupIds"),
            sns_topic_arn: o.get_field("snsTopicArn"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            terminate_instance_on_failure: o.get_field("terminateInstanceOnFailure"),
        }
    }
}
