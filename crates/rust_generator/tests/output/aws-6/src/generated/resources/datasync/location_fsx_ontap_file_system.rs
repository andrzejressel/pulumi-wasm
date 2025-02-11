/// Resource for managing an AWS DataSync Location FSx Ontap File System.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:datasync:LocationFsxOntapFileSystem
///     properties:
///       fsxFilesystemArn: ${testAwsFsxOntapFileSystem.arn}
///       securityGroupArns:
///         - ${testAwsSecurityGroup.arn}
///       storageVirtualMachineArn: ${testAwsFsxOntapStorageVirtualMachine.arn}
///       protocol:
///         nfs:
///           mountOptions:
///             version: NFS3
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_fsx_ontap_file_system` using the `DataSync-ARN#FSx-ontap-svm-ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationFsxOntapFileSystem:LocationFsxOntapFileSystem example arn:aws:datasync:us-west-2:123456789012:location/loc-12345678901234567#arn:aws:fsx:us-west-2:123456789012:storage-virtual-machine/svm-12345678abcdef123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_fsx_ontap_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationFsxOntapFileSystemArgs {
        /// The data transfer protocol that DataSync uses to access your Amazon FSx file system. See Protocol below.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datasync::LocationFsxOntapFileSystemProtocol,
        >,
        /// The security groups that provide access to your file system's preferred subnet. The security groups must allow outbbound traffic on the following ports (depending on the protocol you use):
        /// * Network File System (NFS): TCP ports 111, 635, and 2049
        /// * Server Message Block (SMB): TCP port 445
        #[builder(into)]
        pub security_group_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ARN of the SVM in your file system where you want to copy data to of from.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub storage_virtual_machine_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Path to the file share in the SVM where you'll copy your data. You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares) (e.g. `/vol1`, `/vol1/tree1`, `share1`).
        #[builder(into, default)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationFsxOntapFileSystemResult {
        /// ARN of the DataSync Location for the FSx Ontap File System.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// ARN of the FSx Ontap File System.
        pub fsx_filesystem_arn: pulumi_gestalt_rust::Output<String>,
        /// The data transfer protocol that DataSync uses to access your Amazon FSx file system. See Protocol below.
        pub protocol: pulumi_gestalt_rust::Output<
            super::super::types::datasync::LocationFsxOntapFileSystemProtocol,
        >,
        /// The security groups that provide access to your file system's preferred subnet. The security groups must allow outbbound traffic on the following ports (depending on the protocol you use):
        /// * Network File System (NFS): TCP ports 111, 635, and 2049
        /// * Server Message Block (SMB): TCP port 445
        pub security_group_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ARN of the SVM in your file system where you want to copy data to of from.
        ///
        /// The following arguments are optional:
        pub storage_virtual_machine_arn: pulumi_gestalt_rust::Output<String>,
        /// Path to the file share in the SVM where you'll copy your data. You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares) (e.g. `/vol1`, `/vol1/tree1`, `share1`).
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// URI of the FSx ONTAP file system location
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationFsxOntapFileSystemArgs,
    ) -> LocationFsxOntapFileSystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let protocol_binding = args.protocol.get_output(context);
        let security_group_arns_binding = args.security_group_arns.get_output(context);
        let storage_virtual_machine_arn_binding = args
            .storage_virtual_machine_arn
            .get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationFsxOntapFileSystem:LocationFsxOntapFileSystem"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupArns".into(),
                    value: &security_group_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageVirtualMachineArn".into(),
                    value: &storage_virtual_machine_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: &subdirectory_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationFsxOntapFileSystemResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            fsx_filesystem_arn: o.get_field("fsxFilesystemArn"),
            protocol: o.get_field("protocol"),
            security_group_arns: o.get_field("securityGroupArns"),
            storage_virtual_machine_arn: o.get_field("storageVirtualMachineArn"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
        }
    }
}
