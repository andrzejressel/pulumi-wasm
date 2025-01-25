/// Adds existing resource policies to a disk. You can only add one policy
/// which will be applied to this disk for scheduling snapshot creation.
///
/// > **Note:** This resource does not support zonal disks (`gcp.compute.Disk`). For zonal disks, please refer to the `gcp.compute.DiskResourcePolicyAttachment` resource.
///
///
///
///
/// ## Example Usage
///
/// ### Region Disk Resource Policy Attachment Basic
///
///
/// ```yaml
/// resources:
///   attachment:
///     type: gcp:compute:RegionDiskResourcePolicyAttachment
///     properties:
///       name: ${policy.name}
///       disk: ${ssd.name}
///       region: us-central1
///   disk:
///     type: gcp:compute:Disk
///     properties:
///       name: my-base-disk
///       image: debian-cloud/debian-11
///       size: 50
///       type: pd-ssd
///       zone: us-central1-a
///   snapdisk:
///     type: gcp:compute:Snapshot
///     properties:
///       name: my-snapshot
///       sourceDisk: ${disk.name}
///       zone: us-central1-a
///   ssd:
///     type: gcp:compute:RegionDisk
///     properties:
///       name: my-disk
///       replicaZones:
///         - us-central1-a
///         - us-central1-f
///       snapshot: ${snapdisk.id}
///       size: 50
///       type: pd-ssd
///       region: us-central1
///   policy:
///     type: gcp:compute:ResourcePolicy
///     properties:
///       name: my-resource-policy
///       region: us-central1
///       snapshotSchedulePolicy:
///         schedule:
///           dailySchedule:
///             daysInCycle: 1
///             startTime: 04:00
/// variables:
///   myImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// RegionDiskResourcePolicyAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/disks/{{disk}}/{{name}}`
///
/// * `{{project}}/{{region}}/{{disk}}/{{name}}`
///
/// * `{{region}}/{{disk}}/{{name}}`
///
/// * `{{disk}}/{{name}}`
///
/// When using the `pulumi import` command, RegionDiskResourcePolicyAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment default projects/{{project}}/regions/{{region}}/disks/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment default {{project}}/{{region}}/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment default {{region}}/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment default {{disk}}/{{name}}
/// ```
///
pub mod region_disk_resource_policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionDiskResourcePolicyAttachmentArgs {
        /// The name of the regional disk in which the resource policies are attached to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub disk: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource policy to be attached to the disk for scheduling snapshot
        /// creation. Do not specify the self link.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the disk resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionDiskResourcePolicyAttachmentResult {
        /// The name of the regional disk in which the resource policies are attached to.
        ///
        ///
        /// - - -
        pub disk: pulumi_wasm_rust::Output<String>,
        /// The resource policy to be attached to the disk for scheduling snapshot
        /// creation. Do not specify the self link.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// A reference to the region where the disk resides.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionDiskResourcePolicyAttachmentArgs,
    ) -> RegionDiskResourcePolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disk_binding = args.disk.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disk".into(),
                    value: &disk_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disk".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionDiskResourcePolicyAttachmentResult {
            disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disk").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
