//! Provides a resource which manages Cloudflare Pages projects.
//! 
//! > If you are using a `source` block configuration, you must first have a
//!    connected GitHub or GitLab account connected to Cloudflare. See the
//!    [Getting Started with Pages] documentation on how to link your accounts.
//! 
//! ## Import
//! 
//! !> It is not possible to import a pages project with secret environment variables. If you have a secret environment variable, you must remove it from your project before importing it.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pagesProject:PagesProject example <account_id>/<project_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    /// Configuration for deployments in a project.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub deployment_configs: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectDeploymentConfigs>>,
    /// Name of the project.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the branch that is used for the production environment.
    #[builder(into)]
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
}

pub struct PagesProjectResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    /// When the project was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Configuration for deployments in a project.
    pub deployment_configs: pulumi_wasm_rust::Output<crate::types::PagesProjectDeploymentConfigs>,
    /// A list of associated custom domains for the project.
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the project.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the branch that is used for the production environment.
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
    /// The Cloudflare subdomain associated with the project.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PagesProjectArgs) -> PagesProjectResult {

    let result = crate::bindings::pulumi::cloudflare::pages_project::invoke(name, &crate::bindings::pulumi::cloudflare::pages_project::Args {
        account_id: &args.account_id.get_inner(),
        build_config: &args.build_config.get_inner(),
        deployment_configs: &args.deployment_configs.get_inner(),
        name: &args.name.get_inner(),
        production_branch: &args.production_branch.get_inner(),
        source: &args.source.get_inner(),
    });

    PagesProjectResult {
        account_id: crate::into_domain(result.account_id),
        build_config: crate::into_domain(result.build_config),
        created_on: crate::into_domain(result.created_on),
        deployment_configs: crate::into_domain(result.deployment_configs),
        domains: crate::into_domain(result.domains),
        name: crate::into_domain(result.name),
        production_branch: crate::into_domain(result.production_branch),
        source: crate::into_domain(result.source),
        subdomain: crate::into_domain(result.subdomain),
    }
}
