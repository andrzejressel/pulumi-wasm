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
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// If set to true, skip repository creation if a repository with the same name already exists.
        #[builder(into, default)]
        pub create_ignore_already_exists: pulumi_wasm_rust::Output<Option<bool>>,
        /// Resource name of the repository, of the form `{{repo}}`.
        /// The repo name may contain slashes. eg, `name/with/slash`
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// How this repository publishes a change in the repository through Cloud Pub/Sub.
        /// Keyed by the topic names.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pubsub_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sourcerepo::RepositoryPubsubConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// If set to true, skip repository creation if a repository with the same name already exists.
        pub create_ignore_already_exists: pulumi_wasm_rust::Output<Option<bool>>,
        /// Resource name of the repository, of the form `{{repo}}`.
        /// The repo name may contain slashes. eg, `name/with/slash`
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// How this repository publishes a change in the repository through Cloud Pub/Sub.
        /// Keyed by the topic names.
        /// Structure is documented below.
        pub pubsub_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::sourcerepo::RepositoryPubsubConfig>>,
        >,
        /// The disk usage of the repo, in bytes.
        pub size: pulumi_wasm_rust::Output<i32>,
        /// URL to clone the repository from Google Cloud Source Repositories.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RepositoryArgs) -> RepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let create_ignore_already_exists_binding = args
            .create_ignore_already_exists
            .get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let pubsub_configs_binding = args.pubsub_configs.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:sourcerepo/repository:Repository".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "createIgnoreAlreadyExists".into(),
                    value: &create_ignore_already_exists_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pubsubConfigs".into(),
                    value: &pubsub_configs_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createIgnoreAlreadyExists".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pubsubConfigs".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryResult {
            create_ignore_already_exists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createIgnoreAlreadyExists").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pubsub_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pubsubConfigs").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
