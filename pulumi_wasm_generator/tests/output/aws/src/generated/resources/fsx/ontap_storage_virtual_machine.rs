/// Manages a FSx Storage Virtual Machine.
/// See the [FSx ONTAP User Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-svms.html) for more information.
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
///     let test = ontap_storage_virtual_machine::create(
///         "test",
///         OntapStorageVirtualMachineArgs::builder()
///             .file_system_id("${testAwsFsxOntapFileSystem.id}")
///             .name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using a Self-Managed Microsoft Active Directory
///
/// Additional information for using AWS Directory Service with ONTAP File Systems can be found in the [FSx ONTAP Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/self-managed-AD.html).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = ontap_storage_virtual_machine::create(
///         "test",
///         OntapStorageVirtualMachineArgs::builder()
///             .active_directory_configuration(
///                 OntapStorageVirtualMachineActiveDirectoryConfiguration::builder()
///                     .netbiosName("mysvm")
///                     .selfManagedActiveDirectoryConfiguration(
///                         OntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration::builder()
///                             .dnsIps(vec!["10.0.0.111", "10.0.0.222",])
///                             .domainName("corp.example.com")
///                             .password("avoid-plaintext-passwords")
///                             .username("Admin")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .file_system_id("${testAwsFsxOntapFileSystem.id}")
///             .name("mysvm")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx Storage Virtual Machine using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/ontapStorageVirtualMachine:OntapStorageVirtualMachine example svm-12345678abcdef123
/// ```
/// Certain resource arguments, like `svm_admin_password` and the `self_managed_active_directory` configuation block `password`, do not have a FSx API method for reading the information after creation. If these arguments are set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
pub mod ontap_storage_virtual_machine {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OntapStorageVirtualMachineArgs {
        /// Configuration block that Amazon FSx uses to join the FSx ONTAP Storage Virtual Machine(SVM) to your Microsoft Active Directory (AD) directory. Detailed below.
        #[builder(into, default)]
        pub active_directory_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::fsx::OntapStorageVirtualMachineActiveDirectoryConfiguration,
            >,
        >,
        /// The ID of the Amazon FSx ONTAP File System that this SVM will be created on.
        #[builder(into)]
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The name of the SVM. You can use a maximum of 47 alphanumeric characters, plus the underscore (_) special character.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the root volume security style, Valid values are `UNIX`, `NTFS`, and `MIXED`. All volumes created under this SVM will inherit the root security style unless the security style is specified on the volume. Default value is `UNIX`.
        #[builder(into, default)]
        pub root_volume_security_style: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the password to use when logging on to the SVM using a secure shell (SSH) connection to the SVM's management endpoint. Doing so enables you to manage the SVM using the NetApp ONTAP CLI or REST API. If you do not specify a password, you can still use the file system's fsxadmin user to manage the SVM.
        #[builder(into, default)]
        pub svm_admin_password: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the storage virtual machine. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct OntapStorageVirtualMachineResult {
        /// Configuration block that Amazon FSx uses to join the FSx ONTAP Storage Virtual Machine(SVM) to your Microsoft Active Directory (AD) directory. Detailed below.
        pub active_directory_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::fsx::OntapStorageVirtualMachineActiveDirectoryConfiguration,
            >,
        >,
        /// Amazon Resource Name of the storage virtual machine.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The endpoints that are used to access data or to manage the storage virtual machine using the NetApp ONTAP CLI, REST API, or NetApp SnapMirror. See Endpoints below.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpoint>,
        >,
        /// The ID of the Amazon FSx ONTAP File System that this SVM will be created on.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The name of the SVM. You can use a maximum of 47 alphanumeric characters, plus the underscore (_) special character.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the root volume security style, Valid values are `UNIX`, `NTFS`, and `MIXED`. All volumes created under this SVM will inherit the root security style unless the security style is specified on the volume. Default value is `UNIX`.
        pub root_volume_security_style: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the SVM's subtype, e.g. `DEFAULT`
        pub subtype: pulumi_wasm_rust::Output<String>,
        /// Specifies the password to use when logging on to the SVM using a secure shell (SSH) connection to the SVM's management endpoint. Doing so enables you to manage the SVM using the NetApp ONTAP CLI or REST API. If you do not specify a password, you can still use the file system's fsxadmin user to manage the SVM.
        pub svm_admin_password: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the storage virtual machine. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The SVM's UUID (universally unique identifier).
        pub uuid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OntapStorageVirtualMachineArgs,
    ) -> OntapStorageVirtualMachineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_directory_configuration_binding = args
            .active_directory_configuration
            .get_inner();
        let file_system_id_binding = args.file_system_id.get_inner();
        let name_binding = args.name.get_inner();
        let root_volume_security_style_binding = args
            .root_volume_security_style
            .get_inner();
        let svm_admin_password_binding = args.svm_admin_password.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/ontapStorageVirtualMachine:OntapStorageVirtualMachine"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeDirectoryConfiguration".into(),
                    value: &active_directory_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rootVolumeSecurityStyle".into(),
                    value: &root_volume_security_style_binding,
                },
                register_interface::ObjectField {
                    name: "svmAdminPassword".into(),
                    value: &svm_admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeDirectoryConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rootVolumeSecurityStyle".into(),
                },
                register_interface::ResultField {
                    name: "subtype".into(),
                },
                register_interface::ResultField {
                    name: "svmAdminPassword".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "uuid".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OntapStorageVirtualMachineResult {
            active_directory_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeDirectoryConfiguration").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            root_volume_security_style: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootVolumeSecurityStyle").unwrap(),
            ),
            subtype: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subtype").unwrap(),
            ),
            svm_admin_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("svmAdminPassword").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uuid").unwrap(),
            ),
        }
    }
}