/// Provides a SageMaker Notebook Instance resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```yaml
/// resources:
///   ni:
///     type: aws:sagemaker:NotebookInstance
///     properties:
///       name: my-notebook-instance
///       roleArn: ${role.arn}
///       instanceType: ml.t2.medium
///       tags:
///         Name: foo
/// ```
///
/// ### Code repository usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:CodeRepository
///     properties:
///       codeRepositoryName: my-notebook-instance-code-repo
///       gitConfig:
///         repositoryUrl: https://github.com/github/docs.git
///   ni:
///     type: aws:sagemaker:NotebookInstance
///     properties:
///       name: my-notebook-instance
///       roleArn: ${role.arn}
///       instanceType: ml.t2.medium
///       defaultCodeRepository: ${example.codeRepositoryName}
///       tags:
///         Name: foo
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Notebook Instances using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/notebookInstance:NotebookInstance test_notebook_instance my-notebook-instance
/// ```
pub mod notebook_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotebookInstanceArgs {
        /// A list of Elastic Inference (EI) instance types to associate with this notebook instance. See [Elastic Inference Accelerator](https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html) for more details. Valid values: `ml.eia1.medium`, `ml.eia1.large`, `ml.eia1.xlarge`, `ml.eia2.medium`, `ml.eia2.large`, `ml.eia2.xlarge`.
        #[builder(into, default)]
        pub accelerator_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An array of up to three Git repositories to associate with the notebook instance.
        /// These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in [AWS CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance.
        #[builder(into, default)]
        pub additional_code_repositories: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in [AWS CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or in any other Git repository.
        #[builder(into, default)]
        pub default_code_repository: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to `Disabled` to disable internet access to notebook. Requires `security_groups` and `subnet_id` to be set. Supported values: `Enabled` (Default) or `Disabled`. If set to `Disabled`, the notebook instance will be able to access resources only in your VPC, and will not be able to connect to Amazon SageMaker training and endpoint services unless your configure a NAT Gateway in your VPC.
        #[builder(into, default)]
        pub direct_internet_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information on the IMDS configuration of the notebook instance. Conflicts with `instance_metadata_service_configuration`. see details below.
        #[builder(into, default)]
        pub instance_metadata_service_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::sagemaker::NotebookInstanceInstanceMetadataServiceConfiguration,
            >,
        >,
        /// The name of ML compute instance type.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a lifecycle configuration to associate with the notebook instance.
        #[builder(into, default)]
        pub lifecycle_config_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the notebook instance (must be unique).
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The platform identifier of the notebook instance runtime environment. This value can be either `notebook-al1-v1`, `notebook-al2-v1`, `notebook-al2-v2`, or `notebook-al2-v3`, depending on which version of Amazon Linux you require.
        #[builder(into, default)]
        pub platform_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the IAM role to be used by the notebook instance which allows SageMaker to call other services on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether root access is `Enabled` or `Disabled` for users of the notebook instance. The default value is `Enabled`.
        #[builder(into, default)]
        pub root_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The associated security groups.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The VPC subnet ID.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.
        #[builder(into, default)]
        pub volume_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct NotebookInstanceResult {
        /// A list of Elastic Inference (EI) instance types to associate with this notebook instance. See [Elastic Inference Accelerator](https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html) for more details. Valid values: `ml.eia1.medium`, `ml.eia1.large`, `ml.eia1.xlarge`, `ml.eia2.medium`, `ml.eia2.large`, `ml.eia2.xlarge`.
        pub accelerator_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An array of up to three Git repositories to associate with the notebook instance.
        /// These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in [AWS CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance.
        pub additional_code_repositories: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Amazon Resource Name (ARN) assigned by AWS to this notebook instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in [AWS CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or in any other Git repository.
        pub default_code_repository: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set to `Disabled` to disable internet access to notebook. Requires `security_groups` and `subnet_id` to be set. Supported values: `Enabled` (Default) or `Disabled`. If set to `Disabled`, the notebook instance will be able to access resources only in your VPC, and will not be able to connect to Amazon SageMaker training and endpoint services unless your configure a NAT Gateway in your VPC.
        pub direct_internet_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// Information on the IMDS configuration of the notebook instance. Conflicts with `instance_metadata_service_configuration`. see details below.
        pub instance_metadata_service_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::sagemaker::NotebookInstanceInstanceMetadataServiceConfiguration,
            >,
        >,
        /// The name of ML compute instance type.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of a lifecycle configuration to associate with the notebook instance.
        pub lifecycle_config_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the notebook instance (must be unique).
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network interface ID that Amazon SageMaker created at the time of creating the instance. Only available when setting `subnet_id`.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The platform identifier of the notebook instance runtime environment. This value can be either `notebook-al1-v1`, `notebook-al2-v1`, `notebook-al2-v2`, or `notebook-al2-v3`, depending on which version of Amazon Linux you require.
        pub platform_identifier: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role to be used by the notebook instance which allows SageMaker to call other services on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether root access is `Enabled` or `Disabled` for users of the notebook instance. The default value is `Enabled`.
        pub root_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// The associated security groups.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The VPC subnet ID.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL that you use to connect to the Jupyter notebook that is running in your notebook instance.
        pub url: pulumi_gestalt_rust::Output<String>,
        /// The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.
        pub volume_size: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NotebookInstanceArgs,
    ) -> NotebookInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accelerator_types_binding = args
            .accelerator_types
            .get_output(context)
            .get_inner();
        let additional_code_repositories_binding = args
            .additional_code_repositories
            .get_output(context)
            .get_inner();
        let default_code_repository_binding = args
            .default_code_repository
            .get_output(context)
            .get_inner();
        let direct_internet_access_binding = args
            .direct_internet_access
            .get_output(context)
            .get_inner();
        let instance_metadata_service_configuration_binding = args
            .instance_metadata_service_configuration
            .get_output(context)
            .get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let lifecycle_config_name_binding = args
            .lifecycle_config_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let platform_identifier_binding = args
            .platform_identifier
            .get_output(context)
            .get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let root_access_binding = args.root_access.get_output(context).get_inner();
        let security_groups_binding = args
            .security_groups
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let volume_size_binding = args.volume_size.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/notebookInstance:NotebookInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorTypes".into(),
                    value: &accelerator_types_binding,
                },
                register_interface::ObjectField {
                    name: "additionalCodeRepositories".into(),
                    value: &additional_code_repositories_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCodeRepository".into(),
                    value: &default_code_repository_binding,
                },
                register_interface::ObjectField {
                    name: "directInternetAccess".into(),
                    value: &direct_internet_access_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMetadataServiceConfiguration".into(),
                    value: &instance_metadata_service_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "lifecycleConfigName".into(),
                    value: &lifecycle_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platformIdentifier".into(),
                    value: &platform_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "rootAccess".into(),
                    value: &root_access_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
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
                    name: "volumeSize".into(),
                    value: &volume_size_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NotebookInstanceResult {
            accelerator_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratorTypes"),
            ),
            additional_code_repositories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalCodeRepositories"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            default_code_repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultCodeRepository"),
            ),
            direct_internet_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directInternetAccess"),
            ),
            instance_metadata_service_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceMetadataServiceConfiguration"),
            ),
            instance_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            lifecycle_config_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifecycleConfigName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            platform_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformIdentifier"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            root_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootAccess"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
            volume_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeSize"),
            ),
        }
    }
}
