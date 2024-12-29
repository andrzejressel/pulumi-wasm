/// Attaches a Lightsail disk to a Lightsail Instance
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let available = get_availability_zones::invoke(
///         GetAvailabilityZonesArgs::builder()
///             .filters(
///                 vec![
///                     GetAvailabilityZonesFilter::builder().name("opt-in-status")
///                     .values(vec!["opt-in-not-required",]).build_struct(),
///                 ],
///             )
///             .state("available")
///             .build_struct(),
///     );
///     let test = disk::create(
///         "test",
///         DiskArgs::builder()
///             .availability_zone("${available.names[0]}")
///             .name("test-disk")
///             .size_in_gb(8)
///             .build_struct(),
///     );
///     let testDisk_attachment = disk_attachment::create(
///         "testDisk_attachment",
///         DiskAttachmentArgs::builder()
///             .disk_name("${test.name}")
///             .disk_path("/dev/xvdf")
///             .instance_name("${testInstance.name}")
///             .build_struct(),
///     );
///     let testInstance = instance::create(
///         "testInstance",
///         InstanceArgs::builder()
///             .availability_zone("${available.names[0]}")
///             .blueprint_id("amazon_linux_2")
///             .bundle_id("nano_3_0")
///             .name("test-instance")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_disk` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/disk_attachment:Disk_attachment test test-disk,test-instance
/// ```
pub mod disk_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Disk_attachmentArgs {
        /// The name of the Lightsail Disk.
        #[builder(into)]
        pub disk_name: pulumi_wasm_rust::Output<String>,
        /// The disk path to expose to the instance.
        #[builder(into)]
        pub disk_path: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail Instance to attach to.
        #[builder(into)]
        pub instance_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct Disk_attachmentResult {
        /// The name of the Lightsail Disk.
        pub disk_name: pulumi_wasm_rust::Output<String>,
        /// The disk path to expose to the instance.
        pub disk_path: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail Instance to attach to.
        pub instance_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: Disk_attachmentArgs) -> Disk_attachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disk_name_binding = args.disk_name.get_inner();
        let disk_path_binding = args.disk_path.get_inner();
        let instance_name_binding = args.instance_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/disk_attachment:Disk_attachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "diskName".into(),
                    value: &disk_name_binding,
                },
                register_interface::ObjectField {
                    name: "diskPath".into(),
                    value: &disk_path_binding,
                },
                register_interface::ObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "diskName".into(),
                },
                register_interface::ResultField {
                    name: "diskPath".into(),
                },
                register_interface::ResultField {
                    name: "instanceName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Disk_attachmentResult {
            disk_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskName").unwrap(),
            ),
            disk_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskPath").unwrap(),
            ),
            instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceName").unwrap(),
            ),
        }
    }
}
