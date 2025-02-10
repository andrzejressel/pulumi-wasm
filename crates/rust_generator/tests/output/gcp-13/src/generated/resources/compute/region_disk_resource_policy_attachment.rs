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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_disk_resource_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionDiskResourcePolicyAttachmentArgs {
        /// The name of the regional disk in which the resource policies are attached to.
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
        /// A reference to the region where the disk resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionDiskResourcePolicyAttachmentResult {
        /// The name of the regional disk in which the resource policies are attached to.
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
        /// A reference to the region where the disk resides.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionDiskResourcePolicyAttachmentArgs,
    ) -> RegionDiskResourcePolicyAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disk_binding = args.disk.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionDiskResourcePolicyAttachment:RegionDiskResourcePolicyAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disk".into(),
                    value: disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionDiskResourcePolicyAttachmentResult {
            disk: o.get_field("disk"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
