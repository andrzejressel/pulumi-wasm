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
///         foo2:
/// ```
///
/// ### Public Domain Names
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["ecr:BatchGetImage", "ecr:GetDownloadUrlForLayer",])
///                     .effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["${defaultContainerService.privateRegistryAccess.ecrImagePullerRole.principalArn}",])
///                     . type ("AWS").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let defaultContainerService = container_service::create(
///         "defaultContainerService",
///         ContainerServiceArgs::builder()
///             .private_registry_access(
///                 ContainerServicePrivateRegistryAccess::builder()
///                     .ecrImagePullerRole(
///                         ContainerServicePrivateRegistryAccessEcrImagePullerRole::builder()
///                             .isActive(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultRepositoryPolicy = repository_policy::create(
///         "defaultRepositoryPolicy",
///         RepositoryPolicyArgs::builder()
///             .policy("${default.json}")
///             .repository("${defaultAwsEcrRepository.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Container Service using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/containerService:ContainerService my_container_service container-service-1
/// ```
pub mod container_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerServiceArgs {
        /// A Boolean value indicating whether the container service is disabled. Defaults to `false`.
        #[builder(into, default)]
        pub is_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name for the container service. Names must be of length 1 to 63, and be
        /// unique within each AWS Region in your Lightsail account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The power specification for the container service. The power specifies the amount of memory,
        /// the number of vCPUs, and the monthly price of each node of the container service.
        /// Possible values: `nano`, `micro`, `small`, `medium`, `large`, `xlarge`.
        #[builder(into)]
        pub power: pulumi_wasm_rust::Output<String>,
        /// An object to describe the configuration for the container service to access private container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private repositories. See Private Registry Access below for more details.
        #[builder(into, default)]
        pub private_registry_access: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::ContainerServicePrivateRegistryAccess>,
        >,
        /// The public domain names to use with the container service, such as example.com
        /// and www.example.com. You can specify up to four public domain names for a container service. The domain names that you
        /// specify are used when you create a deployment with a container configured as the public endpoint of your container
        /// service. If you don't specify public domain names, then you can use the default domain of the container service.
        /// Defined below.
        #[builder(into, default)]
        pub public_domain_names: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::ContainerServicePublicDomainNames>,
        >,
        /// The scale specification for the container service. The scale specifies the allocated compute
        /// nodes of the container service.
        #[builder(into)]
        pub scale: pulumi_wasm_rust::Output<i32>,
        /// Map of container service tags. To create a key-only tag, use an empty string as the value. To tag at launch, specify the tags in the Launch Template. If
        /// configured with a provider
        /// `default_tags` configuration block
        /// present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContainerServiceResult {
        /// The Amazon Resource Name (ARN) of the container service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone. Follows the format us-east-2a (case-sensitive).
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A Boolean value indicating whether the container service is disabled. Defaults to `false`.
        pub is_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name for the container service. Names must be of length 1 to 63, and be
        /// unique within each AWS Region in your Lightsail account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The power specification for the container service. The power specifies the amount of memory,
        /// the number of vCPUs, and the monthly price of each node of the container service.
        /// Possible values: `nano`, `micro`, `small`, `medium`, `large`, `xlarge`.
        pub power: pulumi_wasm_rust::Output<String>,
        /// The ID of the power of the container service.
        pub power_id: pulumi_wasm_rust::Output<String>,
        /// The principal ARN of the container service. The principal ARN can be used to create a trust
        /// relationship between your standard AWS account and your Lightsail container service. This allows you to give your
        /// service permission to access resources in your standard AWS account.
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// The private domain name of the container service. The private domain name is accessible only
        /// by other resources within the default virtual private cloud (VPC) of your Lightsail account.
        pub private_domain_name: pulumi_wasm_rust::Output<String>,
        /// An object to describe the configuration for the container service to access private container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private repositories. See Private Registry Access below for more details.
        pub private_registry_access: pulumi_wasm_rust::Output<
            super::super::types::lightsail::ContainerServicePrivateRegistryAccess,
        >,
        /// The public domain names to use with the container service, such as example.com
        /// and www.example.com. You can specify up to four public domain names for a container service. The domain names that you
        /// specify are used when you create a deployment with a container configured as the public endpoint of your container
        /// service. If you don't specify public domain names, then you can use the default domain of the container service.
        /// Defined below.
        pub public_domain_names: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::ContainerServicePublicDomainNames>,
        >,
        /// The Lightsail resource type of the container service (i.e., ContainerService).
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The scale specification for the container service. The scale specifies the allocated compute
        /// nodes of the container service.
        pub scale: pulumi_wasm_rust::Output<i32>,
        /// The current state of the container service.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of container service tags. To create a key-only tag, use an empty string as the value. To tag at launch, specify the tags in the Launch Template. If
        /// configured with a provider
        /// `default_tags` configuration block
        /// present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider
        /// `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The publicly accessible URL of the container service. If no public endpoint is specified in the
        /// currentDeployment, this URL returns a 404 response.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContainerServiceArgs) -> ContainerServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let is_disabled_binding = args.is_disabled.get_inner();
        let name_binding = args.name.get_inner();
        let power_binding = args.power.get_inner();
        let private_registry_access_binding = args.private_registry_access.get_inner();
        let public_domain_names_binding = args.public_domain_names.get_inner();
        let scale_binding = args.scale.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/containerService:ContainerService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "isDisabled".into(),
                    value: &is_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "power".into(),
                    value: &power_binding,
                },
                register_interface::ObjectField {
                    name: "privateRegistryAccess".into(),
                    value: &private_registry_access_binding,
                },
                register_interface::ObjectField {
                    name: "publicDomainNames".into(),
                    value: &public_domain_names_binding,
                },
                register_interface::ObjectField {
                    name: "scale".into(),
                    value: &scale_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "isDisabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "power".into(),
                },
                register_interface::ResultField {
                    name: "powerId".into(),
                },
                register_interface::ResultField {
                    name: "principalArn".into(),
                },
                register_interface::ResultField {
                    name: "privateDomainName".into(),
                },
                register_interface::ResultField {
                    name: "privateRegistryAccess".into(),
                },
                register_interface::ResultField {
                    name: "publicDomainNames".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "scale".into(),
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
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            is_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDisabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            power: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("power").unwrap(),
            ),
            power_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("powerId").unwrap(),
            ),
            principal_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalArn").unwrap(),
            ),
            private_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDomainName").unwrap(),
            ),
            private_registry_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateRegistryAccess").unwrap(),
            ),
            public_domain_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDomainNames").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            scale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scale").unwrap(),
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
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}