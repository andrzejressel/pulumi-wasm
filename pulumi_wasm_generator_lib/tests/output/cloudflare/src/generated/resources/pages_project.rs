#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    #[builder(into, default)]
    pub build_config: pulumi_wasm_rust::Output<
        Option<super::types::PagesProjectBuildConfig>,
    >,
    /// Configuration for deployments in a project.
    #[builder(into, default)]
    pub deployment_configs: pulumi_wasm_rust::Output<
        Option<super::types::PagesProjectDeploymentConfigs>,
    >,
    /// Name of the project.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the branch that is used for the production environment.
    #[builder(into)]
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    #[builder(into, default)]
    pub source: pulumi_wasm_rust::Output<Option<super::types::PagesProjectSource>>,
}
pub struct PagesProjectResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project build process. Read more about the build configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/build-configuration).
    pub build_config: pulumi_wasm_rust::Output<
        Option<super::types::PagesProjectBuildConfig>,
    >,
    /// When the project was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Configuration for deployments in a project.
    pub deployment_configs: pulumi_wasm_rust::Output<
        super::types::PagesProjectDeploymentConfigs,
    >,
    /// A list of associated custom domains for the project.
    pub domains: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the project.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the branch that is used for the production environment.
    pub production_branch: pulumi_wasm_rust::Output<String>,
    /// Configuration for the project source. Read more about the source configuration in the [developer documentation](https://developers.cloudflare.com/pages/platform/branch-build-controls/).
    pub source: pulumi_wasm_rust::Output<Option<super::types::PagesProjectSource>>,
    /// The Cloudflare subdomain associated with the project.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PagesProjectArgs) -> PagesProjectResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let build_config_binding = args.build_config.get_inner();
    let deployment_configs_binding = args.deployment_configs.get_inner();
    let name_binding = args.name.get_inner();
    let production_branch_binding = args.production_branch.get_inner();
    let source_binding = args.source.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/pagesProject:PagesProject".into(),
        name: name.to_string(),
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
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "buildConfig".into() },
            register_interface::ResultField { name : "createdOn".into() },
            register_interface::ResultField { name : "deploymentConfigs".into() },
            register_interface::ResultField { name : "domains".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "productionBranch".into() },
            register_interface::ResultField { name : "source".into() },
            register_interface::ResultField { name : "subdomain".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    PagesProjectResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        build_config: into_domain(hashmap.remove("buildConfig").unwrap()),
        created_on: into_domain(hashmap.remove("createdOn").unwrap()),
        deployment_configs: into_domain(hashmap.remove("deploymentConfigs").unwrap()),
        domains: into_domain(hashmap.remove("domains").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        production_branch: into_domain(hashmap.remove("productionBranch").unwrap()),
        source: into_domain(hashmap.remove("source").unwrap()),
        subdomain: into_domain(hashmap.remove("subdomain").unwrap()),
    }
}
