pub struct PagesProjectArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    pub deployment_configs:
        pulumi_wasm_rust::Output<Option<crate::types::PagesProjectDeploymentConfigs>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub production_branch: pulumi_wasm_rust::Output<String>,
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
}

pub struct PagesProjectResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub deployment_configs: pulumi_wasm_rust::Output<crate::types::PagesProjectDeploymentConfigs>,
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub production_branch: pulumi_wasm_rust::Output<String>,
    pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: PagesProjectArgs) -> PagesProjectResult {
    let result = crate::bindings::pulumi::cloudflare::pages_project::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::pages_project::Args {
            account_id: args.account_id.get_inner(),
            build_config: args.build_config.get_inner(),
            deployment_configs: args.deployment_configs.get_inner(),
            name: args.name.get_inner(),
            production_branch: args.production_branch.get_inner(),
            source: args.source.get_inner(),
        },
    );

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
