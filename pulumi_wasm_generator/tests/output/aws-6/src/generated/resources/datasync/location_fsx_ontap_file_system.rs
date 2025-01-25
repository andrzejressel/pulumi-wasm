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
pub mod location_fsx_ontap_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationFsxOntapFileSystemArgs {
        /// The data transfer protocol that DataSync uses to access your Amazon FSx file system. See Protocol below.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::InputOrOutput<
            super::super::types::datasync::LocationFsxOntapFileSystemProtocol,
        >,
        /// The security groups that provide access to your file system's preferred subnet. The security groups must allow outbbound traffic on the following ports (depending on the protocol you use):
        /// * Network File System (NFS): TCP ports 111, 635, and 2049
        /// * Server Message Block (SMB): TCP port 445
        #[builder(into)]
        pub security_group_arns: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The ARN of the SVM in your file system where you want to copy data to of from.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub storage_virtual_machine_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Path to the file share in the SVM where you'll copy your data. You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares) (e.g. `/vol1`, `/vol1/tree1`, `share1`).
        #[builder(into, default)]
        pub subdirectory: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationFsxOntapFileSystemResult {
        /// ARN of the DataSync Location for the FSx Ontap File System.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// ARN of the FSx Ontap File System.
        pub fsx_filesystem_arn: pulumi_wasm_rust::Output<String>,
        /// The data transfer protocol that DataSync uses to access your Amazon FSx file system. See Protocol below.
        pub protocol: pulumi_wasm_rust::Output<
            super::super::types::datasync::LocationFsxOntapFileSystemProtocol,
        >,
        /// The security groups that provide access to your file system's preferred subnet. The security groups must allow outbbound traffic on the following ports (depending on the protocol you use):
        /// * Network File System (NFS): TCP ports 111, 635, and 2049
        /// * Server Message Block (SMB): TCP port 445
        pub security_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the SVM in your file system where you want to copy data to of from.
        ///
        /// The following arguments are optional:
        pub storage_virtual_machine_arn: pulumi_wasm_rust::Output<String>,
        /// Path to the file share in the SVM where you'll copy your data. You can specify a junction path (also known as a mount point), qtree path (for NFS file shares), or share name (for SMB file shares) (e.g. `/vol1`, `/vol1/tree1`, `share1`).
        pub subdirectory: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// URI of the FSx ONTAP file system location
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LocationFsxOntapFileSystemArgs,
    ) -> LocationFsxOntapFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let protocol_binding = args.protocol.get_output(context).get_inner();
        let security_group_arns_binding = args
            .security_group_arns
            .get_output(context)
            .get_inner();
        let storage_virtual_machine_arn_binding = args
            .storage_virtual_machine_arn
            .get_output(context)
            .get_inner();
        let subdirectory_binding = args.subdirectory.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/locationFsxOntapFileSystem:LocationFsxOntapFileSystem"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupArns".into(),
                    value: &security_group_arns_binding,
                },
                register_interface::ObjectField {
                    name: "storageVirtualMachineArn".into(),
                    value: &storage_virtual_machine_arn_binding,
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
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "fsxFilesystemArn".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupArns".into(),
                },
                register_interface::ResultField {
                    name: "storageVirtualMachineArn".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocationFsxOntapFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            fsx_filesystem_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fsxFilesystemArn").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            security_group_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupArns").unwrap(),
            ),
            storage_virtual_machine_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageVirtualMachineArn").unwrap(),
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
