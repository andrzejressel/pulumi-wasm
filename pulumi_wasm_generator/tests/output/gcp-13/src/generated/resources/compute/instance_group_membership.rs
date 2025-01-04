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
pub mod instance_group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupMembershipArgs {
        /// An instance being added to the InstanceGroup
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
        /// Represents an Instance Group resource name that the instance belongs to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_group: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the zone where the instance group resides.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupMembershipResult {
        /// An instance being added to the InstanceGroup
        pub instance: pulumi_wasm_rust::Output<String>,
        /// Represents an Instance Group resource name that the instance belongs to.
        ///
        ///
        /// - - -
        pub instance_group: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// A reference to the zone where the instance group resides.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceGroupMembershipArgs,
    ) -> InstanceGroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_inner();
        let instance_group_binding = args.instance_group.get_inner();
        let project_binding = args.project.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroupMembership:InstanceGroupMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "instanceGroup".into(),
                    value: &instance_group_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroup".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceGroupMembershipResult {
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            instance_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroup").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
