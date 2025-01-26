/// Repositories store source code. It supports all Git SCM client commands and has built-in pull requests and issue tracking. Both HTTPS and SSH authentication are supported.
///
///
/// To get more information about Repository, see:
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/secure-source-manager/docs/overview)
///
/// ## Example Usage
///
/// ### Secure Source Manager Repository Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = repository::create(
///         "default",
///         RepositoryArgs::builder()
///             .instance("${instance.name}")
///             .location("us-central1")
///             .repository_id("my-repository")
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Secure Source Manager Repository Initial Config
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = repository::create(
///         "default",
///         RepositoryArgs::builder()
///             .description("This is a test repository")
///             .initial_config(
///                 RepositoryInitialConfig::builder()
///                     .defaultBranch("main")
///                     .gitignores(vec!["python",])
///                     .license("mit")
///                     .readme("default")
///                     .build_struct(),
///             )
///             .instance("${instance.name}")
///             .location("us-central1")
///             .repository_id("my-repository")
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}`
///
/// * `{{project}}/{{location}}/{{repository_id}}`
///
/// * `{{location}}/{{repository_id}}`
///
/// * `{{repository_id}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repository:Repository default projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repository:Repository default {{project}}/{{location}}/{{repository_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repository:Repository default {{location}}/{{repository_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/repository:Repository default {{repository_id}}
/// ```
///
pub mod repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Description of the repository, which cannot exceed 500 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Initial configurations for the repository.
        /// Structure is documented below.
        #[builder(into, default)]
        pub initial_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securesourcemanager::RepositoryInitialConfig>,
        >,
        /// The name of the instance in which the repository is hosted.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location for the Repository.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub repository_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Time the repository was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the repository, which cannot exceed 500 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Initial configurations for the repository.
        /// Structure is documented below.
        pub initial_config: pulumi_wasm_rust::Output<
            Option<super::super::types::securesourcemanager::RepositoryInitialConfig>,
        >,
        /// The name of the instance in which the repository is hosted.
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The location for the Repository.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for the Repository.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        pub repository_id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the repository.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time the repository was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// URIs for the repository.
        /// Structure is documented below.
        pub uris: pulumi_wasm_rust::Output<
            Vec<super::super::types::securesourcemanager::RepositoryUri>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let initial_config_binding = args.initial_config.get_output(context).get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repository_id_binding = args.repository_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "initialConfig".into(),
                    value: &initial_config_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "initialConfig".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "repositoryId".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "uris".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            initial_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialConfig").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryId").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            uris: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uris").unwrap(),
            ),
        }
    }
}
