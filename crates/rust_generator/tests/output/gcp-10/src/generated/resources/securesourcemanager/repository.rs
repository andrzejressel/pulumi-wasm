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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Description of the repository, which cannot exceed 500 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Initial configurations for the repository.
        /// Structure is documented below.
        #[builder(into, default)]
        pub initial_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securesourcemanager::RepositoryInitialConfig>,
        >,
        /// The name of the instance in which the repository is hosted.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the Repository.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Time the repository was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the repository, which cannot exceed 500 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Initial configurations for the repository.
        /// Structure is documented below.
        pub initial_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::securesourcemanager::RepositoryInitialConfig>,
        >,
        /// The name of the instance in which the repository is hosted.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The location for the Repository.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the Repository.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the repository.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the repository was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// URIs for the repository.
        /// Structure is documented below.
        pub uris: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securesourcemanager::RepositoryUri>,
        >,
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
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let initial_config_binding = args.initial_config.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let repository_id_binding = args.repository_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialConfig".into(),
                    value: &initial_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: &instance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            initial_config: o.get_field("initialConfig"),
            instance: o.get_field("instance"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            repository_id: o.get_field("repositoryId"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            uris: o.get_field("uris"),
        }
    }
}
