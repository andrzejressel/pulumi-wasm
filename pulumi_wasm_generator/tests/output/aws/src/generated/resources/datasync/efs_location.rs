/// Manages an AWS DataSync EFS Location.
///
/// > **NOTE:** The EFS File System must have a mounted EFS Mount Target before creating this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = efs_location::create(
///         "example",
///         EfsLocationArgs::builder()
///             .ec_2_config(
///                 EfsLocationEc2Config::builder()
///                     .securityGroupArns(vec!["${exampleAwsSecurityGroup.arn}",])
///                     .subnetArn("${exampleAwsSubnet.arn}")
///                     .build_struct(),
///             )
///             .efs_file_system_arn("${exampleAwsEfsMountTarget.fileSystemArn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_efs` using the DataSync Task Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/efsLocation:EfsLocation example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
pub mod efs_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EfsLocationArgs {
        /// Specifies the Amazon Resource Name (ARN) of the access point that DataSync uses to access the Amazon EFS file system.
        #[builder(into, default)]
        pub access_point_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block containing EC2 configurations for connecting to the EFS File System.
        #[builder(into)]
        pub ec2_config: pulumi_wasm_rust::Output<
            super::super::types::datasync::EfsLocationEc2Config,
        >,
        /// Amazon Resource Name (ARN) of EFS File System.
        #[builder(into)]
        pub efs_file_system_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies an Identity and Access Management (IAM) role that DataSync assumes when mounting the Amazon EFS file system.
        #[builder(into, default)]
        pub file_system_access_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether you want DataSync to use TLS encryption when transferring data to or from your Amazon EFS file system. Valid values are `NONE` and `TLS1_2`.
        #[builder(into, default)]
        pub in_transit_encryption: pulumi_wasm_rust::Output<Option<String>>,
        /// Subdirectory to perform actions as source or destination. Default `/`.
        #[builder(into, default)]
        pub subdirectory: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EfsLocationResult {
        /// Specifies the Amazon Resource Name (ARN) of the access point that DataSync uses to access the Amazon EFS file system.
        pub access_point_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block containing EC2 configurations for connecting to the EFS File System.
        pub ec2_config: pulumi_wasm_rust::Output<
            super::super::types::datasync::EfsLocationEc2Config,
        >,
        /// Amazon Resource Name (ARN) of EFS File System.
        pub efs_file_system_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies an Identity and Access Management (IAM) role that DataSync assumes when mounting the Amazon EFS file system.
        pub file_system_access_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether you want DataSync to use TLS encryption when transferring data to or from your Amazon EFS file system. Valid values are `NONE` and `TLS1_2`.
        pub in_transit_encryption: pulumi_wasm_rust::Output<Option<String>>,
        /// Subdirectory to perform actions as source or destination. Default `/`.
        pub subdirectory: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EfsLocationArgs) -> EfsLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_point_arn_binding = args.access_point_arn.get_inner();
        let ec2_config_binding = args.ec2_config.get_inner();
        let efs_file_system_arn_binding = args.efs_file_system_arn.get_inner();
        let file_system_access_role_arn_binding = args
            .file_system_access_role_arn
            .get_inner();
        let in_transit_encryption_binding = args.in_transit_encryption.get_inner();
        let subdirectory_binding = args.subdirectory.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/efsLocation:EfsLocation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPointArn".into(),
                    value: &access_point_arn_binding,
                },
                register_interface::ObjectField {
                    name: "ec2Config".into(),
                    value: &ec2_config_binding,
                },
                register_interface::ObjectField {
                    name: "efsFileSystemArn".into(),
                    value: &efs_file_system_arn_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemAccessRoleArn".into(),
                    value: &file_system_access_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "inTransitEncryption".into(),
                    value: &in_transit_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "subdirectory".into(),
                    value: &subdirectory_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPointArn".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ec2Config".into(),
                },
                register_interface::ResultField {
                    name: "efsFileSystemArn".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemAccessRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "inTransitEncryption".into(),
                },
                register_interface::ResultField {
                    name: "subdirectory".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EfsLocationResult {
            access_point_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPointArn").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            ec2_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2Config").unwrap(),
            ),
            efs_file_system_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("efsFileSystemArn").unwrap(),
            ),
            file_system_access_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemAccessRoleArn").unwrap(),
            ),
            in_transit_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inTransitEncryption").unwrap(),
            ),
            subdirectory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subdirectory").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uri").unwrap()),
        }
    }
}