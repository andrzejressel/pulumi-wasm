/// Manages an App Service Source Control Slot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleLinuxWebApp = linux_web_app::create(
///         "exampleLinuxWebApp",
///         LinuxWebAppArgs::builder()
///             .location("${exampleServicePlan.location}")
///             .name("example-web-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleLinuxWebAppSlot = linux_web_app_slot::create(
///         "exampleLinuxWebAppSlot",
///         LinuxWebAppSlotArgs::builder()
///             .app_service_id("${exampleLinuxWebApp.id}")
///             .name("example-slot")
///             .site_config(LinuxWebAppSlotSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
///     let exampleSourceControlSlot = source_control_slot::create(
///         "exampleSourceControlSlot",
///         SourceControlSlotArgs::builder()
///             .branch("master")
///             .repo_url("https://github.com/Azure-Samples/python-docs-hello-world")
///             .slot_id("${exampleLinuxWebAppSlot.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// an App Service Source Control Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/sourceControlSlot:SourceControlSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/slots/slot1"
/// ```
///
pub mod source_control_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceControlSlotArgs {
        /// The URL for the repository. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub branch: pulumi_wasm_rust::Output<Option<String>>,
        /// A `github_action_configuration` block as detailed below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub github_action_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appservice::SourceControlSlotGithubActionConfiguration,
            >,
        >,
        /// The branch name to use for deployments. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub repo_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Deployment Rollback be enabled? Defaults to `false` Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rollback_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Linux or Windows Web App Slot. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Function App Slots are not supported at this time.
        #[builder(into)]
        pub slot_id: pulumi_wasm_rust::Output<String>,
        /// Should the Slot use local Git configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_local_git: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should code be deployed manually. Set to `true` to disable continuous integration, such as webhooks into online repos such as GitHub. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_manual_integration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The repository specified is Mercurial. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub use_mercurial: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SourceControlSlotResult {
        /// The URL for the repository. Changing this forces a new resource to be created.
        pub branch: pulumi_wasm_rust::Output<String>,
        /// A `github_action_configuration` block as detailed below. Changing this forces a new resource to be created.
        pub github_action_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appservice::SourceControlSlotGithubActionConfiguration,
            >,
        >,
        /// The branch name to use for deployments. Changing this forces a new resource to be created.
        pub repo_url: pulumi_wasm_rust::Output<String>,
        /// Should the Deployment Rollback be enabled? Defaults to `false` Changing this forces a new resource to be created.
        pub rollback_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SCM Type in use. This value is decoded by the service from the repository information supplied.
        pub scm_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Linux or Windows Web App Slot. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Function App Slots are not supported at this time.
        pub slot_id: pulumi_wasm_rust::Output<String>,
        /// Should the Slot use local Git configuration. Changing this forces a new resource to be created.
        pub use_local_git: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should code be deployed manually. Set to `true` to disable continuous integration, such as webhooks into online repos such as GitHub. Defaults to `false`. Changing this forces a new resource to be created.
        pub use_manual_integration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The repository specified is Mercurial. Defaults to `false`. Changing this forces a new resource to be created.
        pub use_mercurial: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates if the Slot uses a GitHub action for deployment. This value is decoded by the service from the repository information supplied.
        pub uses_github_action: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SourceControlSlotArgs) -> SourceControlSlotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let branch_binding = args.branch.get_inner();
        let github_action_configuration_binding = args
            .github_action_configuration
            .get_inner();
        let repo_url_binding = args.repo_url.get_inner();
        let rollback_enabled_binding = args.rollback_enabled.get_inner();
        let slot_id_binding = args.slot_id.get_inner();
        let use_local_git_binding = args.use_local_git.get_inner();
        let use_manual_integration_binding = args.use_manual_integration.get_inner();
        let use_mercurial_binding = args.use_mercurial.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/sourceControlSlot:SourceControlSlot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "branch".into(),
                    value: &branch_binding,
                },
                register_interface::ObjectField {
                    name: "githubActionConfiguration".into(),
                    value: &github_action_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "repoUrl".into(),
                    value: &repo_url_binding,
                },
                register_interface::ObjectField {
                    name: "rollbackEnabled".into(),
                    value: &rollback_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "slotId".into(),
                    value: &slot_id_binding,
                },
                register_interface::ObjectField {
                    name: "useLocalGit".into(),
                    value: &use_local_git_binding,
                },
                register_interface::ObjectField {
                    name: "useManualIntegration".into(),
                    value: &use_manual_integration_binding,
                },
                register_interface::ObjectField {
                    name: "useMercurial".into(),
                    value: &use_mercurial_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "branch".into(),
                },
                register_interface::ResultField {
                    name: "githubActionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "repoUrl".into(),
                },
                register_interface::ResultField {
                    name: "rollbackEnabled".into(),
                },
                register_interface::ResultField {
                    name: "scmType".into(),
                },
                register_interface::ResultField {
                    name: "slotId".into(),
                },
                register_interface::ResultField {
                    name: "useLocalGit".into(),
                },
                register_interface::ResultField {
                    name: "useManualIntegration".into(),
                },
                register_interface::ResultField {
                    name: "useMercurial".into(),
                },
                register_interface::ResultField {
                    name: "usesGithubAction".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SourceControlSlotResult {
            branch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("branch").unwrap(),
            ),
            github_action_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("githubActionConfiguration").unwrap(),
            ),
            repo_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repoUrl").unwrap(),
            ),
            rollback_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rollbackEnabled").unwrap(),
            ),
            scm_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scmType").unwrap(),
            ),
            slot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slotId").unwrap(),
            ),
            use_local_git: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useLocalGit").unwrap(),
            ),
            use_manual_integration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useManualIntegration").unwrap(),
            ),
            use_mercurial: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useMercurial").unwrap(),
            ),
            uses_github_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usesGithubAction").unwrap(),
            ),
        }
    }
}