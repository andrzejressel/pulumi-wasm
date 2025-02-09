/// An isolated set of Cloud Spanner resources on which databases can be
/// hosted.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/spanner/docs/reference/rest/v1/projects.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/spanner/)
///
/// ## Example Usage
///
/// ### Spanner Instance Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:spanner:Instance
///     properties:
///       config: regional-us-central1
///       displayName: Test Spanner Instance
///       numNodes: 2
///       edition: STANDARD
///       defaultBackupScheduleType: AUTOMATIC
///       labels:
///         foo: bar
/// ```
/// ### Spanner Instance Processing Units
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:spanner:Instance
///     properties:
///       config: regional-us-central1
///       displayName: Test Spanner Instance
///       processingUnits: 200
///       labels:
///         foo: bar
/// ```
/// ### Spanner Instance With Autoscaling
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:spanner:Instance
///     properties:
///       config: regional-us-central1
///       displayName: Test Spanner Instance
///       autoscalingConfig:
///         autoscalingLimits:
///           maxProcessingUnits: 3000
///           minProcessingUnits: 2000
///         autoscalingTargets:
///           highPriorityCpuUtilizationPercent: 75
///           storageUtilizationPercent: 90
///       labels:
///         foo: bar
/// ```
/// ### Spanner Instance Multi Regional
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:spanner:Instance
///     properties:
///       config: nam-eur-asia1
///       displayName: Multi Regional Instance
///       numNodes: 2
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:spanner/instance:Instance default projects/{{project}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/instance:Instance default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/instance:Instance default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The autoscaling configuration. Autoscaling is enabled if this field is set.
        /// When autoscaling is enabled, num_nodes and processing_units are treated as,
        /// OUTPUT_ONLY fields and reflect the current compute capacity allocated to
        /// the instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscaling_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::InstanceAutoscalingConfig>,
        >,
        /// The name of the instance's configuration (similar but not
        /// quite the same as a region) which defines the geographic placement and
        /// replication of your databases in this instance. It determines where your data
        /// is stored. Values are typically of the form `regional-europe-west1` , `us-central` etc.
        /// In order to obtain a valid list please consult the
        /// [Configuration section of the docs](https://cloud.google.com/spanner/docs/instances).
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Controls the default backup behavior for new databases within the instance.
        /// Note that `AUTOMATIC` is not permitted for free instances, as backups and backup schedules are not allowed for free instances.
        /// if unset or NONE, no default backup schedule will be created for new databases within the instance.
        /// Possible values are: `NONE`, `AUTOMATIC`.
        #[builder(into, default)]
        pub default_backup_schedule_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The descriptive name for this instance as it appears in UIs. Must be
        /// unique per project and between 4 and 30 characters in length.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The edition selected for this instance. Different editions provide different capabilities at different price points.
        /// Possible values are: `EDITION_UNSPECIFIED`, `STANDARD`, `ENTERPRISE`, `ENTERPRISE_PLUS`.
        #[builder(into, default)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When deleting a spanner instance, this boolean option will delete all backups of this instance.
        /// This must be set to true if you created a backup manually in the console.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the instance, which cannot be changed after
        /// the instance is created. The name must be between 6 and 30 characters
        /// in length.
        /// If not provided, a random string starting with `tf-` will be selected.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub num_nodes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub processing_units: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The autoscaling configuration. Autoscaling is enabled if this field is set.
        /// When autoscaling is enabled, num_nodes and processing_units are treated as,
        /// OUTPUT_ONLY fields and reflect the current compute capacity allocated to
        /// the instance.
        /// Structure is documented below.
        pub autoscaling_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::InstanceAutoscalingConfig>,
        >,
        /// The name of the instance's configuration (similar but not
        /// quite the same as a region) which defines the geographic placement and
        /// replication of your databases in this instance. It determines where your data
        /// is stored. Values are typically of the form `regional-europe-west1` , `us-central` etc.
        /// In order to obtain a valid list please consult the
        /// [Configuration section of the docs](https://cloud.google.com/spanner/docs/instances).
        pub config: pulumi_gestalt_rust::Output<String>,
        /// Controls the default backup behavior for new databases within the instance.
        /// Note that `AUTOMATIC` is not permitted for free instances, as backups and backup schedules are not allowed for free instances.
        /// if unset or NONE, no default backup schedule will be created for new databases within the instance.
        /// Possible values are: `NONE`, `AUTOMATIC`.
        pub default_backup_schedule_type: pulumi_gestalt_rust::Output<String>,
        /// The descriptive name for this instance as it appears in UIs. Must be
        /// unique per project and between 4 and 30 characters in length.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The edition selected for this instance. Different editions provide different capabilities at different price points.
        /// Possible values are: `EDITION_UNSPECIFIED`, `STANDARD`, `ENTERPRISE`, `ENTERPRISE_PLUS`.
        pub edition: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// When deleting a spanner instance, this boolean option will delete all backups of this instance.
        /// This must be set to true if you created a backup manually in the console.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the instance, which cannot be changed after
        /// the instance is created. The name must be between 6 and 30 characters
        /// in length.
        /// If not provided, a random string starting with `tf-` will be selected.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub num_nodes: pulumi_gestalt_rust::Output<i32>,
        pub processing_units: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Instance status: `CREATING` or `READY`.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_config_binding = args.autoscaling_config.get_output(context);
        let config_binding = args.config.get_output(context);
        let default_backup_schedule_type_binding = args
            .default_backup_schedule_type
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let edition_binding = args.edition.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let num_nodes_binding = args.num_nodes.get_output(context);
        let processing_units_binding = args.processing_units.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:spanner/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingConfig".into(),
                    value: autoscaling_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultBackupScheduleType".into(),
                    value: default_backup_schedule_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numNodes".into(),
                    value: num_nodes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "processingUnits".into(),
                    value: processing_units_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            autoscaling_config: o.get_field("autoscalingConfig"),
            config: o.get_field("config"),
            default_backup_schedule_type: o.get_field("defaultBackupScheduleType"),
            display_name: o.get_field("displayName"),
            edition: o.get_field("edition"),
            effective_labels: o.get_field("effectiveLabels"),
            force_destroy: o.get_field("forceDestroy"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            num_nodes: o.get_field("numNodes"),
            processing_units: o.get_field("processingUnits"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
        }
    }
}
