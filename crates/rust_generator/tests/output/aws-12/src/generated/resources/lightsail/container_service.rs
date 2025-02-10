/// An Amazon Lightsail container service is a highly scalable compute and networking resource on which you can deploy, run,
/// and manage containers. For more information, see
/// [Container services in Amazon Lightsail](https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-container-services).
///
/// > **Note:** For more information about the AWS Regions in which you can create Amazon Lightsail container services,
/// see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   myContainerService:
///     type: aws:lightsail:ContainerService
///     name: my_container_service
///     properties:
///       name: container-service-1
///       power: nano
///       scale: 1
///       isDisabled: false
///       tags:
///         foo1: bar1
///         foo2: ""
/// ```
///
/// ### Public Domain Names
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myContainerService = container_service::create(
///         "myContainerService",
///         ContainerServiceArgs::builder()
///             .public_domain_names(
///                 ContainerServicePublicDomainNames::builder()
///                     .certificates(
///                         vec![
///                             ContainerServicePublicDomainNamesCertificate::builder()
///                             .certificateName("example-certificate")
///                             .domainNames(vec!["www.example.com",]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Private Registry Access
///
/// ```yaml
/// resources:
///   defaultContainerService:
///     type: aws:lightsail:ContainerService
///     name: default
///     properties:
///       privateRegistryAccess:
///         ecrImagePullerRole:
///           isActive: true
///   defaultRepositoryPolicy:
///     type: aws:ecr:RepositoryPolicy
///     name: default
///     properties:
///       repository: ${defaultAwsEcrRepository.name}
///       policy: ${default.json}
/// variables:
///   default:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - ${defaultContainerService.privateRegistryAccess.ecrImagePullerRole.principalArn}
///             actions:
///               - ecr:BatchGetImage
///               - ecr:GetDownloadUrlForLayer
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Container Service using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/containerService:ContainerService my_container_service container-service-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod container_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerServiceArgs {
        /// A Boolean value indicating whether the container service is disabled. Defaults to `false`.
        #[builder(into, default)]
        pub is_disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name for the container service. Names must be of length 1 to 63, and be
        /// unique within each AWS Region in your Lightsail account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The power specification for the container service. The power specifies the amount of memory,
        /// the number of vCPUs, and the monthly price of each node of the container service.
        /// Possible values: `nano`, `micro`, `small`, `medium`, `large`, `xlarge`.
        #[builder(into)]
        pub power: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An object to describe the configuration for the container service to access private container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private repositories. See Private Registry Access below for more details.
        #[builder(into, default)]
        pub private_registry_access: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lightsail::ContainerServicePrivateRegistryAccess>,
        >,
        /// The public domain names to use with the container service, such as example.com
        /// and www.example.com. You can specify up to four public domain names for a container service. The domain names that you
        /// specify are used when you create a deployment with a container configured as the public endpoint of your container
        /// service. If you don't specify public domain names, then you can use the default domain of the container service.
        /// Defined below.
        #[builder(into, default)]
        pub public_domain_names: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lightsail::ContainerServicePublicDomainNames>,
        >,
        /// The scale specification for the container service. The scale specifies the allocated compute
        /// nodes of the container service.
        #[builder(into)]
        pub scale: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Map of container service tags. To create a key-only tag, use an empty string as the value. To tag at launch, specify the tags in the Launch Template. If
        /// configured with a provider
        /// `default_tags` configuration block
        /// present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContainerServiceResult {
        /// The Amazon Resource Name (ARN) of the container service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone. Follows the format us-east-2a (case-sensitive).
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A Boolean value indicating whether the container service is disabled. Defaults to `false`.
        pub is_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name for the container service. Names must be of length 1 to 63, and be
        /// unique within each AWS Region in your Lightsail account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The power specification for the container service. The power specifies the amount of memory,
        /// the number of vCPUs, and the monthly price of each node of the container service.
        /// Possible values: `nano`, `micro`, `small`, `medium`, `large`, `xlarge`.
        pub power: pulumi_gestalt_rust::Output<String>,
        /// The ID of the power of the container service.
        pub power_id: pulumi_gestalt_rust::Output<String>,
        /// The principal ARN of the container service. The principal ARN can be used to create a trust
        /// relationship between your standard AWS account and your Lightsail container service. This allows you to give your
        /// service permission to access resources in your standard AWS account.
        pub principal_arn: pulumi_gestalt_rust::Output<String>,
        /// The private domain name of the container service. The private domain name is accessible only
        /// by other resources within the default virtual private cloud (VPC) of your Lightsail account.
        pub private_domain_name: pulumi_gestalt_rust::Output<String>,
        /// An object to describe the configuration for the container service to access private container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private repositories. See Private Registry Access below for more details.
        pub private_registry_access: pulumi_gestalt_rust::Output<
            super::super::types::lightsail::ContainerServicePrivateRegistryAccess,
        >,
        /// The public domain names to use with the container service, such as example.com
        /// and www.example.com. You can specify up to four public domain names for a container service. The domain names that you
        /// specify are used when you create a deployment with a container configured as the public endpoint of your container
        /// service. If you don't specify public domain names, then you can use the default domain of the container service.
        /// Defined below.
        pub public_domain_names: pulumi_gestalt_rust::Output<
            Option<super::super::types::lightsail::ContainerServicePublicDomainNames>,
        >,
        /// The Lightsail resource type of the container service (i.e., ContainerService).
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// The scale specification for the container service. The scale specifies the allocated compute
        /// nodes of the container service.
        pub scale: pulumi_gestalt_rust::Output<i32>,
        /// The current state of the container service.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of container service tags. To create a key-only tag, use an empty string as the value. To tag at launch, specify the tags in the Launch Template. If
        /// configured with a provider
        /// `default_tags` configuration block
        /// present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider
        /// `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The publicly accessible URL of the container service. If no public endpoint is specified in the
        /// currentDeployment, this URL returns a 404 response.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerServiceArgs,
    ) -> ContainerServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let is_disabled_binding = args.is_disabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let power_binding = args.power.get_output(context);
        let private_registry_access_binding = args
            .private_registry_access
            .get_output(context);
        let public_domain_names_binding = args.public_domain_names.get_output(context);
        let scale_binding = args.scale.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/containerService:ContainerService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDisabled".into(),
                    value: is_disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "power".into(),
                    value: power_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateRegistryAccess".into(),
                    value: private_registry_access_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicDomainNames".into(),
                    value: public_domain_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scale".into(),
                    value: scale_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContainerServiceResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            created_at: o.get_field("createdAt"),
            is_disabled: o.get_field("isDisabled"),
            name: o.get_field("name"),
            power: o.get_field("power"),
            power_id: o.get_field("powerId"),
            principal_arn: o.get_field("principalArn"),
            private_domain_name: o.get_field("privateDomainName"),
            private_registry_access: o.get_field("privateRegistryAccess"),
            public_domain_names: o.get_field("publicDomainNames"),
            resource_type: o.get_field("resourceType"),
            scale: o.get_field("scale"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
        }
    }
}
