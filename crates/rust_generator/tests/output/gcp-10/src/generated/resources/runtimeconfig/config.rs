/// ## Example Usage
///
/// Example creating a RuntimeConfig resource.
///
/// ```yaml
/// resources:
///   my-runtime-config:
///     type: gcp:runtimeconfig:Config
///     properties:
///       name: my-service-runtime-config
///       description: Runtime configuration values for my service
/// ```
///
/// ## Import
///
/// Runtime Configs can be imported using the `name` or full config name, e.g.
///
/// * `projects/{{project_id}}/configs/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Runtime Configs can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/config:Config default projects/{{project_id}}/configs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/config:Config default {{name}}
/// ```
///
/// When importing using only the name, the provider project must be set.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigArgs {
        /// The description to associate with the runtime
        /// config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the runtime config.
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigResult {
        /// The description to associate with the runtime
        /// config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the runtime config.
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigArgs,
    ) -> ConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:runtimeconfig/config:Config".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
