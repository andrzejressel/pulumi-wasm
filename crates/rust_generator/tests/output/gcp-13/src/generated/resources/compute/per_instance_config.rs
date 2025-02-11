/// A config defined for a single managed instance that belongs to an instance group manager. It preserves the instance name
/// across instance group manager operations and can define stateful disks or metadata that are unique to the instance.
///
///
/// To get more information about PerInstanceConfig, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceGroupManagers)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/instance-groups/stateful-migs#per-instance_configs)
///
/// ## Example Usage
///
/// ### Stateful Igm
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
///   igm-no-tp:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       description: Test instance group manager
///       name: my-igm
///       versions:
///         - name: prod
///           instanceTemplate: ${["igm-basic"].selfLink}
///       baseInstanceName: igm-no-tp
///       zone: us-central1-c
///       targetSize: 2
///   default:
///     type: gcp:compute:Disk
///     properties:
///       name: my-disk-name
///       type: pd-ssd
///       zone: ${igm.zone}
///       image: debian-11-bullseye-v20220719
///       physicalBlockSizeBytes: 4096
///   withDisk:
///     type: gcp:compute:PerInstanceConfig
///     name: with_disk
///     properties:
///       zone: ${igm.zone}
///       instanceGroupManager: ${igm.name}
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
/// PerInstanceConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{instance_group_manager}}/{{name}}`
///
/// * `{{project}}/{{zone}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{zone}}/{{instance_group_manager}}/{{name}}`
///
/// * `{{instance_group_manager}}/{{name}}`
///
/// When using the `pulumi import` command, PerInstanceConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/perInstanceConfig:PerInstanceConfig default projects/{{project}}/zones/{{zone}}/instanceGroupManagers/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/perInstanceConfig:PerInstanceConfig default {{project}}/{{zone}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/perInstanceConfig:PerInstanceConfig default {{zone}}/{{instance_group_manager}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/perInstanceConfig:PerInstanceConfig default {{instance_group_manager}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod per_instance_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PerInstanceConfigArgs {
        /// The instance group manager this instance config is part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_group_manager: pulumi_gestalt_rust::InputOrOutput<String>,
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
            Option<super::super::types::compute::PerInstanceConfigPreservedState>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        /// Zone where the containing instance group manager is located
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PerInstanceConfigResult {
        /// The instance group manager this instance config is part of.
        ///
        ///
        /// - - -
        pub instance_group_manager: pulumi_gestalt_rust::Output<String>,
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
            Option<super::super::types::compute::PerInstanceConfigPreservedState>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// When true, deleting this config will immediately remove the underlying instance.
        /// When false, deleting this config will use the behavior as determined by remove_instance_on_destroy.
        pub remove_instance_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When true, deleting this config will immediately remove any specified state from the underlying instance.
        /// When false, deleting this config will *not* immediately remove any state from the underlying instance.
        /// State will be removed on the next instance recreation or update.
        pub remove_instance_state_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Zone where the containing instance group manager is located
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PerInstanceConfigArgs,
    ) -> PerInstanceConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_group_manager_binding = args
            .instance_group_manager
            .get_output(context);
        let minimal_action_binding = args.minimal_action.get_output(context);
        let most_disruptive_allowed_action_binding = args
            .most_disruptive_allowed_action
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let preserved_state_binding = args.preserved_state.get_output(context);
        let project_binding = args.project.get_output(context);
        let remove_instance_on_destroy_binding = args
            .remove_instance_on_destroy
            .get_output(context);
        let remove_instance_state_on_destroy_binding = args
            .remove_instance_state_on_destroy
            .get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/perInstanceConfig:PerInstanceConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceGroupManager".into(),
                    value: &instance_group_manager_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimalAction".into(),
                    value: &minimal_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostDisruptiveAllowedAction".into(),
                    value: &most_disruptive_allowed_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preservedState".into(),
                    value: &preserved_state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "removeInstanceOnDestroy".into(),
                    value: &remove_instance_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "removeInstanceStateOnDestroy".into(),
                    value: &remove_instance_state_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PerInstanceConfigResult {
            instance_group_manager: o.get_field("instanceGroupManager"),
            minimal_action: o.get_field("minimalAction"),
            most_disruptive_allowed_action: o.get_field("mostDisruptiveAllowedAction"),
            name: o.get_field("name"),
            preserved_state: o.get_field("preservedState"),
            project: o.get_field("project"),
            remove_instance_on_destroy: o.get_field("removeInstanceOnDestroy"),
            remove_instance_state_on_destroy: o
                .get_field("removeInstanceStateOnDestroy"),
            zone: o.get_field("zone"),
        }
    }
}
