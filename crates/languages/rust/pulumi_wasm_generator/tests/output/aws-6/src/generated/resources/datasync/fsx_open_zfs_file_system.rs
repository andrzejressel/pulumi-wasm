/// Manages an AWS DataSync FSx OpenZfs Location.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fsx_open_zfs_file_system::create(
///         "example",
///         FsxOpenZfsFileSystemArgs::builder()
///             .fsx_filesystem_arn("${exampleAwsFsxOpenzfsFileSystem.arn}")
///             .protocol(
///                 FsxOpenZfsFileSystemProtocol::builder()
///                     .nfs(
///                         FsxOpenZfsFileSystemProtocolNfs::builder()
///                             .mountOptions(
///                                 FsxOpenZfsFileSystemProtocolNfsMountOptions::builder()
///                                     .version("AUTOMATIC")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .security_group_arns(vec!["${exampleAwsSecurityGroup.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_fsx_openzfs_file_system` using the `DataSync-ARN#FSx-openzfs-ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:datasync/fsxOpenZfsFileSystem:FsxOpenZfsFileSystem example arn:aws:datasync:us-west-2:123456789012:location/loc-12345678901234567#arn:aws:fsx:us-west-2:123456789012:file-system/fs-08e04cd442c1bb94a
/// ```
pub mod fsx_open_zfs_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FsxOpenZfsFileSystemArgs {
        /// The Amazon Resource Name (ARN) for the FSx for OpenZfs file system.
        #[builder(into)]
        pub fsx_filesystem_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of protocol that DataSync uses to access your file system. See below.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<
            super::super::types::datasync::FsxOpenZfsFileSystemProtocol,
        >,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for openzfs file system.
        #[builder(into)]
        pub security_group_arns: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Subdirectory to perform actions as source or destination. Must start with `/fsx`.
        #[builder(into, default)]
        pub subdirectory: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FsxOpenZfsFileSystemResult {
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The time that the FSx for openzfs location was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the FSx for OpenZfs file system.
        pub fsx_filesystem_arn: pulumi_wasm_rust::Output<String>,
        /// The type of protocol that DataSync uses to access your file system. See below.
        pub protocol: pulumi_wasm_rust::Output<
            super::super::types::datasync::FsxOpenZfsFileSystemProtocol,
        >,
        /// The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for openzfs file system.
        pub security_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subdirectory to perform actions as source or destination. Must start with `/fsx`.
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the FSx for openzfs location that was described.
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FsxOpenZfsFileSystemArgs,
    ) -> FsxOpenZfsFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fsx_filesystem_arn_binding = args
            .fsx_filesystem_arn
            .get_output(context)
            .get_inner();
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let security_group_arns_binding = args
            .security_group_arns
            .get_output(context)
            .get_inner();
        let subdirectory_binding = args.subdirectory.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/fsxOpenZfsFileSystem:FsxOpenZfsFileSystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fsxFilesystemArn".into(),
                    value: &fsx_filesystem_arn_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupArns".into(),
                    value: &security_group_arns_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        FsxOpenZfsFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            fsx_filesystem_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fsxFilesystemArn"),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            security_group_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityGroupArns"),
            ),
            subdirectory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subdirectory"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("uri")),
        }
    }
}
