/// Represents a Machine Image resource. Machine images store all the configuration,
/// metadata, permissions, and data from one or more disks required to create a
/// Virtual machine (VM) instance.
///
/// To get more information about MachineImage, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/machineImages)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/machine-images)
///
/// ## Example Usage
///
/// ### Machine Image Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let image = machine_image::create(
///         "image",
///         MachineImageArgs::builder()
///             .name("my-image")
///             .source_instance("${vm.selfLink}")
///             .build_struct(),
///     );
///     let vm = instance::create(
///         "vm",
///         InstanceArgs::builder()
///             .boot_disk(
///                 InstanceBootDisk::builder()
///                     .initializeParams(
///                         InstanceBootDiskInitializeParams::builder()
///                             .image("debian-cloud/debian-11")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .machine_type("e2-medium")
///             .name("my-vm")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder().network("default")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Machine Image Kms
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder().key_ring("${keyRing.id}").name("key").build_struct(),
///     );
///     let image = machine_image::create(
///         "image",
///         MachineImageArgs::builder()
///             .machine_image_encryption_key(
///                 MachineImageMachineImageEncryptionKey::builder()
///                     .kmsKeyName("${cryptoKey.id}")
///                     .build_struct(),
///             )
///             .name("my-image")
///             .source_instance("${vm.selfLink}")
///             .build_struct(),
///     );
///     let keyRing = key_ring::create(
///         "keyRing",
///         KeyRingArgs::builder().location("us").name("keyring").build_struct(),
///     );
///     let vm = instance::create(
///         "vm",
///         InstanceArgs::builder()
///             .boot_disk(
///                 InstanceBootDisk::builder()
///                     .initializeParams(
///                         InstanceBootDiskInitializeParams::builder()
///                             .image("debian-cloud/debian-11")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .machine_type("e2-medium")
///             .name("my-vm")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder().network("default")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MachineImage can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/machineImages/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, MachineImage can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/machineImage:MachineImage default projects/{{project}}/global/machineImages/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/machineImage:MachineImage default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/machineImage:MachineImage default {{name}}
/// ```
///
pub mod machine_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MachineImageArgs {
        /// A text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify this to create an application consistent machine image by informing the OS to prepare for the snapshot process.
        /// Currently only supported on Windows instances using the Volume Shadow Copy Service (VSS).
        #[builder(into, default)]
        pub guest_flush: pulumi_wasm_rust::Output<Option<bool>>,
        /// Encrypts the machine image using a customer-supplied encryption key.
        /// After you encrypt a machine image with a customer-supplied key, you must
        /// provide the same key if you use the machine image later (e.g. to create a
        /// instance from the image)
        /// Structure is documented below.
        #[builder(into, default)]
        pub machine_image_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::MachineImageMachineImageEncryptionKey>,
        >,
        /// Name of the resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The source instance used to create the machine image. You can provide this as a partial or full URL to the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub source_instance: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MachineImageResult {
        /// A text description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify this to create an application consistent machine image by informing the OS to prepare for the snapshot process.
        /// Currently only supported on Windows instances using the Volume Shadow Copy Service (VSS).
        pub guest_flush: pulumi_wasm_rust::Output<Option<bool>>,
        /// Encrypts the machine image using a customer-supplied encryption key.
        /// After you encrypt a machine image with a customer-supplied key, you must
        /// provide the same key if you use the machine image later (e.g. to create a
        /// instance from the image)
        /// Structure is documented below.
        pub machine_image_encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::MachineImageMachineImageEncryptionKey>,
        >,
        /// Name of the resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The source instance used to create the machine image. You can provide this as a partial or full URL to the resource.
        ///
        ///
        /// - - -
        pub source_instance: pulumi_wasm_rust::Output<String>,
        /// The regional or multi-regional Cloud Storage bucket location where the machine image is stored.
        pub storage_locations: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MachineImageArgs) -> MachineImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let guest_flush_binding = args.guest_flush.get_inner();
        let machine_image_encryption_key_binding = args
            .machine_image_encryption_key
            .get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let source_instance_binding = args.source_instance.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/machineImage:MachineImage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "guestFlush".into(),
                    value: &guest_flush_binding,
                },
                register_interface::ObjectField {
                    name: "machineImageEncryptionKey".into(),
                    value: &machine_image_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sourceInstance".into(),
                    value: &source_instance_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "guestFlush".into(),
                },
                register_interface::ResultField {
                    name: "machineImageEncryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "sourceInstance".into(),
                },
                register_interface::ResultField {
                    name: "storageLocations".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MachineImageResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            guest_flush: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guestFlush").unwrap(),
            ),
            machine_image_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineImageEncryptionKey").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            source_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceInstance").unwrap(),
            ),
            storage_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageLocations").unwrap(),
            ),
        }
    }
}
