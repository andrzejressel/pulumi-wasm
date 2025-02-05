/// Associate an Amazon FSx file system with the FSx File Gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. This operation only supports the FSx File Gateway type.
///
/// [FSx File Gateway requirements](https://docs.aws.amazon.com/filegateway/latest/filefsxw/Requirements.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = file_system_association::create(
///         "example",
///         FileSystemAssociationArgs::builder()
///             .audit_destination_arn("${exampleAwsS3Bucket.arn}")
///             .gateway_arn("${exampleAwsStoragegatewayGateway.arn}")
///             .location_arn("${exampleAwsFsxWindowsFileSystem.arn}")
///             .password("avoid-plaintext-passwords")
///             .username("Admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Required Services Example
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${awsServiceStoragegatewayAmiFILES3Latest.value}
///       associatePublicIpAddress: true
///       instanceType: ${available.instanceType}
///       vpcSecurityGroupIds:
///         - ${testAwsSecurityGroup.id}
///       subnetId: ${testAwsSubnet[0].id}
///     options:
///       dependsOn:
///         - ${testAwsRoute}
///         - ${testAwsVpcDhcpOptionsAssociation}
///   testGateway:
///     type: aws:storagegateway:Gateway
///     name: test
///     properties:
///       gatewayIpAddress: ${test.publicIp}
///       gatewayName: test-sgw
///       gatewayTimezone: GMT
///       gatewayType: FILE_FSX_SMB
///       smbActiveDirectorySettings:
///         domainName: ${testAwsDirectoryServiceDirectory.name}
///         password: ${testAwsDirectoryServiceDirectory.password}
///         username: Admin
///   testWindowsFileSystem:
///     type: aws:fsx:WindowsFileSystem
///     name: test
///     properties:
///       activeDirectoryId: ${testAwsDirectoryServiceDirectory.id}
///       securityGroupIds:
///         - ${testAwsSecurityGroup.id}
///       skipFinalBackup: true
///       storageCapacity: 32
///       subnetIds:
///         - ${testAwsSubnet[0].id}
///       throughputCapacity: 8
///   fsx:
///     type: aws:storagegateway:FileSystemAssociation
///     properties:
///       gatewayArn: ${testGateway.arn}
///       locationArn: ${testWindowsFileSystem.arn}
///       username: Admin
///       password: ${testAwsDirectoryServiceDirectory.password}
///       cacheAttributes:
///         cacheStaleTimeoutInSeconds: 400
///       auditDestinationArn: ${testAwsCloudwatchLogGroup.arn}
/// variables:
///   awsServiceStoragegatewayAmiFILES3Latest:
///     fn::invoke:
///       function: aws:ssm:getParameter
///       arguments:
///         name: /aws/service/storagegateway/ami/FILE_S3/latest
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_file_system_association` using the FSx file system association Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/fileSystemAssociation:FileSystemAssociation example arn:aws:storagegateway:us-east-1:123456789012:fs-association/fsa-0DA347732FDB40125
/// ```
pub mod file_system_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FileSystemAssociationArgs {
        /// The Amazon Resource Name (ARN) of the storage used for the audit logs.
        #[builder(into, default)]
        pub audit_destination_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        #[builder(into, default)]
        pub cache_attributes: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::storagegateway::FileSystemAssociationCacheAttributes,
            >,
        >,
        /// The Amazon Resource Name (ARN) of the gateway.
        #[builder(into)]
        pub gateway_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Amazon FSx file system to associate with the FSx File Gateway.
        #[builder(into)]
        pub location_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The password of the user credential.
        #[builder(into)]
        pub password: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user name of the user credential that has permission to access the root share of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.
        #[builder(into)]
        pub username: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FileSystemAssociationResult {
        /// Amazon Resource Name (ARN) of the newly created file system association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the storage used for the audit logs.
        pub audit_destination_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Refresh cache information. see Cache Attributes for more details.
        pub cache_attributes: pulumi_wasm_rust::Output<
            Option<
                super::super::types::storagegateway::FileSystemAssociationCacheAttributes,
            >,
        >,
        /// The Amazon Resource Name (ARN) of the gateway.
        pub gateway_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon FSx file system to associate with the FSx File Gateway.
        pub location_arn: pulumi_wasm_rust::Output<String>,
        /// The password of the user credential.
        pub password: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user name of the user credential that has permission to access the root share of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FileSystemAssociationArgs,
    ) -> FileSystemAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_destination_arn_binding = args
            .audit_destination_arn
            .get_output(context)
            .get_inner();
        let cache_attributes_binding = args
            .cache_attributes
            .get_output(context)
            .get_inner();
        let gateway_arn_binding = args.gateway_arn.get_output(context).get_inner();
        let location_arn_binding = args.location_arn.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/fileSystemAssociation:FileSystemAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditDestinationArn".into(),
                    value: &audit_destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "cacheAttributes".into(),
                    value: &cache_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayArn".into(),
                    value: &gateway_arn_binding,
                },
                register_interface::ObjectField {
                    name: "locationArn".into(),
                    value: &location_arn_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FileSystemAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            audit_destination_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditDestinationArn"),
            ),
            cache_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cacheAttributes"),
            ),
            gateway_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayArn"),
            ),
            location_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationArn"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
