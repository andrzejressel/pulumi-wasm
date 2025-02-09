/// A repository (or repo) is a Git repository storing versioned source content.
///
///
/// To get more information about Repository, see:
///
/// * [API documentation](https://cloud.google.com/source-repositories/docs/reference/rest/v1/projects.repos)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/source-repositories/)
///
/// ## Example Usage
///
/// ### Sourcerepo Repository Basic
///
///
/// ```yaml
/// resources:
///   my-repo:
///     type: gcp:sourcerepo:Repository
///     properties:
///       name: my/repository
/// ```
/// ### Sourcerepo Repository Full
///
///
/// ```yaml
/// resources:
///   testAccount:
///     type: gcp:serviceaccount:Account
///     name: test_account
///     properties:
///       accountId: my-account
///       displayName: Test Service Account
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: my-topic
///   my-repo:
///     type: gcp:sourcerepo:Repository
///     properties:
///       name: my-repository
///       pubsubConfigs:
///         - topic: ${topic.id}
///           messageFormat: JSON
///           serviceAccountEmail: ${testAccount.email}
/// ```
///
/// ## Import
///
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/repos/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:sourcerepo/repository:Repository default projects/{{project}}/repos/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sourcerepo/repository:Repository default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// If set to true, skip repository creation if a repository with the same name already exists.
        #[builder(into, default)]
        pub create_ignore_already_exists: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Resource name of the repository, of the form `{{repo}}`.
        /// The repo name may contain slashes. eg, `name/with/slash`
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How this repository publishes a change in the repository through Cloud Pub/Sub.
        /// Keyed by the topic names.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pubsub_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::sourcerepo::RepositoryPubsubConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// If set to true, skip repository creation if a repository with the same name already exists.
        pub create_ignore_already_exists: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Resource name of the repository, of the form `{{repo}}`.
        /// The repo name may contain slashes. eg, `name/with/slash`
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// How this repository publishes a change in the repository through Cloud Pub/Sub.
        /// Keyed by the topic names.
        /// Structure is documented below.
        pub pubsub_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::sourcerepo::RepositoryPubsubConfig>>,
        >,
        /// The disk usage of the repo, in bytes.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// URL to clone the repository from Google Cloud Source Repositories.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let create_ignore_already_exists_binding = args
            .create_ignore_already_exists
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let pubsub_configs_binding = args.pubsub_configs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:sourcerepo/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createIgnoreAlreadyExists".into(),
                    value: create_ignore_already_exists_binding.get_id(),
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
                    name: "pubsubConfigs".into(),
                    value: pubsub_configs_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryResult {
            create_ignore_already_exists: o.get_field("createIgnoreAlreadyExists"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pubsub_configs: o.get_field("pubsubConfigs"),
            size: o.get_field("size"),
            url: o.get_field("url"),
        }
    }
}
