/// Provides a resource which manages Cloudflare Pages projects.
///
/// > If you are using a `source` block configuration, you must first have a
///    connected GitHub or GitLab account connected to Cloudflare. See the
///    [Getting Started with Pages] documentation on how to link your accounts.
///
/// ## Import
///
/// !> It is not possible to import a pages project with secret environment variables. If you have a secret environment variable, you must remove it from your project before importing it.
///
/// ```sh
/// $ pulumi import cloudflare:index/pagesProject:PagesProject example <account_id>/<project_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pages_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PagesProjectArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
        #[builder(into, default)]
        pub build_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectBuildConfig>,
        >,
        /// Configuration for deployments in a project.
        #[builder(into, default)]
        pub deployment_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectDeploymentConfigs>,
        >,
        /// Name of the project.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the branch that is used for the production environment.
        #[builder(into)]
        pub production_branch: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::PagesProjectSource>,
        >,
    }
    #[allow(dead_code)]
    pub struct PagesProjectResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
        pub build_config: pulumi_gestalt_rust::Output<
            Option<super::types::PagesProjectBuildConfig>,
        >,
        /// When the project was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Configuration for deployments in a project.
        pub deployment_configs: pulumi_gestalt_rust::Output<
            super::types::PagesProjectDeploymentConfigs,
        >,
        /// A list of associated custom domains for the project.
        pub domains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the branch that is used for the production environment.
        pub production_branch: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
        pub source: pulumi_gestalt_rust::Output<
            Option<super::types::PagesProjectSource>,
        >,
        /// The Cloudflare subdomain associated with the project.
        pub subdomain: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PagesProjectArgs,
    ) -> PagesProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let build_config_binding = args.build_config.get_output(context).get_inner();
        let deployment_configs_binding = args
            .deployment_configs
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let production_branch_binding = args
            .production_branch
            .get_output(context)
            .get_inner();
        let source_binding = args.source.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/pagesProject:PagesProject".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "buildConfig".into(),
                    value: &build_config_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentConfigs".into(),
                    value: &deployment_configs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "productionBranch".into(),
                    value: &production_branch_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PagesProjectResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            build_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buildConfig"),
            ),
            created_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdOn"),
            ),
            deployment_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentConfigs"),
            ),
            domains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domains"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            production_branch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productionBranch"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
            subdomain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subdomain"),
            ),
        }
    }
}
