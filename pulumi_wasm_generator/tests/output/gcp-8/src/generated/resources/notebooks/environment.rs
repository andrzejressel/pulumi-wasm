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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub container_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::EnvironmentContainerImage>,
        >,
        /// A brief description of this environment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Display name of this environment for the UI.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name specified for the Environment instance.
        /// Format: projects/{project_id}/locations/{location}/environments/{environmentId}
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Path to a Bash script that automatically runs after a notebook instance fully boots up.
        /// The path must be a URL or Cloud Storage path. Example: "gs://path-to-file/file-name"
        #[builder(into, default)]
        pub post_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vm_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::EnvironmentVmImage>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        pub container_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::EnvironmentContainerImage>,
        >,
        /// Instance creation time
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A brief description of this environment.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Display name of this environment for the UI.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name specified for the Environment instance.
        /// Format: projects/{project_id}/locations/{location}/environments/{environmentId}
        pub name: pulumi_wasm_rust::Output<String>,
        /// Path to a Bash script that automatically runs after a notebook instance fully boots up.
        /// The path must be a URL or Cloud Storage path. Example: "gs://path-to-file/file-name"
        pub post_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        pub vm_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::EnvironmentVmImage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_image_binding = args.container_image.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let post_startup_script_binding = args.post_startup_script.get_inner();
        let project_binding = args.project.get_inner();
        let vm_image_binding = args.vm_image.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:notebooks/environment:Environment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerImage".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "postStartupScript".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "vmImage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentResult {
            container_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerImage").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            post_startup_script: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postStartupScript").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            vm_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmImage").unwrap(),
            ),
        }
    }
}
