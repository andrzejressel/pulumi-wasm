/// A config defined for a single managed instance that belongs to an instance group manager. It preserves the instance name
/// across instance group manager operations and can define stateful disks or metadata that are unique to the instance.
/// This resource works with regional instance group managers.
///
///
/// To get more information about RegionPerInstanceConfig, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/instance-groups/stateful-migs#per-instance_configs)
///
/// ## Example Usage
///
/// ### Stateful Rigm
///
///
/// ```yaml
/// resources:
///   igm-basic:
///     type: gcp:compute:InstanceTemplate
///     properties:
///       name: my-template
///       machineType: e2-medium
///       canIpForward: false
///       tags:
///         - foo
///         - bar
///       disks:
///         - sourceImage: ${myImage.selfLink}
///           autoDelete: true
///           boot: true
///       networkInterfaces:
///         - network: default
///       serviceAccount:
///         scopes:
///           - userinfo-email
///           - compute-ro
///           - storage-ro
///   rigm:
///     type: gcp:compute:RegionInstanceGroupManager
///     properties:
///       description: Demo test instance group manager
///       name: my-rigm
///       versions:
///         - name: prod
///           instanceTemplate: ${["igm-basic"].selfLink}
///       updatePolicy:
///         type: OPPORTUNISTIC
///         instanceRedistributionType: NONE
///         minimalAction: RESTART
///       baseInstanceName: rigm
///       region: us-central1
///       targetSize: 2
///   default:
///     type: gcp:compute:Disk
///     properties:
///       name: my-disk-name
///       type: pd-ssd
///       zone: us-central1-a
///       image: debian-11-bullseye-v20220719
///       physicalBlockSizeBytes: 4096
///   withDisk:
///     type: gcp:compute:RegionPerInstanceConfig
///     name: with_disk
///     properties:
///       region: ${igm.region}
///       regionInstanceGroupManager: ${rigm.name}
///       name: instance-1
///       preservedState:
///         metadata:
///           foo: bar
///           instance_template: ${["igm-basic"].selfLink}
///         disks:
///           - deviceName: my-stateful-disk
///             source: ${default.id}
///             mode: READ_ONLY
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
/// RegionPerInstanceConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/instanceGroupManagers/{{region_instance_group_manager}}/{{name}}`
///
/// * `{{project}}/{{region}}/{{region_instance_group_manager}}/{{name}}`
///
/// * `{{region}}/{{region_instance_group_manager}}/{{name}}`
///
/// * `{{region_instance_group_manager}}/{{name}}`
///
/// When using the `pulumi import` command, RegionPerInstanceConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionPerInstanceConfig:RegionPerInstanceConfig default projects/{{project}}/regions/{{region}}/instanceGroupManagers/{{region_instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionPerInstanceConfig:RegionPerInstanceConfig default {{project}}/{{region}}/{{region_instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionPerInstanceConfig:RegionPerInstanceConfig default {{region}}/{{region_instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionPerInstanceConfig:RegionPerInstanceConfig default {{region_instance_group_manager}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_per_instance_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionPerInstanceConfigArgs {
        /// The minimal action to perform on the instance during an update.
        /// Default is `NONE`. Possible values are:
        /// * REPLACE
        /// * RESTART
        /// * REFRESH
        /// * NONE
        #[builder(into, default)]
        pub minimal_action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The most disruptive action to perform on the instance during an update.
        /// Default is `REPLACE`. Possible values are:
        /// * REPLACE
        /// * RESTART
        /// * REFRESH
        /// * NONE
        #[builder(into, default)]
        pub most_disruptive_allowed_action: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name for this per-instance config and its corresponding instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The preserved state for this instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub preserved_state: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionPerInstanceConfigPreservedState>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the containing instance group manager is located
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region instance group manager this instance config is part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region_instance_group_manager: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When true, deleting this config will immediately remove the underlying instance.
        /// When false, deleting this config will use the behavior as determined by remove_instance_on_destroy.
        #[builder(into, default)]
        pub remove_instance_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When true, deleting this config will immediately remove any specified state from the underlying instance.
        /// When false, deleting this config will *not* immediately remove any state from the underlying instance.
        /// State will be removed on the next instance recreation or update.
        #[builder(into, default)]
        pub remove_instance_state_on_destroy: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegionPerInstanceConfigResult {
        /// The minimal action to perform on the instance during an update.
        /// Default is `NONE`. Possible values are:
        /// * REPLACE
        /// * RESTART
        /// * REFRESH
        /// * NONE
        pub minimal_action: pulumi_gestalt_rust::Output<Option<String>>,
        /// The most disruptive action to perform on the instance during an update.
        /// Default is `REPLACE`. Possible values are:
        /// * REPLACE
        /// * RESTART
        /// * REFRESH
        /// * NONE
        pub most_disruptive_allowed_action: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name for this per-instance config and its corresponding instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The preserved state for this instance.
        /// Structure is documented below.
        pub preserved_state: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionPerInstanceConfigPreservedState>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where the containing instance group manager is located
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The region instance group manager this instance config is part of.
        ///
        ///
        /// - - -
        pub region_instance_group_manager: pulumi_gestalt_rust::Output<String>,
        /// When true, deleting this config will immediately remove the underlying instance.
        /// When false, deleting this config will use the behavior as determined by remove_instance_on_destroy.
        pub remove_instance_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When true, deleting this config will immediately remove any specified state from the underlying instance.
        /// When false, deleting this config will *not* immediately remove any state from the underlying instance.
        /// State will be removed on the next instance recreation or update.
        pub remove_instance_state_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionPerInstanceConfigArgs,
    ) -> RegionPerInstanceConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let minimal_action_binding_1 = args.minimal_action.get_output(context);
        let minimal_action_binding = minimal_action_binding_1.get_inner();
        let most_disruptive_allowed_action_binding_1 = args
            .most_disruptive_allowed_action
            .get_output(context);
        let most_disruptive_allowed_action_binding = most_disruptive_allowed_action_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let preserved_state_binding_1 = args.preserved_state.get_output(context);
        let preserved_state_binding = preserved_state_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let region_instance_group_manager_binding_1 = args
            .region_instance_group_manager
            .get_output(context);
        let region_instance_group_manager_binding = region_instance_group_manager_binding_1
            .get_inner();
        let remove_instance_on_destroy_binding_1 = args
            .remove_instance_on_destroy
            .get_output(context);
        let remove_instance_on_destroy_binding = remove_instance_on_destroy_binding_1
            .get_inner();
        let remove_instance_state_on_destroy_binding_1 = args
            .remove_instance_state_on_destroy
            .get_output(context);
        let remove_instance_state_on_destroy_binding = remove_instance_state_on_destroy_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionPerInstanceConfig:RegionPerInstanceConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "minimalAction".into(),
                    value: &minimal_action_binding,
                },
                register_interface::ObjectField {
                    name: "mostDisruptiveAllowedAction".into(),
                    value: &most_disruptive_allowed_action_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "preservedState".into(),
                    value: &preserved_state_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "regionInstanceGroupManager".into(),
                    value: &region_instance_group_manager_binding,
                },
                register_interface::ObjectField {
                    name: "removeInstanceOnDestroy".into(),
                    value: &remove_instance_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "removeInstanceStateOnDestroy".into(),
                    value: &remove_instance_state_on_destroy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionPerInstanceConfigResult {
            minimal_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimalAction"),
            ),
            most_disruptive_allowed_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostDisruptiveAllowedAction"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            preserved_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preservedState"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            region_instance_group_manager: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("regionInstanceGroupManager"),
            ),
            remove_instance_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("removeInstanceOnDestroy"),
            ),
            remove_instance_state_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("removeInstanceStateOnDestroy"),
            ),
        }
    }
}
