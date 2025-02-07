/// Adds existing resource policies to a disk. You can only add one policy
/// which will be applied to this disk for scheduling snapshot creation.
///
/// > **Note:** This resource does not support regional disks (`gcp.compute.RegionDisk`). For regional disks, please refer to the `gcp.compute.RegionDiskResourcePolicyAttachment` resource.
///
///
/// ## Example Usage
///
/// ### Disk Resource Policy Attachment Basic
///
///
/// ```yaml
/// resources:
///   attachment:
///     type: gcp:compute:DiskResourcePolicyAttachment
///     properties:
///       name: ${policy.name}
///       disk: ${ssd.name}
///       zone: us-central1-a
///   ssd:
///     type: gcp:compute:Disk
///     properties:
///       name: my-disk
///       image: ${myImage.selfLink}
///       size: 50
///       type: pd-ssd
///       zone: us-central1-a
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
/// DiskResourcePolicyAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/disks/{{disk}}/{{name}}`
///
/// * `{{project}}/{{zone}}/{{disk}}/{{name}}`
///
/// * `{{zone}}/{{disk}}/{{name}}`
///
/// * `{{disk}}/{{name}}`
///
/// When using the `pulumi import` command, DiskResourcePolicyAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/diskResourcePolicyAttachment:DiskResourcePolicyAttachment default projects/{{project}}/zones/{{zone}}/disks/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/diskResourcePolicyAttachment:DiskResourcePolicyAttachment default {{project}}/{{zone}}/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/diskResourcePolicyAttachment:DiskResourcePolicyAttachment default {{zone}}/{{disk}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/diskResourcePolicyAttachment:DiskResourcePolicyAttachment default {{disk}}/{{name}}
/// ```
///
pub mod disk_resource_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskResourcePolicyAttachmentArgs {
        /// The name of the disk in which the resource policies are attached to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub disk: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource policy to be attached to the disk for scheduling snapshot
        /// creation. Do not specify the self link.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the disk resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DiskResourcePolicyAttachmentResult {
        /// The name of the disk in which the resource policies are attached to.
        ///
        ///
        /// - - -
        pub disk: pulumi_gestalt_rust::Output<String>,
        /// The resource policy to be attached to the disk for scheduling snapshot
        /// creation. Do not specify the self link.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A reference to the zone where the disk resides.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DiskResourcePolicyAttachmentArgs,
    ) -> DiskResourcePolicyAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disk_binding = args.disk.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/diskResourcePolicyAttachment:DiskResourcePolicyAttachment"
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
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DiskResourcePolicyAttachmentResult {
            disk: pulumi_gestalt_rust::__private::into_domain(o.extract_field("disk")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
