/// Persistent disks can be attached to a compute instance using the `attached_disk`
/// section within the compute instance configuration.
/// However there may be situations where managing the attached disks via the compute
/// instance config isn't preferable or possible, such as attaching dynamic
/// numbers of disks using the `count` variable.
///
///
/// To get more information about attaching disks, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instances/attachDisk)
/// * How-to Guides
///     * [Adding a persistent disk](https://cloud.google.com/compute/docs/disks/add-persistent-disk)
///
/// **Note:** When using `gcp.compute.AttachedDisk` you **must** use `lifecycle.ignore_changes = ["attached_disk"]` on the `gcp.compute.Instance` resource that has the disks attached. Otherwise the two resources will fight for control of the attached disk block.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = attached_disk::create(
///         "default",
///         AttachedDiskArgs::builder()
///             .disk("${defaultGoogleComputeDisk.id}")
///             .instance("${defaultInstance.id}")
///             .build_struct(),
///     );
///     let defaultInstance = instance::create(
///         "defaultInstance",
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
///             .name("attached-disk-instance")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder().network("default")
///                     .build_struct(),
///                 ],
///             )
///             .zone("us-west1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Attached Disk can be imported the following ways:
///
/// * `projects/{{project}}/zones/{{zone}}/instances/{{instance.name}}/{{disk.name}}`
///
/// * `{{project}}/{{zone}}/{{instance.name}}/{{disk.name}}`
///
/// When using the `pulumi import` command, Attached Disk can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/attachedDisk:AttachedDisk default projects/{{project}}/zones/{{zone}}/instances/{{instance.name}}/{{disk.name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/attachedDisk:AttachedDisk default {{project}}/{{zone}}/{{instance.name}}/{{disk.name}}
/// ```
///
pub mod attached_disk {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachedDiskArgs {
        /// Specifies a unique device name of your choice that is
        /// reflected into the /dev/disk/by-id/google-* tree of a Linux operating
        /// system running within the instance. This name can be used to
        /// reference the device for mounting, resizing, and so on, from within
        /// the instance.
        ///
        /// If not specified, the server chooses a default device name to apply
        /// to this disk, in the form persistent-disks-x, where x is a number
        /// assigned by Google Compute Engine.
        #[builder(into, default)]
        pub device_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// `name` or `self_link` of the disk that will be attached.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub disk: pulumi_gestalt_rust::InputOrOutput<String>,
        /// `name` or `self_link` of the compute instance that the disk will be attached to.
        /// If the `self_link` is provided then `zone` and `project` are extracted from the
        /// self link. If only the name is used then `zone` and `project` must be defined
        /// as properties on the resource or provider.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The disk interface used for attaching this disk.
        ///
        /// This field is only used for specific cases, please don't specify
        /// this field without advice from Google. Not specifying the field
        /// will allow the the server to assign the correct interface.
        ///
        /// Possible values:
        /// "SCSI"
        /// "NVME"
        #[builder(into, default)]
        pub interface: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The mode in which to attach this disk, either READ_WRITE or
        /// READ_ONLY. If not specified, the default is to attach the disk in
        /// READ_WRITE mode.
        ///
        /// Possible values:
        /// "READ_ONLY"
        /// "READ_WRITE"
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project that the referenced compute instance is a part of. If `instance` is referenced by its
        /// `self_link` the project defined in the link will take precedence.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone that the referenced compute instance is located within. If `instance` is referenced by its
        /// `self_link` the zone defined in the link will take precedence.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AttachedDiskResult {
        /// Specifies a unique device name of your choice that is
        /// reflected into the /dev/disk/by-id/google-* tree of a Linux operating
        /// system running within the instance. This name can be used to
        /// reference the device for mounting, resizing, and so on, from within
        /// the instance.
        ///
        /// If not specified, the server chooses a default device name to apply
        /// to this disk, in the form persistent-disks-x, where x is a number
        /// assigned by Google Compute Engine.
        pub device_name: pulumi_gestalt_rust::Output<String>,
        /// `name` or `self_link` of the disk that will be attached.
        ///
        ///
        /// - - -
        pub disk: pulumi_gestalt_rust::Output<String>,
        /// `name` or `self_link` of the compute instance that the disk will be attached to.
        /// If the `self_link` is provided then `zone` and `project` are extracted from the
        /// self link. If only the name is used then `zone` and `project` must be defined
        /// as properties on the resource or provider.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The disk interface used for attaching this disk.
        ///
        /// This field is only used for specific cases, please don't specify
        /// this field without advice from Google. Not specifying the field
        /// will allow the the server to assign the correct interface.
        ///
        /// Possible values:
        /// "SCSI"
        /// "NVME"
        pub interface: pulumi_gestalt_rust::Output<Option<String>>,
        /// The mode in which to attach this disk, either READ_WRITE or
        /// READ_ONLY. If not specified, the default is to attach the disk in
        /// READ_WRITE mode.
        ///
        /// Possible values:
        /// "READ_ONLY"
        /// "READ_WRITE"
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The project that the referenced compute instance is a part of. If `instance` is referenced by its
        /// `self_link` the project defined in the link will take precedence.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The zone that the referenced compute instance is located within. If `instance` is referenced by its
        /// `self_link` the zone defined in the link will take precedence.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AttachedDiskArgs,
    ) -> AttachedDiskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_name_binding = args.device_name.get_output(context).get_inner();
        let disk_binding = args.disk.get_output(context).get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let interface_binding = args.interface.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/attachedDisk:AttachedDisk".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceName".into(),
                    value: &device_name_binding,
                },
                register_interface::ObjectField {
                    name: "disk".into(),
                    value: &disk_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "interface".into(),
                    value: &interface_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AttachedDiskResult {
            device_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceName"),
            ),
            disk: pulumi_gestalt_rust::__private::into_domain(o.extract_field("disk")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            interface: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interface"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
