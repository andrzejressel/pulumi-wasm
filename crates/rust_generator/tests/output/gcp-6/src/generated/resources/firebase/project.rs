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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The GCP project display name
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The number of the Google Project that Firebase is enabled on.
        pub project_number: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            project_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectNumber"),
            ),
        }
    }
}
