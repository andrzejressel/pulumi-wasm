/// Resource for managing an AWS FinSpace Kx Environment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleKxEnvironment = kx_environment::create(
///         "exampleKxEnvironment",
///         KxEnvironmentArgs::builder()
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Transit Gateway Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleEnv = kx_environment::create(
///         "exampleEnv",
///         KxEnvironmentArgs::builder()
///             .custom_dns_configurations(
///                 vec![
///                     KxEnvironmentCustomDnsConfiguration::builder()
///                     .customDnsServerIp("10.0.0.76")
///                     .customDnsServerName("example.finspace.amazonaws.com")
///                     .build_struct(),
///                 ],
///             )
///             .description("Environment description")
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .transit_gateway_configuration(
///                 KxEnvironmentTransitGatewayConfiguration::builder()
///                     .routableCidrSpace("100.64.0.0/26")
///                     .transitGatewayId("${exampleTransitGateway.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().description("example").build_struct(),
///     );
/// }
/// ```
///
/// ### With Transit Gateway Attachment Network ACL Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleEnv = kx_environment::create(
///         "exampleEnv",
///         KxEnvironmentArgs::builder()
///             .custom_dns_configurations(
///                 vec![
///                     KxEnvironmentCustomDnsConfiguration::builder()
///                     .customDnsServerIp("10.0.0.76")
///                     .customDnsServerName("example.finspace.amazonaws.com")
///                     .build_struct(),
///                 ],
///             )
///             .description("Environment description")
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .transit_gateway_configuration(
///                 KxEnvironmentTransitGatewayConfiguration::builder()
///                     .attachmentNetworkAclConfigurations(
///                         vec![
///                             KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration::builder()
///                             .cidrBlock("0.0.0.0/0")
///                             .icmpTypeCode(KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationIcmpTypeCode::builder()
///                             .code(- 1). type (- 1).build_struct())
///                             .portRange(KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationPortRange::builder()
///                             .from(53).to(53).build_struct()).protocol("6")
///                             .ruleAction("allow").ruleNumber(1).build_struct(),
///                         ],
///                     )
///                     .routableCidrSpace("100.64.0.0/26")
///                     .transitGatewayId("${exampleTransitGateway.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().description("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Environment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxEnvironment:KxEnvironment example n3ceo7wqxoxcti5tujqwzs
/// ```
pub mod kx_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxEnvironmentArgs {
        /// List of DNS server name and server IP. This is used to set up Route-53 outbound resolvers. Defined below.
        #[builder(into, default)]
        pub custom_dns_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::finspace::KxEnvironmentCustomDnsConfiguration>,
            >,
        >,
        /// Description for the KX environment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS key ID to encrypt your data in the FinSpace environment.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Name of the KX environment that you want to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Transit gateway and network configuration that is used to connect the KX environment to an internal network. Defined below.
        #[builder(into, default)]
        pub transit_gateway_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::finspace::KxEnvironmentTransitGatewayConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct KxEnvironmentResult {
        /// Amazon Resource Name (ARN) identifier of the KX environment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS Availability Zone IDs that this environment is available in. Important when selecting VPC subnets to use in cluster creation.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Timestamp at which the environment is created in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub created_timestamp: pulumi_wasm_rust::Output<String>,
        /// List of DNS server name and server IP. This is used to set up Route-53 outbound resolvers. Defined below.
        pub custom_dns_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::finspace::KxEnvironmentCustomDnsConfiguration>,
            >,
        >,
        /// Description for the KX environment.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier for the AWS environment infrastructure account.
        pub infrastructure_account_id: pulumi_wasm_rust::Output<String>,
        /// KMS key ID to encrypt your data in the FinSpace environment.
        ///
        /// The following arguments are optional:
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Last timestamp at which the environment was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_wasm_rust::Output<String>,
        /// Name of the KX environment that you want to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Status of environment creation
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Transit gateway and network configuration that is used to connect the KX environment to an internal network. Defined below.
        pub transit_gateway_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::finspace::KxEnvironmentTransitGatewayConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KxEnvironmentArgs) -> KxEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_dns_configurations_binding = args
            .custom_dns_configurations
            .get_inner();
        let description_binding = args.description.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_configuration_binding = args
            .transit_gateway_configuration
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxEnvironment:KxEnvironment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customDnsConfigurations".into(),
                    value: &custom_dns_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayConfiguration".into(),
                    value: &transit_gateway_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "createdTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "customDnsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureAccountId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KxEnvironmentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            created_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTimestamp").unwrap(),
            ),
            custom_dns_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDnsConfigurations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            infrastructure_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureAccountId").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            last_modified_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTimestamp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayConfiguration").unwrap(),
            ),
        }
    }
}
