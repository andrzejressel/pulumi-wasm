/// Provides a SageMaker Domain resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["sagemaker.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleDomain = domain::create(
///         "exampleDomain",
///         DomainArgs::builder()
///             .auth_mode("IAM")
///             .default_user_settings(
///                 DomainDefaultUserSettings::builder()
///                     .executionRole("${exampleRole.arn}")
///                     .build_struct(),
///             )
///             .domain_name("example")
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
///     let exampleRole = role::create(
///         "exampleRole",
///         RoleArgs::builder()
///             .assume_role_policy("${example.json}")
///             .name("example")
///             .path("/")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Custom Images
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sagemaker:Image
///     properties:
///       imageName: example
///       roleArn: ${exampleAwsIamRole.arn}
///   exampleAppImageConfig:
///     type: aws:sagemaker:AppImageConfig
///     name: example
///     properties:
///       appImageConfigName: example
///       kernelGatewayImageConfig:
///         kernelSpecs:
///           - name: example
///   exampleImageVersion:
///     type: aws:sagemaker:ImageVersion
///     name: example
///     properties:
///       imageName: ${example.id}
///       baseImage: base-image
///   exampleDomain:
///     type: aws:sagemaker:Domain
///     name: example
///     properties:
///       domainName: example
///       authMode: IAM
///       vpcId: ${exampleAwsVpc.id}
///       subnetIds:
///         - ${exampleAwsSubnet.id}
///       defaultUserSettings:
///         executionRole: ${exampleAwsIamRole.arn}
///         kernelGatewayAppSettings:
///           customImages:
///             - appImageConfigName: ${exampleAppImageConfig.appImageConfigName}
///               imageName: ${exampleImageVersion.imageName}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Domains using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/domain:Domain test_domain d-8jgsjtilstu8
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Specifies the VPC used for non-EFS traffic. The default value is `PublicInternetOnly`. Valid values are `PublicInternetOnly` and `VpcOnly`.
        #[builder(into, default)]
        pub app_network_access_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The entity that creates and manages the required security groups for inter-app communication in `VPCOnly` mode. Valid values are `Service` and `Customer`.
        #[builder(into, default)]
        pub app_security_group_management: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode of authentication that members use to access the domain. Valid values are `IAM` and `SSO`.
        #[builder(into)]
        pub auth_mode: pulumi_wasm_rust::Output<String>,
        /// The default space settings. See `default_space_settings` Block below.
        #[builder(into, default)]
        pub default_space_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainDefaultSpaceSettings>,
        >,
        /// The default user settings. See `default_user_settings` Block below.
        #[builder(into)]
        pub default_user_settings: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DomainDefaultUserSettings,
        >,
        /// The domain name.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The domain settings. See `domain_settings` Block below.
        #[builder(into, default)]
        pub domain_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainDomainSettings>,
        >,
        /// The AWS KMS customer managed CMK used to encrypt the EFS volume attached to the domain.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The retention policy for this domain, which specifies whether resources will be retained after the Domain is deleted. By default, all resources are retained. See `retention_policy` Block below.
        #[builder(into, default)]
        pub retention_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainRetentionPolicy>,
        >,
        /// The VPC subnets that Studio uses for communication.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates whether custom tag propagation is supported for the domain. Defaults to `DISABLED`. Valid values are: `ENABLED` and `DISABLED`.
        #[builder(into, default)]
        pub tag_propagation: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Amazon Virtual Private Cloud (VPC) that Studio uses for communication.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// Specifies the VPC used for non-EFS traffic. The default value is `PublicInternetOnly`. Valid values are `PublicInternetOnly` and `VpcOnly`.
        pub app_network_access_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The entity that creates and manages the required security groups for inter-app communication in `VPCOnly` mode. Valid values are `Service` and `Customer`.
        pub app_security_group_management: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this Domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The mode of authentication that members use to access the domain. Valid values are `IAM` and `SSO`.
        pub auth_mode: pulumi_wasm_rust::Output<String>,
        /// The default space settings. See `default_space_settings` Block below.
        pub default_space_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainDefaultSpaceSettings>,
        >,
        /// The default user settings. See `default_user_settings` Block below.
        pub default_user_settings: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DomainDefaultUserSettings,
        >,
        /// The domain name.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The domain settings. See `domain_settings` Block below.
        pub domain_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainDomainSettings>,
        >,
        /// The ID of the Amazon Elastic File System (EFS) managed by this Domain.
        pub home_efs_file_system_id: pulumi_wasm_rust::Output<String>,
        /// The AWS KMS customer managed CMK used to encrypt the EFS volume attached to the domain.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The retention policy for this domain, which specifies whether resources will be retained after the Domain is deleted. By default, all resources are retained. See `retention_policy` Block below.
        pub retention_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::DomainRetentionPolicy>,
        >,
        /// The ID of the security group that authorizes traffic between the RSessionGateway apps and the RStudioServerPro app.
        pub security_group_id_for_domain_boundary: pulumi_wasm_rust::Output<String>,
        /// The ARN of the application managed by SageMaker in IAM Identity Center. This value is only returned for domains created after September 19, 2023.
        pub single_sign_on_application_arn: pulumi_wasm_rust::Output<String>,
        /// The SSO managed application instance ID.
        pub single_sign_on_managed_application_instance_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// The VPC subnets that Studio uses for communication.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates whether custom tag propagation is supported for the domain. Defaults to `DISABLED`. Valid values are: `ENABLED` and `DISABLED`.
        pub tag_propagation: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The domain's URL.
        pub url: pulumi_wasm_rust::Output<String>,
        /// The ID of the Amazon Virtual Private Cloud (VPC) that Studio uses for communication.
        ///
        /// The following arguments are optional:
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_network_access_type_binding = args.app_network_access_type.get_inner();
        let app_security_group_management_binding = args
            .app_security_group_management
            .get_inner();
        let auth_mode_binding = args.auth_mode.get_inner();
        let default_space_settings_binding = args.default_space_settings.get_inner();
        let default_user_settings_binding = args.default_user_settings.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let domain_settings_binding = args.domain_settings.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let retention_policy_binding = args.retention_policy.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tag_propagation_binding = args.tag_propagation.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/domain:Domain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appNetworkAccessType".into(),
                    value: &app_network_access_type_binding,
                },
                register_interface::ObjectField {
                    name: "appSecurityGroupManagement".into(),
                    value: &app_security_group_management_binding,
                },
                register_interface::ObjectField {
                    name: "authMode".into(),
                    value: &auth_mode_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSpaceSettings".into(),
                    value: &default_space_settings_binding,
                },
                register_interface::ObjectField {
                    name: "defaultUserSettings".into(),
                    value: &default_user_settings_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainSettings".into(),
                    value: &domain_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPolicy".into(),
                    value: &retention_policy_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tagPropagation".into(),
                    value: &tag_propagation_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appNetworkAccessType".into(),
                },
                register_interface::ResultField {
                    name: "appSecurityGroupManagement".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authMode".into(),
                },
                register_interface::ResultField {
                    name: "defaultSpaceSettings".into(),
                },
                register_interface::ResultField {
                    name: "defaultUserSettings".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainSettings".into(),
                },
                register_interface::ResultField {
                    name: "homeEfsFileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "retentionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIdForDomainBoundary".into(),
                },
                register_interface::ResultField {
                    name: "singleSignOnApplicationArn".into(),
                },
                register_interface::ResultField {
                    name: "singleSignOnManagedApplicationInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tagPropagation".into(),
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
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            app_network_access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appNetworkAccessType").unwrap(),
            ),
            app_security_group_management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSecurityGroupManagement").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authMode").unwrap(),
            ),
            default_space_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSpaceSettings").unwrap(),
            ),
            default_user_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultUserSettings").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainSettings").unwrap(),
            ),
            home_efs_file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homeEfsFileSystemId").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            retention_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPolicy").unwrap(),
            ),
            security_group_id_for_domain_boundary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIdForDomainBoundary").unwrap(),
            ),
            single_sign_on_application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleSignOnApplicationArn").unwrap(),
            ),
            single_sign_on_managed_application_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleSignOnManagedApplicationInstanceId").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tag_propagation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagPropagation").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("url").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}