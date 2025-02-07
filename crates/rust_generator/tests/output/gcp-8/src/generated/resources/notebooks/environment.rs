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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let container_image_binding = args
            .container_image
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let post_startup_script_binding = args
            .post_startup_script
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let vm_image_binding = args.vm_image.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:notebooks/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerImage".into(),
                    value: &container_image_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "postStartupScript".into(),
                    value: &post_startup_script_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vmImage".into(),
                    value: &vm_image_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentResult {
            container_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerImage"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            post_startup_script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("postStartupScript"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            vm_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmImage"),
            ),
        }
    }
}
