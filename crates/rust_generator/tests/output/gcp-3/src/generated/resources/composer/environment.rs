///
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/environments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default projects/{{project}}/locations/{{region}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Configuration parameters for this environment.
        #[builder(into, default)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::composer::EnvironmentConfig>,
        >,
        /// User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map
        /// are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and
        /// must conform to the following regular expression: a-z?. Label values must be between 0 and 63 characters long and must
        /// conform to the regular expression (a-z?)?. No more than 64 labels can be associated with a given environment. Both keys
        /// and values must be <= 128 bytes in size. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the environment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location or Compute Engine region for the environment.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration options for storage used by Composer environment.
        #[builder(into, default)]
        pub storage_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::composer::EnvironmentStorageConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Configuration parameters for this environment.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::composer::EnvironmentConfig,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map
        /// are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and
        /// must conform to the following regular expression: a-z?. Label values must be between 0 and 63 characters long and must
        /// conform to the regular expression (a-z?)?. No more than 64 labels can be associated with a given environment. Both keys
        /// and values must be <= 128 bytes in size. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the environment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The location or Compute Engine region for the environment.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Configuration options for storage used by Composer environment.
        pub storage_config: pulumi_gestalt_rust::Output<
            super::super::types::composer::EnvironmentStorageConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_binding = args.config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let storage_config_binding = args.storage_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:composer/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: config_binding.get_id(),
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
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfig".into(),
                    value: storage_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            config: o.get_field("config"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            storage_config: o.get_field("storageConfig"),
        }
    }
}
