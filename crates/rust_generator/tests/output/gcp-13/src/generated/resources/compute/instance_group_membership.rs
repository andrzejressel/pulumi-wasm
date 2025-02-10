/// Represents the Instance membership to the Instance Group.
///
/// > **NOTE** You can use this resource instead of the `instances` field in the
/// `gcp.compute.InstanceGroup`, however it's not recommended to use it alongside this field.
/// It might cause inconsistencies, as they can end up competing over control.
///
/// > **NOTE** This resource has been added to avoid a situation, where after
/// Instance is recreated, it's removed from Instance Group and it's needed to
/// perform `apply` twice. To avoid situations like this, please use this resource
/// with the lifecycle `replace_triggered_by` method, with the passed Instance's ID.
///
///
/// To get more information about InstanceGroupMembership, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroups)
/// * How-to Guides
///     * [Add instances](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroups/addInstances)
///     * [List instances](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroups/listInstances)
///     * [Remove instances](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroups/removeInstances)
///
/// ## Example Usage
///
/// ### Instance Group Membership
///
///
/// ```yaml
/// resources:
///   default-network:
///     type: gcp:compute:Network
///     properties:
///       name: network
///   default-instance:
///     type: gcp:compute:Instance
///     properties:
///       name: instance
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: debian-cloud/debian-11
///       networkInterfaces:
///         - network: ${["default-network"].name}
///   default-instance-group:
///     type: gcp:compute:InstanceGroup
///     properties:
///       name: instance-group
///   default-ig-membership:
///     type: gcp:compute:InstanceGroupMembership
///     properties:
///       instance: ${["default-instance"].selfLink}
///       instanceGroup: ${["default-instance-group"].name}
/// ```
///
/// ## Import
///
/// InstanceGroupMembership can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroups/{{instance_group}}/{{instance}}`
///
/// * `{{project}}/{{zone}}/{{instance_group}}/{{instance}}`
///
/// * `{{zone}}/{{instance_group}}/{{instance}}`
///
/// * `{{instance_group}}/{{instance}}`
///
/// When using the `pulumi import` command, InstanceGroupMembership can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupMembership:InstanceGroupMembership default projects/{{project}}/zones/{{zone}}/instanceGroups/{{instance_group}}/{{instance}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupMembership:InstanceGroupMembership default {{project}}/{{zone}}/{{instance_group}}/{{instance}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupMembership:InstanceGroupMembership default {{zone}}/{{instance_group}}/{{instance}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroupMembership:InstanceGroupMembership default {{instance_group}}/{{instance}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_group_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupMembershipArgs {
        /// An instance being added to the InstanceGroup
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Represents an Instance Group resource name that the instance belongs to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the instance group resides.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupMembershipResult {
        /// An instance being added to the InstanceGroup
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Represents an Instance Group resource name that the instance belongs to.
        ///
        ///
        /// - - -
        pub instance_group: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A reference to the zone where the instance group resides.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupMembershipArgs,
    ) -> InstanceGroupMembershipResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_binding = args.instance.get_output(context);
        let instance_group_binding = args.instance_group.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroupMembership:InstanceGroupMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceGroup".into(),
                    value: instance_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceGroupMembershipResult {
            instance: o.get_field("instance"),
            instance_group: o.get_field("instanceGroup"),
            project: o.get_field("project"),
            zone: o.get_field("zone"),
        }
    }
}
