/// A possible configuration for a Cloud Spanner instance. Configurations
/// define the geographic placement of nodes and their replication.
///
///
/// To get more information about InstanceConfig, see:
///
/// * [API documentation](https://cloud.google.com/spanner/docs/reference/rest/v1/projects.instanceConfigs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/spanner/)
///
/// ## Example Usage
///
/// ## Import
///
/// InstanceConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instanceConfigs/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, InstanceConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:spanner/instanceConfig:InstanceConfig default projects/{{project}}/instanceConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/instanceConfig:InstanceConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/instanceConfig:InstanceConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceConfigArgs {
        /// Base configuration name, e.g. nam3, based on which this configuration is created. Only set for user managed
        /// configurations. baseConfig must refer to a configuration of type GOOGLE_MANAGED in the same project as this
        /// configuration.
        #[builder(into, default)]
        pub base_config: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of this instance configuration as it appears in UIs.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer
        /// to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the instance configuration. Values are of the
        /// form projects/<project>/instanceConfigs/[a-z][-a-z0-9]*
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The geographic placement of nodes in this instance configuration and their replication properties.
        /// Structure is documented below.
        #[builder(into)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::spanner::InstanceConfigReplica>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceConfigResult {
        /// Base configuration name, e.g. nam3, based on which this configuration is created. Only set for user managed
        /// configurations. baseConfig must refer to a configuration of type GOOGLE_MANAGED in the same project as this
        /// configuration.
        pub base_config: pulumi_gestalt_rust::Output<String>,
        /// Output only. Whether this instance config is a Google or User Managed Configuration.
        pub config_type: pulumi_gestalt_rust::Output<String>,
        /// The name of this instance configuration as it appears in UIs.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer
        /// to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the instance configuration. Values are of the
        /// form projects/<project>/instanceConfigs/[a-z][-a-z0-9]*
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The geographic placement of nodes in this instance configuration and their replication properties.
        /// Structure is documented below.
        pub replicas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::spanner::InstanceConfigReplica>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceConfigArgs,
    ) -> InstanceConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_config_binding = args.base_config.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let replicas_binding = args.replicas.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:spanner/instanceConfig:InstanceConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseConfig".into(),
                    value: &base_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicas".into(),
                    value: &replicas_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceConfigResult {
            base_config: o.get_field("baseConfig"),
            config_type: o.get_field("configType"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            replicas: o.get_field("replicas"),
        }
    }
}
