/// A Cloud AI Platform Notebook environment.
///
///
/// To get more information about Environment, see:
///
/// * [API documentation](https://cloud.google.com/ai-platform/notebooks/docs/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/ai-platform-notebooks)
///
/// ## Example Usage
///
/// ### Notebook Environment Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let environment = environment::create(
///         "environment",
///         EnvironmentArgs::builder()
///             .container_image(
///                 EnvironmentContainerImage::builder()
///                     .repository("gcr.io/deeplearning-platform-release/base-cpu")
///                     .build_struct(),
///             )
///             .location("us-west1-a")
///             .name("notebooks-environment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/environments/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:notebooks/environment:Environment default projects/{{project}}/locations/{{location}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/environment:Environment default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/environment:Environment default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub container_image: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::notebooks::EnvironmentContainerImage>,
        >,
        /// A brief description of this environment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name of this environment for the UI.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name specified for the Environment instance.
        /// Format: projects/{project_id}/locations/{location}/environments/{environmentId}
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path to a Bash script that automatically runs after a notebook instance fully boots up.
        /// The path must be a URL or Cloud Storage path. Example: "gs://path-to-file/file-name"
        #[builder(into, default)]
        pub post_startup_script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vm_image: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::notebooks::EnvironmentVmImage>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        pub container_image: pulumi_gestalt_rust::Output<
            Option<super::super::types::notebooks::EnvironmentContainerImage>,
        >,
        /// Instance creation time
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A brief description of this environment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Display name of this environment for the UI.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name specified for the Environment instance.
        /// Format: projects/{project_id}/locations/{location}/environments/{environmentId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path to a Bash script that automatically runs after a notebook instance fully boots up.
        /// The path must be a URL or Cloud Storage path. Example: "gs://path-to-file/file-name"
        pub post_startup_script: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        pub vm_image: pulumi_gestalt_rust::Output<
            Option<super::super::types::notebooks::EnvironmentVmImage>,
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
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_image_binding = args.container_image.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let post_startup_script_binding = args.post_startup_script.get_output(context);
        let project_binding = args.project.get_output(context);
        let vm_image_binding = args.vm_image.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:notebooks/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerImage".into(),
                    value: &container_image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postStartupScript".into(),
                    value: &post_startup_script_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmImage".into(),
                    value: &vm_image_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            container_image: o.get_field("containerImage"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            post_startup_script: o.get_field("postStartupScript"),
            project: o.get_field("project"),
            vm_image: o.get_field("vmImage"),
        }
    }
}
