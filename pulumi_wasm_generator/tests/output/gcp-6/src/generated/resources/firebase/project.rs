/// A Google Cloud Firebase instance. This enables Firebase resources on a given Google Project.
/// Since a FirebaseProject is actually also a GCP Project, a FirebaseProject uses underlying GCP
/// identifiers (most importantly, the projectId) as its own for easy interop with GCP APIs.
/// Once Firebase has been added to a Google Project it cannot be removed.
///
/// To get more information about Project, see:
///
/// * [API documentation](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects)
/// * How-to Guides
///     * Official Documentation
///
/// > **Note:** This resource should usually be used with a provider configuration
/// with `user_project_override = true` unless you wish for your quota
/// project to be different from the Firebase project.
///
/// ## Example Usage
///
/// ### Firebase Project Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project
///       name: my-project
///       orgId: '123456789'
///       deletionPolicy: DELETE
///       labels:
///         firebase: enabled
///   defaultProject:
///     type: gcp:firebase:Project
///     name: default
///     properties:
///       project: ${default.projectId}
/// ```
///
/// ## Import
///
/// Project can be imported using any of these accepted formats:
///
/// * `projects/{{project}}`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, Project can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/project:Project default projects/{{project}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/project:Project default {{project}}
/// ```
///
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The GCP project display name
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The number of the Google Project that Firebase is enabled on.
        pub project_number: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProjectArgs) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "projectNumber".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            project_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectNumber").unwrap(),
            ),
        }
    }
}
